use crate::source::{Location, HasLoc};


pub struct File {
    pub top_levels: Vec<Box<TopLevelNode>>
}

impl File {
    pub fn new(top_levels: Vec<Box<TopLevelNode>>) -> File {
        File { top_levels }
    }
}


pub enum TopLevelNode {
    Import(ImportNode),
    Struct(StructNode)
}

pub struct ImportNode {
    pub loc: Location,
}

pub struct StructNode {
    pub loc: Location,
    pub name: String,
    pub generic_parameters: Vec<Box<GenericParameter>>,
    pub superstruct: Option<Box<QualifiedNameNode>>,
    pub interfaces: Vec<Box<QualifiedNameNode>>
}

pub struct GenericParameter {
    pub loc: Location,
    pub name: String,
    pub bound: Option<Box<TypeNode>>
}

impl HasLoc for TopLevelNode {
    fn get_loc(& self) -> Location {
        match self {
            TopLevelNode::Import(n) => n.loc.clone(),
            TopLevelNode::Struct(n) => n.loc.clone(),
        }
    }
}


pub enum QualifiedNameNode {
    Name(NameNode),
    Namespace(NamespaceNode)
}

pub struct NameNode {
    pub loc: Location,
    pub name: String,
}

pub struct NamespaceNode {
    pub loc: Location,
    pub source: Box<QualifiedNameNode>,
    pub attr: String,
}


pub enum TypeNode {
    Name(NameTypeNode),
    Function(FunctionTypeNode),
}

pub struct NameTypeNode {
    pub loc: Location,
    pub name: Box<QualifiedNameNode>,
    pub generic_arguments: Option<Vec<Box<TypeNode>>>
}

pub struct FunctionTypeNode {
    pub loc: Location,
    pub arguments: Vec<Box<TypeNode>>,
    pub ret: Box<TypeNode>
}
