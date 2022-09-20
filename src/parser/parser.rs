use std::collections::LinkedList;
use std::rc::Rc;

use crate::compiler::CompilerState;
use crate::source::{Source, PathBuf};
use crate::parser::error::ParseError;
use crate::error::ErrorSet;
use crate::ast;
use crate::parser::lexer;
use crate::parser::lexer::{Token, TokenType};


pub fn parse_program(state: &mut CompilerState, start: PathBuf) -> Result<(), ErrorSet<ParseError>> {
    let mut to_visit: LinkedList<PathBuf> = LinkedList::from([start]);

    let mut errors: ErrorSet<ParseError> = ErrorSet::new();

    while !to_visit.is_empty() {
        let next = to_visit.pop_front().unwrap();
        if state.sources.iter().any(|s| s.path.as_ref().map_or(false, |p| p == &next)) {
            continue
        }

        state.sources.push(match Source::from_file(next.as_path()) {
            Some(s) => Rc::from(s),
            None => continue
        });
        let source = state.sources.last().unwrap();

        let tokens = match lexer::lex_source(Rc::clone(source)) {
            Ok(t) => t,
            Err(mut e) => {
                errors.append(&mut e);
                continue
            }
        };

        let mut parser = Parser::new(tokens);
        parser.parse();
        errors.append(&mut parser.errors);
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(())
    }
}


struct Parser {
    errors: Vec<ParseError>,
    handlers: Vec<(Vec<TokenType>, SyncFlag)>,
    tokens: Vec<Token>,
    curr_index: usize
}

type SyncFlag = i64;
type ParseResult<T> = Result<T, SyncFlag>;


impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        return Parser { errors: Vec::new(), handlers: vec![(Vec::new(), 0)], tokens, curr_index: 0 };
    }

    fn is_done(&self) -> bool {
        return self.curr_index >= self.tokens.len();
    }

    fn curr(&self) -> Token {
        self.tokens[self.curr_index].clone()
    }

    fn expect(&self, expected: TokenType) -> bool {
        return self.curr().token_type == expected;
    }

    fn advance(&mut self) -> Token {
        let tok = &self.curr();
        self.curr_index += 1;
        return tok.clone();
    }

    fn catch<T, F>(&mut self, can_catch: &[TokenType], func: F) -> Result<(), SyncFlag>
        where F: FnOnce(&mut Self) -> Result<T, SyncFlag> {
        let this_flag = self.handlers.len() as SyncFlag;
        self.handlers.push((Vec::from(can_catch), this_flag));
        let result = match func(self) {
            Ok(_) => Ok(()),
            Err(flag) => if flag == this_flag { Ok(()) } else { Err(flag) }
        };
        self.handlers.pop();
        return result;
    }

    //noinspection RsBorrowChecker
    fn synchronize<T>(&mut self)  -> ParseResult<T> {
        while !self.is_done() {
            for (can_handle, flag) in &self.handlers {
                if can_handle.contains(&self.curr().token_type) {
                    return Err(*flag);
                }
            }
        }
        return Err(0 as SyncFlag);
    }

    fn consume(&mut self, expected: TokenType) -> ParseResult<Token> {
        return if self.expect(expected) {
            Ok(self.advance())
        } else {
            self.synchronize()
        }
    }

    fn parse(&mut self) -> Option<ast::File> {
        return  self.parse_file().ok();
    }

    fn parse_file(&mut self) -> ParseResult<ast::File> {
        let mut top_levels = Vec::new();
        while !self.is_done() {
            self.catch(&[TokenType::Struct], |s| {
                let top_level = s.parse_top_level()?;
                top_levels.push(top_level);
                Ok(())
            })?;
        }
        return Ok(ast::File { top_levels });
    }

    fn parse_top_level(&mut self) -> ParseResult<Box<ast::TopLevelNode>> {
        if self.expect(TokenType::Struct) {
            Ok(Box::from(ast::TopLevelNode::Struct(self.parse_struct()?)))
        } else {
            Err(0 as SyncFlag)
        }
    }

    fn parse_struct(&mut self) -> ParseResult<ast::StructNode> {
        let start = self.curr();
        self.consume(TokenType::Struct)?;
        let name = self.consume(TokenType::Identifier)?;

        let generic_parameters = Vec::new();
        let superstruct = None;
        let interfaces = Vec::new();
        let loc = self.curr().loc_range(&start);

        return Ok(ast::StructNode { loc, name: String::from(&name.text), generic_parameters, superstruct, interfaces })
    }
}