use crate::source::Source;

pub struct CompilerState {
    pub sources: Vec<Source>
}

impl CompilerState {
    pub fn new() -> CompilerState {
        CompilerState { sources: Vec::new() }
    }
}