use std::collections::{HashMap, LinkedList};

use crate::compiler::CompilerState;
use crate::source::{Source, PathBuf, Location, HasLoc};
use crate::parser::error::ParseError;
use crate::ast;
use crate::parser::lexer;


pub struct Parser<'a> {
    state: &'a mut CompilerState,
    parses: HashMap<&'a Source, &'a ast::File<'a>>,
    errors: Vec<ParseError<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(state: &'a mut CompilerState) -> Parser<'a> {
        return Parser { state , parses: HashMap::new(), errors: Vec::new() };
    }

    pub fn parse_program(&'a mut self, start: PathBuf) {
        let mut to_visit: LinkedList<PathBuf> = LinkedList::from([start]);

        while !to_visit.is_empty() {
            let next = to_visit.pop_front().unwrap();
            if self.state.sources.iter().any(|s| s.path.as_ref().map_or(false, |p| p == &next)) {
                continue
            }

            self.state.sources.push(match Source::from_file(next.as_path()) {
                Some(s) => s,
                None => continue
            });
            let source = self.state.sources.last_mut().unwrap();

            let tokens = match lexer::lex_source(source) {
                Some(p) => p,
                None => continue
            };
        }
    }
}