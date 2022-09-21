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
    pub interfaces: Vec<Box<QualifiedNameNode>>,
    pub fields: Vec<Box<StructField>>
}

pub struct GenericParameter {
    pub loc: Location,
    pub name: String,
    pub bound: Option<Box<TypeNode>>
}

pub struct StructField {
    pub loc: Location,
    pub name: String,
    pub typ: Box<TypeNode>
}

impl HasLoc for TopLevelNode {
    fn get_loc(& self) -> &Location {
        match self {
            TopLevelNode::Import(n) => &n.loc,
            TopLevelNode::Struct(n) => &n.loc,
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

impl HasLoc for QualifiedNameNode {
    fn get_loc(&self) -> &Location {
        match self {
            QualifiedNameNode::Name(node) => &node.loc,
            QualifiedNameNode::Namespace(node) => &node.loc
        }
    }
}


pub enum TypeNode {
    Name(NameTypeNode),
    Function(FunctionTypeNode),
    Reference(ReferenceTypeNode)
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

pub struct ReferenceTypeNode {
    pub loc: Location,
    pub typ: Box<TypeNode>
}


impl HasLoc for TypeNode {
    fn get_loc(&self) -> &Location {
        match self {
            TypeNode::Name(n) => &n.loc,
            TypeNode::Function(n) => &n.loc,
            TypeNode::Reference(n) => &n.loc
        }
    }
}