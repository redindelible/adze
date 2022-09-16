use crate::source::{Location, HasLoc};


pub struct File<'a> {
    top_levels: Vec<TopLevelNode<'a>>
}

impl<'a> File<'a> {
    pub fn new() -> File<'a> {
        File { top_levels: Vec::new() }
    }
}


pub enum TopLevelNode<'a> {
    Import(ImportNode<'a>)
}

impl<'a> HasLoc<'a> for TopLevelNode<'a> {
    fn get_loc(&'a self) -> Location<'a> {
        match self {
            TopLevelNode::Import(n) => n.loc
        }
    }
}


pub struct ImportNode<'a> {
    loc: Location<'a>
}
