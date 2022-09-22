use std::collections::LinkedList;
use std::rc::Rc;

use crate::compiler::CompilerState;
use crate::source::{Source, PathBuf, HasLoc, Location};
use crate::parser::error::ParseError;
use crate::error::ErrorSet;
use crate::ast;
use crate::parser::lexer::{Token, TokenType, lex_source};


pub fn parse_program(state: &mut CompilerState, start: PathBuf) -> Result<ast::Program, ErrorSet<ParseError>> {
    // TODO better location handling for missing file
    let mut to_visit: LinkedList<PathBuf> = LinkedList::from([start]);

    let mut program = ast::Program { files: Vec::new() };
    let mut errors: ErrorSet<ParseError> = ErrorSet::new();

    while !to_visit.is_empty() {
        let next = to_visit.pop_front().unwrap();
        if state.sources.iter().any(|s| s.path.as_ref().map_or(false, |p| p == &next)) {
            continue
        }

        state.sources.push(match Source::from_file(next.as_path()) {
            Some(s) => Rc::from(s),
            None => {
                continue;
            }
        });
        let source = state.sources.last().unwrap();

        let tokens = match lex_source(Rc::clone(source)) {
            Ok(t) => t,
            Err(mut e) => {
                errors.add_errors(&mut e);
                continue
            }
        };

        match Parser::parse(Rc::clone(source), tokens) {
            Ok(f) => program.files.push(f),
            Err(mut e) => errors.add_errors(&mut e)
        }
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(program)
    }
}


struct Parser {
    errors: ErrorSet<ParseError>,
    handlers: Vec<(Vec<TokenType>, SyncFlag)>,
    tokens: Vec<Token>,
    source: Rc<Source>,
    curr_index: usize
}

type SyncFlag = i64;
type ParseResult<T> = Result<T, SyncFlag>;


impl Parser {
    fn new(source: Rc<Source>, tokens: Vec<Token>) -> Parser {
        return Parser { errors: ErrorSet::new(), handlers: vec![(Vec::new(), 0)], tokens, source, curr_index: 0 };
    }

    fn is_done(&self) -> bool {
        return self.curr_index >= self.tokens.len();
    }

    fn curr(&self) -> Token {
        if self.curr_index >= self.tokens.len() {
            Token::new("\0", TokenType::EOF, Location::new_eof(Rc::clone(&self.source)), false)
        } else {
            self.tokens[self.curr_index].clone()
        }
    }

    fn next(&self) -> Token {
        if self.curr_index + 1 >= self.tokens.len() {
            Token::new("\0", TokenType::EOF, Location::new_eof(Rc::clone(&self.source)), false)
        } else {
            self.tokens[self.curr_index + 1].clone()
        }
    }

    fn expect(&self, expected: TokenType) -> bool {
        return self.curr().token_type == expected;
    }

    fn expect_symbol(&self, first: TokenType, second: TokenType) -> bool {
        let next = self.next();
        return self.curr().token_type == first && next.token_type == second && !next.leading_ws;
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

    fn synchronize<T>(&mut self)  -> ParseResult<T> {
        while !self.is_done() {
            for (can_handle, flag) in &self.handlers {
                if can_handle.contains(&self.curr().token_type) {
                    return Err(*flag);
                }
            }
            self.advance();
        }
        return Err(0 as SyncFlag);
    }

    fn consume(&mut self, expected: TokenType) -> ParseResult<Token> {
        return if self.expect(expected) {
            Ok(self.advance())
        } else {
            self.errors.add_error(ParseError::UnexpectedToken {
                expected,
                got: self.curr().token_type,
                loc: self.curr().loc
            });
            self.synchronize()
        }
    }

    fn consume_symbol(&mut self, first: TokenType, second: TokenType, name: &str) -> ParseResult<(Token, Token)> {
        if self.expect_symbol(first, second) {
            Ok((self.advance(), self.advance()))
        } else {
            self.errors.add_error(ParseError::WithMessage(format!("Expected a {}.", name), self.curr().loc));
            self.synchronize()
        }
    }

    fn consume_error(&mut self, expected: TokenType, error_msg: &str) -> ParseResult<Token> {
        return if self.expect(expected) {
            Ok(self.advance())
        } else {
            self.errors.add_error(ParseError::WithMessage(String::from(error_msg), self.curr().loc));
            self.synchronize()
        }
    }

    fn parse(source: Rc<Source>, tokens: Vec<Token>) -> Result<ast::File, ErrorSet<ParseError>> {
        let mut parser = Parser { errors: ErrorSet::new(), handlers: vec![(Vec::new(), 0)], tokens, source, curr_index: 0 };
        match parser.parse_file() {
            Ok(f) => Ok(f),
            Err(_) => Err(parser.errors)
        }
    }

    fn parse_file(&mut self) -> ParseResult<ast::File> {
        let mut top_levels = Vec::new();
        while !self.is_done() {
            self.catch(&[TokenType::Struct, TokenType::Fn], |s| {
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
        } else if self.expect(TokenType::Fn) {
            Ok(Box::from(ast::TopLevelNode::Function(self.parse_function()?)))
        } else {
            self.errors.add_error(ParseError::WithMessage(String::from("Expected the start of a struct, function, or import."), self.curr().loc));
            self.synchronize()
        }
    }

    fn parse_struct(&mut self) -> ParseResult<ast::StructData> {
        let start = self.curr();
        self.consume_error(TokenType::Struct, "Struct definitions must begin with 'struct'")?;
        let name = self.consume(TokenType::Identifier)?;

        let mut generic_parameters = Vec::new();
        if self.expect(TokenType::LeftAngle) {
            self.consume(TokenType::LeftAngle)?;
            while !self.expect(TokenType::RightAngle) {
                generic_parameters.push(self.parse_generic_parameter()?);
                if !self.expect(TokenType::Comma) {
                    break;
                } else {
                    self.consume(TokenType::Comma)?;
                }
            }
            self.consume(TokenType::RightAngle)?;
        }

        let superstruct = if self.expect(TokenType::LeftParenthesis) {
            self.consume(TokenType::LeftParenthesis)?;
            Some(self.parse_qual_name()?)
        } else {
            None
        };
        let interfaces = Vec::new();

        let mut fields = Vec::new();
        self.consume(TokenType::LeftBrace)?;
        while !self.expect(TokenType::RightBrace) {
            fields.push(self.parse_struct_field()?);
        }
        self.consume(TokenType::RightBrace)?;

        let loc = self.curr().loc_range(&start);

        Ok(ast::StructData { loc, name: String::from(&name.text), generic_parameters, superstruct, interfaces, fields})
    }

    fn parse_generic_parameter(&mut self) -> ParseResult<Box<ast::GenericParameter>> {
        let name = self.consume(TokenType::Identifier)?;
        Ok(Box::from(ast::GenericParameter { loc: name.get_loc().clone(), name: name.text, bound: None }))
    }

    fn parse_struct_field(&mut self) -> ParseResult<Box<ast::StructField>> {
        let name = self.consume(TokenType::Identifier)?;
        self.consume(TokenType::Colon)?;
        let typ = self.parse_type()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Box::from(ast::StructField {
            loc: name.get_loc().combine(typ.get_loc()),
            name: name.text,
            typ
        }))
    }

    fn parse_function(&mut self) -> ParseResult<ast::FunctionData> {
        let start = self.consume(TokenType::Fn)?;
        let name = self.consume(TokenType::Identifier)?;

        let mut generic_parameters = Vec::new();

        if self.expect(TokenType::LeftAngle) {
            self.consume(TokenType::LeftAngle)?;
            while !self.expect(TokenType::RightAngle) {
                generic_parameters.push(self.parse_generic_parameter()?);
                if !self.expect(TokenType::Comma) {
                    break;
                } else {
                    self.consume(TokenType::Comma)?;
                }
            }
            self.consume(TokenType::RightAngle)?;
        }

        let mut parameters = Vec::new();
        self.consume(TokenType::LeftParenthesis)?;
        while !self.expect(TokenType::RightParenthesis) {
            parameters.push(self.parse_function_parameter()?);
            if !self.expect(TokenType::Comma) {
                break;
            } else {
                self.consume(TokenType::Comma)?;
            }
        }
        self.consume(TokenType::RightParenthesis)?;

        self.consume_symbol(TokenType::Minus, TokenType::RightAngle, "'->'")?;
        let ret = self.parse_type()?;

        let body = self.parse_block()?;

        Ok(ast::FunctionData {
            loc: start.get_loc().combine(&body.loc),
            name: name.text,
            generic_parameters,
            parameters,
            ret,
            body
        })
    }

    fn parse_function_parameter(&mut self) -> ParseResult<Box<ast::FunctionParameter>> {
        let name = self.consume(TokenType::Identifier)?;
        self.consume(TokenType::Colon)?;
        let typ = self.parse_type()?;
        return Ok(Box::from(ast::FunctionParameter { loc: name.get_loc().combine(typ.get_loc()), name: name.text, typ}))
    }

    fn parse_stmt(&mut self) -> ParseResult<Box<ast::StmtNode>> {
        if self.expect(TokenType::Return) {
            Ok(Box::from(ast::StmtNode::Return(self.parse_return()?)))
        } else {
            Ok(Box::from(ast::StmtNode::Expr(self.parse_expr_stmt()?)))
        }
    }

    fn parse_return(&mut self) -> ParseResult<ast::StmtReturnData> {
        let start = self.consume(TokenType::Return)?;
        let expr = self.parse_expr()?;
        let end = self.consume(TokenType::Semicolon)?;
        Ok(ast::StmtReturnData { loc: start.get_loc().combine(end.get_loc()), expr })
    }

    fn parse_expr_stmt(&mut self) -> ParseResult<ast::StmtExprData> {
        let expr = self.parse_expr()?;
        let end = self.consume(TokenType::Semicolon)?;
        Ok(ast::StmtExprData { loc: expr.get_loc().combine(end.get_loc()), expr })
    }

    fn parse_expr(&mut self) -> ParseResult<Box<ast::ExprNode>> {
        self.parse_expr_block()
    }

    fn parse_expr_block(&mut self) -> ParseResult<Box<ast::ExprNode>> {
        if self.expect(TokenType::LeftBrace) {
            Ok(Box::from(ast::ExprNode::Block(self.parse_block()?)))
        } else {
            self.parse_expr_terminal()
        }
    }

    fn parse_block(&mut self) -> ParseResult<ast::BlockData> {
        let start = self.consume(TokenType::LeftBrace)?;
        let mut stmts = Vec::new();
        while !self.expect(TokenType::RightBrace) {
            stmts.push(self.parse_stmt()?);
        }
        let end = self.consume(TokenType::RightBrace)?;
        Ok(ast::BlockData { loc: start.get_loc().combine(end.get_loc()), stmts })
    }

    fn parse_expr_terminal(&mut self) -> ParseResult<Box<ast::ExprNode>> {
        if self.expect(TokenType::Integer) {
            Ok(Box::from(ast::ExprNode::Integer(self.parse_integer()?)))
        } else if self.expect(TokenType::LeftParenthesis) {
            self.consume(TokenType::LeftParenthesis)?;
            let expr = self.parse_expr()?;
            self.consume(TokenType::RightParenthesis)?;
            Ok(expr)
        } else {
            self.errors.add_error(ParseError::WithMessage(String::from("Expected an expression."), self.curr().loc));
            self.synchronize()
        }
    }

    fn parse_integer(&mut self) -> ParseResult<ast::IntegerData> {
        let num_str = self.consume(TokenType::Integer)?;
        let num = match num_str.text.parse::<u64>() {
            Ok(n) => n,
            Err(_) => {
                self.errors.add_error(ParseError::CouldNotParseLiteral(TokenType::Integer, num_str.get_loc().clone()));
                self.synchronize()?
            }
        };
        Ok(ast::IntegerData {
            loc: num_str.get_loc().clone(),
            integer: num
        })
    }

    fn parse_qual_name(&mut self) -> ParseResult<Box<ast::QualifiedNameNode>> {
        let name = self.consume(TokenType::Identifier)?;
        let mut left = Box::from(ast::QualifiedNameNode::Name(ast::QualNameData { loc: name.get_loc().clone(), name: name.text }));
        loop {
            if self.expect_symbol(TokenType::Colon, TokenType::Colon) {
                self.advance(); self.advance();
                let attr_name = self.consume(TokenType::Identifier)?;
                left = Box::from(ast::QualifiedNameNode::Namespace(ast::QualNamespaceData {
                    loc: left.get_loc().combine(attr_name.get_loc()),
                    source: left,
                    attr: attr_name.text
                }))
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_type(&mut self) -> ParseResult<Box<ast::TypeNode>> {
        let typ = self.parse_type_terminal()?;
        if self.expect(TokenType::Ampersand) {
            let tok = self.consume(TokenType::Ampersand)?;
            Ok(Box::from(ast::TypeNode::Reference(ast::TypeReferenceData {
                loc: typ.get_loc().combine(tok.get_loc()),
                typ
            })))
        } else {
            Ok(typ)
        }
    }

    fn parse_type_terminal(&mut self) -> ParseResult<Box<ast::TypeNode>> {
        if self.expect(TokenType::LeftParenthesis) {
            Ok(Box::from(ast::TypeNode::Function(self.parse_function_type()?)))
        } else {
            Ok(Box::from(ast::TypeNode::Name(self.parse_name_type()?)))
        }
    }

    fn parse_name_type(&mut self) -> ParseResult<ast::TypeNameData> {
        let name = self.parse_qual_name()?;
        Ok(ast::TypeNameData {
            loc: name.get_loc().clone(),
            name,
            generic_arguments: None
        })
    }

    fn parse_function_type(&mut self) -> ParseResult<ast::TypeFunctionData> {
        let mut inputs = Vec::new();
        let start = self.consume(TokenType::LeftParenthesis)?;
        while !self.expect(TokenType::RightParenthesis) {
            inputs.push(self.parse_type()?);
            if !self.expect(TokenType::Comma) {
                break;
            } else {
                self.consume(TokenType::Comma)?;
            }
        }
        self.consume(TokenType::RightParenthesis)?;

        self.consume_symbol(TokenType::Minus, TokenType::RightAngle, "'->'")?;

        let output = self.parse_type()?;

        return Ok(ast::TypeFunctionData {
            loc : start.get_loc().combine(output.get_loc()),
            arguments: inputs,
            ret: output
        })
    }
}