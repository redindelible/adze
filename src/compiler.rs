use std::rc::Rc;
use crate::source::Source;

pub struct CompilerState {
    pub sources: Vec<Rc<Source>>
}

impl CompilerState {
    pub fn new() -> CompilerState {
        CompilerState { sources: Vec::new() }
    }
}