use crate::source::{Location, HasLoc};


pub struct Program {
    pub files: Vec<File>
}

pub struct File {
    pub top_levels: Vec<Box<TopLevelNode>>
}

pub enum TopLevelNode {
    Import(ImportData),
    Struct(StructData),
    Function(FunctionData)
}

pub struct ImportData {
    pub loc: Location,
}

pub struct GenericParameter {
    pub loc: Location,
    pub name: String,
    pub bound: Option<Box<TypeNode>>
}

pub struct StructData {
    pub loc: Location,
    pub name: String,
    pub generic_parameters: Vec<Box<GenericParameter>>,
    pub superstruct: Option<Box<QualifiedNameNode>>,
    pub interfaces: Vec<Box<QualifiedNameNode>>,
    pub fields: Vec<Box<StructField>>
}

pub struct StructField {
    pub loc: Location,
    pub name: String,
    pub typ: Box<TypeNode>
}

pub struct FunctionData {
    pub loc: Location,
    pub name: String,
    pub generic_parameters: Vec<Box<GenericParameter>>,
    pub parameters: Vec<Box<FunctionParameter>>,
    pub ret: Box<TypeNode>,
    pub body: BlockData
}

pub struct FunctionParameter {
    pub loc: Location,
    pub name: String,
    pub typ: Box<TypeNode>
}


impl HasLoc for TopLevelNode {
    fn get_loc(& self) -> &Location {
        match self {
            TopLevelNode::Import(n) => &n.loc,
            TopLevelNode::Struct(n) => &n.loc,
            TopLevelNode::Function(n) => &n.loc
        }
    }
}

pub enum StmtNode {
    Expr(StmtExprData),
    Return(StmtReturnData)
}

pub struct StmtExprData {
    pub loc: Location,
    pub expr: Box<ExprNode>
}

pub struct StmtReturnData {
    pub loc: Location,
    pub expr: Box<ExprNode>
}


pub enum ExprNode {
    Name(NameData),
    Integer(IntegerData),
    Block(BlockData)
}

pub struct NameData {
    pub loc: Location,
    pub name: QualNameData
}

pub struct IntegerData {
    pub loc: Location,
    pub integer: u64
}

pub struct BlockData {
    pub loc: Location,
    pub stmts: Vec<Box<StmtNode>>
}


impl HasLoc for ExprNode {
    fn get_loc(&self) -> &Location {
        match self {
            ExprNode::Name(n) => &n.loc,
            ExprNode::Integer(n) => &n.loc,
            ExprNode::Block(n) => &n.loc
        }
    }
}

pub enum QualifiedNameNode {
    Name(QualNameData),
    Namespace(QualNamespaceData)
}

pub struct QualNameData {
    pub loc: Location,
    pub name: String,
}

pub struct QualNamespaceData {
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
    Name(TypeNameData),
    Function(TypeFunctionData),
    Reference(TypeReferenceData)
}

pub struct TypeNameData {
    pub loc: Location,
    pub name: Box<QualifiedNameNode>,
    pub generic_arguments: Option<Vec<Box<TypeNode>>>
}

pub struct TypeFunctionData {
    pub loc: Location,
    pub arguments: Vec<Box<TypeNode>>,
    pub ret: Box<TypeNode>
}

pub struct TypeReferenceData {
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