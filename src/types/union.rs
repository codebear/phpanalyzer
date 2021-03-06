use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::{OsStr, OsString},
    fmt::Display,
    iter::FromIterator,
    os::unix::prelude::OsStrExt,
    sync::Arc,
};

use nom::error::Error;
use tree_sitter::Range;

use crate::{
    analysis::state::AnalysisState,
    issue::{Issue, IssueEmitter},
    symboldata::{class::ClassName, SymbolData},
    symbols::{FullyQualifiedName, Name},
};

use super::parser::union_type;
use super::{
    parse_types::{ConcreteType, ParsedType, ShapeKey, TypeName, TypeStruct, UnionOfTypes},
    parser::union_type_with_colon,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpecialType {
    Static,
    Self_,
    ClassString(Option<FullyQualifiedName>),
}

impl Display for SpecialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Static => write!(f, "static"),
            Self::Self_ => write!(f, "self"),
            Self::ClassString(class) => {
                if let Some(c) = class {
                    write!(f, "class-string<{}>", c.to_string())
                } else {
                    write!(f, "class-string")
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ShapeTypeKey {
    String(Name),
    Int(i64),
}

impl Display for ShapeTypeKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeTypeKey::String(name) => write!(f, "{}", name),
            ShapeTypeKey::Int(int) => write!(f, "{}", int),
        }
    }
}

impl From<ShapeKey> for ShapeTypeKey {
    fn from(key: ShapeKey) -> Self {
        match key {
            ShapeKey::Num(n) => Self::Int(n),
            ShapeKey::String(s) => Self::String(s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShapeTypeValue {
    optional: bool,
    utype: UnionType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShapeType {
    pub map: BTreeMap<ShapeTypeKey, ShapeTypeValue>,
}

impl ShapeType {
    pub fn new() -> Self {
        let map = BTreeMap::new();
        Self { map }
    }

    fn ensure_valid(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter, range: &Range) {
        for vtype in self.map.values() {
            vtype.ensure_valid(state, emitter, range);
        }
        crate::missing!("Determine if we need to validate shape keys in some way?");
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DiscreteType {
    NULL,
    Void,
    Int,
    Float,
    Resource,
    String,
    Bool,
    Mixed,

    False,
    /// General common array, of unknown content
    Array,
    Object,
    Callable,
    /// *  .0 = List of types for each argument to the callable
    /// *  .1 = Return type of the callable
    TypedCallable(Vec<UnionType>, UnionType),

    // Types with special (contextual) meaning, like static or self
    Special(SpecialType),

    Vector(UnionType),
    HashMap(UnionType, UnionType),
    Shape(ShapeType),
    Unknown,

    /// *  0 = Local name
    /// *  1 = FqName
    Named(Name, FullyQualifiedName),

    Generic(Box<DiscreteType>, Vec<UnionType>),

    ClassType(FullyQualifiedName, Name),
    Template(Name),
}
/*
impl Ord for DiscreteType {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}*/

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnionType {
    pub types: BTreeSet<DiscreteType>,
}

impl Display for UnionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.types
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" -or- ")
        )
    }
}
/*
impl Ord for UnionType {
    fn cmp(&self, other: &Self) -> Ordering {

        for (a, b) in self.types.iter().zip(other.types.iter()) {
            match a.cmp(b) {
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => (),
                Ordering::Greater => return Ordering::Greater,
            }
        }
        self.types().len().cmp(&other.types.len())
    }
}

impl PartialOrd for UnionType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
*/

impl UnionType {
    pub fn new() -> Self {
        UnionType {
            types: BTreeSet::new(),
        }
    }
    pub fn push(&mut self, t: DiscreteType) {
        self.types.insert(t);
    }

    pub fn reduce(list: Vec<Self>) -> Self {
        let mut utype = Self::new();
        for utype_list in list {
            for disc_type in utype_list.types {
                utype.push(disc_type)
            }
        }
        utype
    }

    pub fn merge_into(&mut self, other: UnionType) {
        for t in other.types {
            self.types.insert(t);
        }
    }

    pub(crate) fn is_callable(&self) -> bool {
        for t in &self.types {
            match t {
                DiscreteType::Callable => (),
                _ => return false,
            }
        }
        self.types.len() > 0
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    ///
    /// Returns Some(<type>) if the union safely can coalesce into one single type
    /// if it is empty or has multiple types it will return None
    pub fn single_type(&self) -> Option<DiscreteType> {
        if self.types.len() == 1 {
            return self.types.iter().next().cloned();
        }
        None
    }

    pub fn from_parsed_type(
        parsed_type: UnionOfTypes,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        from_vec_parsed_type(parsed_type, state, Some(emitter))
    }

    pub fn parse_with_remainder(
        type_str: OsString,
        _range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> (Option<UnionType>, Option<OsString>) {
        let parse_result = union_type(true)(type_str.as_bytes());

        Self::handle_parse_result(type_str.clone(), parse_result, state, emitter)
    }

    fn handle_parse_result(
        type_str: OsString,
        parse_result: Result<(&[u8], Vec<ConcreteType>), nom::Err<Error<&[u8]>>>,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> (Option<UnionType>, Option<OsString>) {
        let (rest, parsed_type) = if let Some((rest, parsed_type)) = parse_result.ok() {
            (rest, parsed_type)
        } else {
            return (None, Some(type_str.clone()));
        };

        let remainder = if rest.len() > 0 {
            let rest_str: OsString = OsStr::from_bytes(rest).into();
            Some(rest_str)
        } else {
            None
        };
        let found_types =
            if let Some(utype) = from_vec_parsed_type(parsed_type.clone(), state, Some(emitter)) {
                Some(utype)
            } else {
                eprintln!(
                    "Parsing of type: {:?} failed, parsed into: {:?}",
                    type_str, parsed_type
                );
                None
            };

        (found_types, remainder)
    }

    pub fn parse_with_colon(
        type_str: OsString,
        range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let parse_result = union_type_with_colon(true)(type_str.as_bytes());
        let (utype, remainder) =
            Self::handle_parse_result(type_str.clone(), parse_result, state, emitter);
        Self::handle_remainder(utype, remainder, state, emitter, range)
    }

    pub fn parse_simple(type_str: OsString) -> Option<UnionType> {
        let range = fake_range(&type_str);
        let emitter = VoidEmitter::new();
        let mut state = AnalysisState::new();

        let (utype, remainder) =
            Self::parse_with_remainder(type_str.clone(), range, &mut state, &emitter);

        Self::handle_remainder(utype, remainder, &mut state, &emitter, range)

    }

    pub fn parse(
        type_str: OsString,
        range: Range,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        let (utype, remainder) =
            Self::parse_with_remainder(type_str.clone(), range, state, emitter);

        Self::handle_remainder(utype, remainder, state, emitter, range)
    }

    fn handle_remainder(
        utype: Option<UnionType>,
        remainder: Option<OsString>,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        range: Range,
    ) -> Option<UnionType> {
        if let Some(rest) = remainder {
            if rest.len() > 0 {
                for ch in rest.as_bytes() {
                    match ch {
                        b' ' | b'\t' => (),
                        _ => {
                            emitter.emit(Issue::PHPDocTypeError(
                                state.pos_from_range(range),
                                format!("Remainder from parsing: {:?}", rest),
                            ));
                            return None;
                        }
                    }
                }
            }
        }
        utype
    }

    pub fn to_markdown(&self) -> String {
        let str_types: Vec<_> = self.types.iter().map(|x| x.to_markdown()).collect();
        let buffer = str_types.join("|");

        // let buffer = str::replace(&buffer, "\\", "\\\\");
        let buffer = str::replace(&buffer, "|", "\\|");

        let x = format!("`{}`", buffer);
        eprintln!("DEBUG: markdown generated: {}", x);
        x
    }

    pub(crate) fn single_type_excluding_null(&self) -> Option<DiscreteType> {
        let mut types = BTreeSet::new();
        for t in &self.types {
            match t {
                DiscreteType::NULL => (),
                _ => {
                    types.insert(t.clone());
                }
            }
        }
        if types.len() == 1 {
            types.iter().next().cloned()
        } else {
            None
        }
    }

    pub(crate) fn is_nullable(&self) -> bool {
        for t in &self.types {
            match t {
                DiscreteType::NULL => return true,
                _ => (),
            }
        }
        return false;
    }

    pub(crate) fn ensure_valid(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        range: &Range,
    ) {
        for dtype in &self.types {
            dtype.ensure_valid(state, emitter, range);
        }
    }

    pub fn filter_types<P>(&self, predicate: P) -> UnionType
    where
        P: Sized + FnMut(&&DiscreteType) -> bool,
    {
        let types: Vec<DiscreteType> = self.types.iter().filter(predicate).cloned().collect();
        UnionType::from(types)
    }
}

impl DiscreteType {
    pub fn to_markdown(&self) -> String {
        self.to_string()
    }

    fn ensure_valid(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter, range: &Range) {
        match self {
            DiscreteType::NULL => (),
            DiscreteType::Void => (),
            DiscreteType::Int => (),
            DiscreteType::Float => (),
            DiscreteType::Resource => (),
            DiscreteType::String => (),
            DiscreteType::Mixed => (),
            DiscreteType::Bool => (),
            DiscreteType::False => (),
            DiscreteType::Array => (),
            DiscreteType::Object => (),
            DiscreteType::Callable => (),
            DiscreteType::TypedCallable(a, b) => {
                for u in a {
                    u.ensure_valid(state, emitter, range);
                }
                b.ensure_valid(state, emitter, range);
            }
            DiscreteType::Special(s) => s.ensure_valid(state, emitter, range),
            DiscreteType::Vector(v) => v.ensure_valid(state, emitter, range),
            DiscreteType::HashMap(k, v) => {
                // FIXME k needs to be constrained to string or int, but where is that validated?
                // Should we have a separate type/enum for hash-key?
                k.ensure_valid(state, emitter, range);
                v.ensure_valid(state, emitter, range);
            }
            DiscreteType::Shape(s) => s.ensure_valid(state, emitter, range),
            DiscreteType::Unknown => (),
            DiscreteType::Named(_, fqname) => {
                if let Some(_cdata_handle) = state.symbol_data.get_class(&fqname.into()) {
                    // alles ok?
                } else {
                    emitter.emit(Issue::UnknownClass(
                        state.pos_from_range(range.clone()),
                        fqname.clone(),
                    ))
                }
            }
            DiscreteType::ClassType(_, _) => todo!(),
            DiscreteType::Template(t) => emitter.emit(Issue::EmptyTemplate(
                state.pos_from_range(range.clone()),
                t.clone(),
            )),
            _a @ DiscreteType::Generic(dtype, _utypes) => {
                dtype.ensure_valid(state, emitter, range);
                match &**dtype {
                    DiscreteType::Named(_, fqname) => {
                        if let Some(_cdata_handle) = state.symbol_data.get_class(&fqname.into()) {
                            // alles ok?
                            crate::missing!("Validate that generic arguments are as expected");
                        } else {
                            emitter.emit(Issue::UnknownClass(
                                state.pos_from_range(range.clone()),
                                fqname.clone(),
                            ))
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn can_evaluate_to_true(&self) -> bool {
        match self {
            DiscreteType::NULL => false,
            DiscreteType::Void => false,
            DiscreteType::Int => true,
            DiscreteType::Float => true,
            DiscreteType::Resource => true,
            DiscreteType::String => true,
            DiscreteType::Bool => true,
            DiscreteType::Mixed => true,
            DiscreteType::False => false,
            DiscreteType::Array => true,
            DiscreteType::Object => true,
            DiscreteType::Callable => true,
            DiscreteType::TypedCallable(_, _) => true,
            DiscreteType::Special(_) => true,
            DiscreteType::Vector(_) => true,
            DiscreteType::HashMap(_, _) => true,
            DiscreteType::Shape(_) => true,
            DiscreteType::Unknown => true,
            DiscreteType::Named(_, _) => true,
            DiscreteType::Generic(_, _) => true,
            DiscreteType::ClassType(_, _) => true,
            DiscreteType::Template(_) => true,
        }
    }

    pub fn can_evaluate_to_false(&self) -> bool {
        match self {
            DiscreteType::NULL => true,
            DiscreteType::Void => true,
            DiscreteType::Int => true,
            DiscreteType::Float => true,
            DiscreteType::Resource => false,
            DiscreteType::String => true,
            DiscreteType::Bool => true,
            DiscreteType::Mixed => true,
            DiscreteType::False => true,
            DiscreteType::Array => true,
            DiscreteType::Object => false,
            DiscreteType::Callable => false,
            DiscreteType::TypedCallable(_, _) => false,
            DiscreteType::Special(_) => false,
            DiscreteType::Vector(_) => true,
            DiscreteType::HashMap(_, _) => true,
            DiscreteType::Shape(_) => true,
            DiscreteType::Unknown => true,
            DiscreteType::Named(_, _) => false,
            DiscreteType::Generic(_, _) => {
                crate::missing!("ensure that a generic type never can evaluate to boolean false");
                false
            }
            DiscreteType::ClassType(_, _) => true,
            DiscreteType::Template(_) => true,
        }
    }

    ///
    /// When someting with this type is an argument to `<some> instanceof SomeThing`
    /// could this type evaluate to true?
    pub fn can_be_instance_of(
        &self,
        check_cname: FullyQualifiedName,
        symbol_data: &Arc<SymbolData>,
    ) -> bool {
        match self {
            DiscreteType::NULL => false,
            DiscreteType::Void => false,
            DiscreteType::Int => false,
            DiscreteType::Float => false,
            DiscreteType::Resource => false,
            DiscreteType::String => false,
            DiscreteType::Bool => false,
            DiscreteType::Mixed => true,
            DiscreteType::False => false,
            DiscreteType::Array => false,
            DiscreteType::Object => true,
            DiscreteType::Callable => true,
            DiscreteType::TypedCallable(_, _) => true,
            DiscreteType::Special(_) => {
                // Needs more thight hardening
                crate::missing!();
                true
            }
            DiscreteType::Vector(_) => false,
            DiscreteType::HashMap(_, _) => false,
            DiscreteType::Shape(_) => false,
            DiscreteType::Unknown => true,
            DiscreteType::Named(_, fq_named) => {
                let cname: ClassName = fq_named.into();
                let check_cname: ClassName = check_cname.into();
                if let Some(class_data) = symbol_data.get_class(&cname) {
                    return class_data
                        .read()
                        .unwrap()
                        .instanceof(&check_cname, symbol_data.clone());
                }
                return false;
            }
            DiscreteType::Generic(_, _) => {
                crate::missing!();
                true
            }
            DiscreteType::ClassType(_, _) => {
                crate::missing!();
                true
            }
            DiscreteType::Template(_) => {
                crate::missing!();
                true
            }
        }
    }
}

pub(crate) fn from_vec_parsed_type(
    ptypes: Vec<ConcreteType>,
    state: &mut AnalysisState,
    maybe_emitter: Option<&dyn IssueEmitter>,
) -> Option<UnionType> {
    let mut utype = UnionType::new();
    for ptype in ptypes {
        utype.merge_into(from_parsed_type(ptype, state, maybe_emitter)?);
    }
    Some(utype)
}

fn from_parsed_type(
    ctype: ConcreteType,
    state: &mut AnalysisState,
    maybe_emitter: Option<&dyn IssueEmitter>,
) -> Option<UnionType> {
    let utype = match ctype.ptype {
        ParsedType::Type(type_struct) => from_type_struct(type_struct, state, maybe_emitter),
        ParsedType::Shape(entries) => {
            let mut shape = ShapeType::new();
            let mut key_idx = 0;
            for entry in entries {
                let (key, optional) = if let Some((k, optional)) = entry.0 {
                    if let ShapeKey::Num(idx) = k {
                        if idx >= key_idx {
                            key_idx = idx + 1;
                        }
                    }
                    (k, optional)
                } else {
                    let k = ShapeKey::Num(key_idx);
                    key_idx += 1;
                    (k, false)
                };
                let utype = from_vec_parsed_type(entry.1, state, maybe_emitter)?;
                shape
                    .map
                    .insert(key.into(), ShapeTypeValue { optional, utype });
            }
            Some(DiscreteType::Shape(shape).into())
        }
        ParsedType::Callable(args, cond_return) => {
            let return_type = match cond_return {
                Some(rt) if rt.len() > 0 => match from_vec_parsed_type(rt, state, maybe_emitter) {
                    Some(t) => t,
                    None => {
                        crate::missing!("Failed to parse return type correctly");
                        DiscreteType::Unknown.into()
                    }
                },
                _ => DiscreteType::Void.into(),
            };
            let arg_vector: Vec<UnionType> = args
                .iter()
                .map(
                    |x| match from_vec_parsed_type(x.clone(), state, maybe_emitter) {
                        Some(utype) => utype,
                        None => {
                            crate::missing!("Failed to parse argument type correctly");
                            DiscreteType::Unknown.into()
                        }
                    },
                )
                .collect();
            Some(DiscreteType::TypedCallable(arg_vector, return_type).into())
        }
        ParsedType::CallableUntyped => {
            Some(DiscreteType::Callable.into())
            //             crate::missing_none!("callable type with not details must be cast to UnionType")
        }
        ParsedType::ClassType(cname, tname) => {
            let fq_name = match cname {
                TypeName::Name(symbol_name) => {
                    state.get_fq_symbol_name_from_local_name(&symbol_name)
                }
                TypeName::FQName(fq) => fq,
                TypeName::RelativeName(_r) => {
                    return crate::missing_none!(
                        "Missing support for types in relative class-names"
                    );
                }
            };
            Some(DiscreteType::ClassType(fq_name, tname).into())
        }
    };

    if let Some(mut utype) = utype {
        if ctype.nullable {
            utype.push(DiscreteType::NULL)
        }
        Some(utype)
    } else {
        None
    }
}

fn from_type_struct(
    type_struct: TypeStruct,
    state: &mut AnalysisState,
    maybe_emitter: Option<&dyn IssueEmitter>,
) -> Option<UnionType> {
    let dtype = if let TypeName::Name(tname) = &type_struct.type_name {
        let lc_type_str = tname.to_os_string().to_ascii_lowercase();
        // check for native types
        //    let type_str = lc_types.as_bytes();
        // FIXME ensure that any non-used generics are being reported
        match lc_type_str.as_bytes() {
            b"string" => Some(DiscreteType::String),
            b"int" => Some(DiscreteType::Int),
            b"integer" => Some(DiscreteType::Int),
            b"float" | b"double" => Some(DiscreteType::Float),
            b"boolean" | b"bool" => Some(DiscreteType::Bool),
            b"resource" => Some(DiscreteType::Resource),
            b"self" => Some(DiscreteType::Special(SpecialType::Self_)),
            b"static" => Some(DiscreteType::Special(SpecialType::Static)),
            b"mixed" => Some(DiscreteType::Mixed),
            b"void" => Some(DiscreteType::Void),
            b"null" => Some(DiscreteType::NULL),
            b"class-string" => {
                if let Some(gen) = &type_struct.generics {
                    // FIXME emit or othervise make sure that any problems here aren't overlooked
                    let noe = if gen.len() == 1 {
                        let x = &gen[0];
                        if x.len() == 1 {
                            let y = &x[0];
                            if y.nullable {
                                return None;
                            }
                            match &y.ptype {
                                ParsedType::Type(z) if z.generics.is_none() => z.type_name.clone(),
                                _ => return None,
                            }
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    };

                    let fqname = match noe {
                        TypeName::Name(name) => state.get_fq_symbol_name_from_local_name(&name),
                        TypeName::FQName(fq) => fq,
                        TypeName::RelativeName(_) => todo!(),
                    };
                    Some(DiscreteType::Special(SpecialType::ClassString(Some(
                        fqname,
                    ))))
                } else {
                    Some(DiscreteType::Special(SpecialType::ClassString(None)))
                }
            }
            b"object" => Some(DiscreteType::Object),
            b"array" => {
                if let Some(gen) = &type_struct.generics {
                    if gen.len() == 2 {
                        let key = from_vec_parsed_type(gen[0].clone(), state, maybe_emitter)?;
                        let value = from_vec_parsed_type(gen[1].clone(), state, maybe_emitter)?;

                        Some(DiscreteType::HashMap(key, value))
                    } else if gen.len() == 1 {
                        let value = from_vec_parsed_type(gen[0].clone(), state, maybe_emitter)?;
                        Some(DiscreteType::Vector(value))
                    } else {
                        // void
                        None
                    }
                } else {
                    Some(DiscreteType::Array)
                }
            }
            _ => None,
        }
    } else {
        None
    };

    if type_struct.generics.is_none() {
        if let TypeName::Name(x) = &type_struct.type_name {
            let data = state.get_generic_templates();
            if let Some(data) = data {
                if data.contains(&x) {
                    return Some(DiscreteType::Template(x.clone()).into());
                }
            }
        }
    }
    // ...

    let mut base_type = if let Some(DiscreteType::Special(SpecialType::Self_)) = dtype {
        match &state.in_class {
            Some(class_state) => {
                // void
                let dtype: DiscreteType = class_state.get_name().into();
                dtype
            }
            None => return crate::missing_none!("self outside of class"),
        }
    } else if let Some(dt) = dtype {
        dt
    } else {
        let cname = match &type_struct.type_name {
            TypeName::Name(name) => ClassName::new_with_analysis_state(name, state),
            TypeName::FQName(fq_name) => ClassName::new_with_fq_name(fq_name.clone()),
            TypeName::RelativeName(path) => {
                let mut fq_name = if let Some(ns) = &state.namespace {
                    ns.clone()
                } else {
                    FullyQualifiedName::new()
                };
                fq_name.append(path.clone());
                ClassName::new_with_fq_name(fq_name)
            }
        };
        cname.into()
    };
    match &base_type {
        DiscreteType::Vector(_) | DiscreteType::HashMap(_, _) => return Some(base_type.into()),
        _ => (),
    }

    if let Some(generic_args) = type_struct.generics {
        let mut generics: Vec<UnionType> = vec![];
        for gen_arg in generic_args {
            generics.push(from_vec_parsed_type(gen_arg, state, maybe_emitter)?);
        }

        base_type = DiscreteType::Generic(Box::new(base_type), generics);
    }

    let mut utype = UnionType::new();

    utype.push(base_type.into());
    Some(utype)
}

impl From<ClassName> for DiscreteType {
    fn from(cname: ClassName) -> Self {
        DiscreteType::Named(cname.get_name().clone(), cname.get_fq_name().clone())
    }
}

impl Display for DiscreteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DiscreteType::NULL => "null".into(),
                DiscreteType::Void => "void".into(),
                DiscreteType::Int => "int".to_string(),
                DiscreteType::Float => "double".to_string(),
                DiscreteType::Resource => "resource".to_string(),
                DiscreteType::String => "string".to_string(),
                DiscreteType::Bool => "bool".to_string(),
                DiscreteType::Array => "array".to_string(),
                DiscreteType::Callable => "callable".to_string(),
                DiscreteType::Mixed => "mixed".to_string(),
                DiscreteType::TypedCallable(arg_types, return_type) => format!(
                    "callable({}):{}",
                    arg_types
                        .iter()
                        .map(|x| format!("{}", x))
                        .collect::<Vec<String>>()
                        .join(", "),
                    return_type
                ),
                DiscreteType::Special(s) => s.to_string(),
                DiscreteType::Vector(t) => format!("array<{}>", t.to_string()),
                DiscreteType::HashMap(k, v) =>
                    format!("array<{},{}>", k.to_string(), v.to_string()),
                DiscreteType::Unknown => "*unknown*".to_string(),
                DiscreteType::Named(_, t) => t.to_string(),
                DiscreteType::False => "false".to_string(),
                DiscreteType::Object => "object".to_string(),

                DiscreteType::Template(t) => t.to_string(),

                DiscreteType::Shape(shape) => {
                    let mut buf = String::new();
                    buf.push_str("array{");
                    let mut parts = vec![];
                    for (key, value) in &shape.map {
                        parts.push(format!(
                            "{}{}:{}",
                            key,
                            if value.optional { "?" } else { "" },
                            value.utype
                        ));
                    }
                    buf.push_str(&parts.join(","));
                    buf.push_str("}");
                    buf
                }
                DiscreteType::Generic(base_type, v) => {
                    let indre: Vec<_> = v.iter().map(|x| x.to_string()).collect();

                    format!("{}<{}>", base_type, indre.join(", "))
                }
                DiscreteType::ClassType(fq_cname, tname) => {
                    format!("{}::{}", fq_cname, tname)
                }
            }
        )
    }
}

impl From<DiscreteType> for UnionType {
    fn from(discrete: DiscreteType) -> Self {
        let mut ut = UnionType::new();
        ut.push(discrete);
        ut
    }
}

impl From<&[&UnionType]> for UnionType {
    fn from(list: &[&UnionType]) -> Self {
        let mut utl = UnionType::new();
        for &ut in list {
            utl.merge_into(ut.clone());
        }
        utl
    }
}

impl From<&[DiscreteType]> for UnionType {
    fn from(list: &[DiscreteType]) -> Self {
        let mut ut = UnionType::new();
        for discrete in list {
            ut.push(discrete.clone());
        }
        ut
    }
}

impl From<Vec<DiscreteType>> for UnionType {
    fn from(list: Vec<DiscreteType>) -> Self {
        let mut ut = UnionType::new();
        for discrete in list {
            ut.push(discrete.clone());
        }
        ut
    }
}

impl From<Vec<UnionType>> for UnionType {
    fn from(list: Vec<UnionType>) -> Self {
        let mut ut = UnionType::new();
        for utype in list {
            ut.merge_into(utype.clone());
        }
        ut
    }
}

impl FromIterator<DiscreteType> for UnionType {
    fn from_iter<I: IntoIterator<Item = DiscreteType>>(list: I) -> Self {
        let mut ut = UnionType::new();
        for discrete in list {
            ut.push(discrete);
        }
        ut
    }
}

impl<'a> FromIterator<&'a DiscreteType> for UnionType {
    fn from_iter<I: IntoIterator<Item = &'a DiscreteType>>(list: I) -> Self {
        let mut ut = UnionType::new();
        for discrete in list {
            ut.push(discrete.clone());
        }
        ut
    }
}
impl SpecialType {
    fn ensure_valid(
        &self,
        _state: &mut AnalysisState,
        _emitter: &dyn IssueEmitter,
        _range: &Range,
    ) {
        crate::missing!("Ensure that self and static only are used in usable contexts");
    }
}
impl ShapeTypeValue {
    pub(crate) fn ensure_valid(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
        range: &Range,
    ) {
        self.utype.ensure_valid(state, emitter, range);
    }
}
