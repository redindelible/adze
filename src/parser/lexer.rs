use std::rc::Rc;
use crate::source::{Source, Location, HasLoc};
use crate::parser::error::ParseError;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    Integer,
    Identifier,
    String,

    // Keywords
    Return,
    If,
    For,
    Is,
    While,
    Struct,
    Import,
    Fn,
    Trait,

    // Symbols
    LeftAngle, RightAngle,
    LeftParenthesis, RightParenthesis,
    LeftBracket, RightBracket,
    LeftBrace, RightBrace,
    Plus, Minus,
    Star, Slash, Percent,
    Equal, Tilde,
    Ampersand, VerticalBar,
    Exclamation, Question,
    Period, Comma, Semicolon, Colon,

    // Special
    Error
}

#[derive(Debug, Clone)]
pub struct Token {
    pub loc: Location,
    pub token_type: TokenType,
    pub text: String,
    pub leading_ws: bool
}

impl Token {
    pub fn new(text: &str, token_type: TokenType, loc: Location, leading_ws: bool) -> Token {
        return Token { loc, token_type, text: String::from(text), leading_ws}
    }

    pub fn loc_range(&self, other: &Token) -> Location {
        return self.loc.combine(&other.loc);
    }
}


impl HasLoc for Token {
    fn get_loc(&self) -> Location {
        return self.loc.clone();
    }
}


pub fn lex_source(source: Rc<Source>) -> Result<Vec<Token>, Vec<ParseError>> {
    use TokenType::*;

    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    let characters: Vec<char> = source.text.chars().collect();

    let mut index = 0;
    let mut prev_is_ws = false;
    let mut line = 0;
    let mut line_start = 0;
    while index < characters.len() {
        let mut chr = characters[index];
        if chr == '\n' {
            line_start = index+1;
            line += 1;
            index += 1;
            prev_is_ws = true;
        } else if chr.is_ascii_whitespace() {
            index += 1;
            prev_is_ws = true;
        } else if chr.is_ascii_alphabetic() || chr == '_' {
            let start = index;
            while index < characters.len() && (chr.is_ascii_alphanumeric() || chr == '_') {
                index += 1;
                chr = characters[index];
            }
            let loc = Location::new(Rc::clone(&source), line, start - line_start, index - start);
            let text = &source.text[start..index];
            let token_type = match text {
                "while"  => While,
                "if"     => If,
                "return" => Return,
                "trait"  => Trait,
                "fn"     => Fn,
                "for"    => For,
                "is"     => Is,
                "import" => Import,
                "struct" => Struct,
                _        => Identifier
            };
            let token = Token::new(text, token_type, loc, prev_is_ws);
            tokens.push(token);
            prev_is_ws = false;
        } else if chr.is_ascii_digit() {
            let start = index;
            while index < characters.len() && chr.is_numeric() {
                index += 1;
                chr = characters[index];
            }
            let loc = Location::new(Rc::clone(&source), line, start - line_start, index - start);
            let text = &source.text[start..index];
            let token = Token::new(text, Integer, loc, prev_is_ws);
            tokens.push(token);
            prev_is_ws = false;
        } else {
            let loc = Location::new(Rc::clone(&source), line, index - line_start, 1);
            let text = &source.text[index..index+1];
            let token_type = match chr {
                '<' => LeftAngle,
                '>' => RightAngle,
                '(' => LeftParenthesis,
                ')' => RightParenthesis,
                '[' => LeftBracket,
                ']' => RightBracket,
                '{' => LeftBrace,
                '}' => RightBrace,
                '+' => Plus,
                '-' => Minus,
                '*' => Star,
                '/' => Slash,
                '%' => Percent,
                '=' => Equal,
                '~' => Tilde,
                '&' => Ampersand,
                '|' => VerticalBar,
                '!' => Exclamation,
                '?' => Question,
                '.' => Period,
                ',' => Comma,
                ';' => Semicolon,
                ':' => Colon,
                c => {
                    errors.push(ParseError::UnexpectedCharacter(c, loc.clone()));
                    Error
                }
            };
            let token = Token::new(text, token_type, loc.clone(), prev_is_ws);
            tokens.push(token);
            index += 1;
            prev_is_ws = false;
        }
    }
    return if errors.is_empty() { Ok(tokens) } else { Err(errors) }
}