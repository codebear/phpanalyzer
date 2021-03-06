use std::collections::HashMap;

use crate::symbols::{FullyQualifiedName, Name};

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct TypeStruct {
    pub type_name: TypeName,
    pub generics: Option<Vec<Vec<ConcreteType>>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ShapeKey {
    String(Name),
    Num(i64),
}

#[derive(Clone, Debug)]
pub struct ShapeStruct {
    pub map: HashMap<ShapeKey, ConcreteType>,
}

impl ShapeStruct {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct ShapeEntry(pub Option<(ShapeKey, bool)>, pub Vec<ConcreteType>);

pub type UnionOfTypes = Vec<ConcreteType>;

pub type ArgumentVector = Vec<UnionOfTypes>;

pub type ReturnType = UnionOfTypes;

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum ParsedType {
    Type(TypeStruct),
    Shape(Vec<ShapeEntry>),
    Callable(ArgumentVector, Option<ReturnType>),
    ClassType(TypeName, Name),
    CallableUntyped,
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum TypeName {
    Name(Name),
    FQName(FullyQualifiedName),
    RelativeName(Vec<Name>),
}

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct ConcreteType {
    pub nullable: bool,
    pub ptype: ParsedType,
}
