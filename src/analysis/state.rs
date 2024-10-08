use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tree_sitter::Point;
//use tree_sitter::Range;
use crate::parser::Range;

use crate::autonodes::any::AnyNodeRef;
use crate::config::PHPAnalyzeConfig;
use crate::issue::IssuePosition;
use crate::symboldata::class::ClassName;
use crate::symboldata::class::ClassType;
use crate::symboldata::class::MethodData;
use crate::symboldata::FunctionData;
use crate::symboldata::SymbolData;
use crate::symbols::FullyQualifiedName;
use crate::symbols::Name;
use crate::types::union::PHPType;
use crate::value::PHPValue;
use std::collections::HashMap;

use super::scope::{Scope, ScopeStack};

#[derive(Debug)]
pub struct ConstantData {
    pub fq_name: FullyQualifiedName,
    pub values: HashMap<(OsString, Range), (PHPType, Option<PHPValue>)>,
}

impl ConstantData {
    pub fn new(fq_name: FullyQualifiedName) -> Self {
        Self {
            fq_name,
            values: HashMap::new(),
        }
    }

    pub fn add_value(
        &mut self,
        filename: OsString,
        range: Range,
        constant_type: PHPType,
        val: Option<PHPValue>,
    ) {
        //
        let key = (filename, range);
        let value = (constant_type, val);
        self.values.insert(key, value);
    }

    ///
    /// Returns a value if there is only one known definition of the constant
    pub fn get_value(&self) -> Option<PHPValue> {
        if self.values.len() != 1 {
            return None;
        }
        let (_, (_, val)) = self.values.iter().next()?;
        val.clone()
    }
}

#[derive(Debug)]
pub struct GlobalState {
    pub scope_stack: RwLock<ScopeStack>,
    pub constants: RwLock<HashMap<FullyQualifiedName, ConstantData>>,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalState {
    pub fn new() -> Self {
        GlobalState {
            scope_stack: RwLock::new(ScopeStack::new()),
            constants: RwLock::new(HashMap::new()),
        }
    }
}

#[derive(Debug)]
pub enum ClassState {
    Interface(ClassName, Arc<RwLock<ClassType>>),
    Class(ClassName, Arc<RwLock<ClassType>>),
    Trait(ClassName, Arc<RwLock<ClassType>>),
}

impl ClassState {
    pub fn get_name(&self) -> ClassName {
        match self {
            Self::Class(c, _) => c.clone(),
            Self::Interface(i, _) => i.clone(),
            Self::Trait(t, _) => t.clone(),
        }
    }

    pub(crate) fn get_data(&self) -> Arc<RwLock<ClassType>> {
        match self {
            Self::Class(_, d) => d.clone(),
            Self::Interface(_, d) => d.clone(),
            Self::Trait(_, d) => d.clone(),
        }
    }
}

#[derive(Debug)]

pub enum FunctionDataPointer {
    Method(Arc<RwLock<MethodData>>),
    Function(Arc<RwLock<FunctionData>>),
}

impl FunctionDataPointer {
    pub fn get_generic_templates(&self) -> Option<Vec<Name>> {
        match self {
            Self::Method(m) => {
                let mdata = m.read().unwrap();
                mdata.generic_templates.clone()
            }
            Self::Function(f) => {
                let fdata = f.read().unwrap();
                fdata.generic_templates.clone()
            }
        }
    }
}
#[derive(Debug)]
pub struct FunctionState {
    pub name: Option<Name>,
    pub is_method: bool,
    pub scope_stack: RwLock<ScopeStack>,
    pub returns: RwLock<Vec<(Option<PHPType>, Option<PHPValue>)>>,
    pub data: Option<FunctionDataPointer>,
}

impl FunctionState {
    pub fn new(name: Option<Name>, is_method: bool, data: Option<FunctionDataPointer>) -> Self {
        Self {
            scope_stack: RwLock::new(ScopeStack::new()),
            returns: RwLock::new(Vec::new()),
            name,
            is_method,
            data,
        }
    }

    pub fn add_return(&self, ret_type: Option<PHPType>, ret_value: Option<PHPValue>) {
        let mut rets = self.returns.write().expect("Noe");
        rets.push((ret_type, ret_value));
    }

    pub(crate) fn new_method(
        method_name: Name,
        method_data: Arc<RwLock<MethodData>>,
    ) -> FunctionState {
        Self::new(
            Some(method_name),
            true,
            Some(FunctionDataPointer::Method(method_data)),
        )
    }

    pub(crate) fn new_function(
        name: Name,
        function_data: Option<Arc<RwLock<FunctionData>>>,
    ) -> FunctionState {
        Self::new(
            Some(name),
            false,
            function_data.map(FunctionDataPointer::Function),
        )
    }

    pub(crate) fn new_anonymous() -> FunctionState {
        Self::new(None, false, None)
    }

    pub fn get_generic_templates(&self) -> Option<Vec<Name>> {
        self.data.as_ref()?.get_generic_templates()
    }
}

#[derive(Clone)]
pub struct LookingForNode {
    pub pos: Point,
    pub callback: Arc<
        RwLock<
            Option<Box<dyn FnOnce(AnyNodeRef, &mut AnalysisState, &[AnyNodeRef]) + Send + Sync>>,
        >,
    >,
}

impl std::fmt::Debug for LookingForNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LookingForNode")
            .field("pos", &self.pos)
            // .field("callback", &self.callback)
            .finish()
    }
    // void
}

#[derive(Debug)]
pub struct AnalysisState {
    pub pass: usize,
    pub filename: Option<PathBuf>,
    pub global: Arc<GlobalState>,
    pub in_class: Option<ClassState>,
    pub in_function_stack: Vec<FunctionState>,
    pub use_map: HashMap<Name, (Name, FullyQualifiedName)>,
    pub namespace: Option<FullyQualifiedName>,
    pub symbol_data: Arc<SymbolData>,
    pub last_doc_comment: Option<(OsString, Range)>,
    pub in_conditional_branch: bool,
    pub looking_for_node: Option<LookingForNode>,
    pub config: PHPAnalyzeConfig,
}

impl Default for AnalysisState {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisState {
    pub fn new() -> Self {
        Self::new_with_symbols(Arc::new(SymbolData::new()))
    }

    pub fn new_with_symbols(symbol_data: Arc<SymbolData>) -> Self {
        AnalysisState {
            pass: 0,
            filename: None,
            global: Arc::new(GlobalState::new()),
            in_class: None,
            in_function_stack: vec![],
            use_map: HashMap::new(),
            namespace: None,
            symbol_data,
            last_doc_comment: None,
            in_conditional_branch: false,
            looking_for_node: None,
            config: Default::default(),
        }
    }

    pub fn pos_as_string(&self, range: Range) -> String {
        let fname = if let Some(fname) = &self.filename {
            String::from_utf8_lossy(fname.as_os_str().as_bytes()).to_string()
        } else {
            String::from("*unknown*")
        };
        format!(
            "{}:{}:{}",
            fname,
            range.start_point.row + 1,
            range.start_point.column + 1,
        )
    }

    pub fn pos_from_range(&self, range: Range) -> IssuePosition {
        IssuePosition::new(&self.filename, range)
    }

    /// Appends namespace to local names. Does no lookup in use-map
    pub fn get_fq_symbol_name_without_aliasing(&self, symbol_name: &Name) -> FullyQualifiedName {
        let mut fq_name = if let Some(ns) = &self.namespace {
            ns.clone()
        } else {
            FullyQualifiedName::new()
        };
        fq_name.push(symbol_name);
        fq_name
    }

    //
    pub fn get_fq_symbol_name_from_local_name(&self, symbol_name: &Name) -> FullyQualifiedName {
        // eprintln!("AnalysisState.get_fq_symbol_name({:?}). use_map: {:?}", symbol_name, &self.use_map);
        let lc_symbol_name = symbol_name.to_ascii_lowercase();
        if let Some((_correct_cased_name, fq_name)) = self.use_map.get(&lc_symbol_name) {
            // eprintln!("USEMAP: {:?}", &self.use_map);
            //   eprintln!("fra use_map: {:?}", &fq_name);
            return fq_name.clone();
        };
        if let Some(ns) = &self.namespace {
            let mut fq = ns.clone();
            fq.push(symbol_name);
            fq
        } else {
            self.get_fq_symbol_name_without_aliasing(symbol_name)
        }
    }

    pub fn get_fq_function_name(&self, local_name: Name) -> FullyQualifiedName {
        self.get_fq_symbol_name_from_local_name(&local_name)
    }

    pub fn current_scope_stack(&self) -> &RwLock<ScopeStack> {
        if let Some(current_func) = self.in_function_stack.last() {
            &current_func.scope_stack
        } else {
            &self.global.scope_stack
        }
    }

    pub fn current_scope(&self) -> Arc<RwLock<Scope>> {
        self.current_scope_stack().read().unwrap().top()
    }

    pub(crate) fn push_scope(&self, scope: Arc<RwLock<Scope>>) {
        self.current_scope_stack().write().unwrap().push(scope)
    }

    pub(crate) fn pop_scope(&self) -> Arc<RwLock<Scope>> {
        self.current_scope_stack().write().unwrap().pop()
    }

    pub fn in_method<S>(&self, method_name: S) -> bool
    where
        S: AsRef<OsStr>,
    {
        let in_function = if let Some(in_function) = self.in_function_stack.last() {
            in_function
        } else {
            return false;
        };

        if !in_function.is_method {
            return false;
        }

        let name = if let Some(name) = &in_function.name {
            name
        } else {
            return false;
        };

        name.eq_ignore_ascii_case(method_name)
    }

    pub(crate) fn in_constructor(&self) -> bool {
        self.in_method("__construct")
    }

    ///
    /// In some parse-situations (especially parsing of method php-doc-block)
    /// We discover template-definitions AND usage of the same templates, before
    /// FunctionState is available in the state-object. Therefore a generic-map
    /// is optionally supplied
    pub(crate) fn get_generic_templates(
        &self,
        temp_generics: Option<&Vec<Name>>,
    ) -> Option<Vec<Name>> {
        let class_templates = self.in_class.as_ref().map(|x| x.get_data()).and_then(|x| {
            let read = x.read().unwrap();
            read.get_generic_templates()
        });
        let func_templates = self
            .in_function_stack
            .last()
            .and_then(|x| x.get_generic_templates());

        let func_templates = match (func_templates, temp_generics) {
            (Some(_), Some(_)) => todo!("Both generic-sources are populated..."),
            (None, Some(a)) => Some(a.clone()),
            (Some(a), None) => Some(a),
            _ => None,
        };

        match (class_templates, func_templates) {
            (Some(mut class_templates), Some(func_templates)) => {
                class_templates.extend(func_templates);
                Some(class_templates)
            }
            (Some(class_templates), None) => Some(class_templates),
            (None, Some(func_templates)) => Some(func_templates),
            (None, None) => None,
        }
    }
}

impl LookingForNode {
    pub fn found(&self, child: AnyNodeRef, state: &mut AnalysisState, path: &[AnyNodeRef]) {
        let mut handle = self.callback.write().unwrap();
        let cb = handle.take().expect("Already consumed the callback?");
        eprintln!(
            "FANT EN NODE: {:?}, path:len() = {}",
            child.kind(),
            path.len()
        );
        cb(child, state, path);
    }
}
