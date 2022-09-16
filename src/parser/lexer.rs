use crate::source::{Source, Location, HasLoc};
use crate::parser::error;


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
    Plus, Minus,
    Star, Slash, Percent,
    Equal, Tilde,
    Ampersand, VerticalBar,
    Exclamation, Question,
    Period, Comma, Semicolon, Colon
}

pub struct Token<'a> {
    pub loc: Location<'a>,
    pub token_type: TokenType,
    pub text: String,
    pub leading_ws: bool
}

impl<'a> Token<'a> {
    pub fn new(text: &str, token_type: TokenType, loc: Location<'a>, leading_ws: bool) -> Token<'a> {
        return Token { loc, token_type, text: String::from(text), leading_ws}
    }
}


impl<'a> HasLoc<'a> for Token<'a> {
    fn get_loc(&self) -> Location<'a> {
        return self.loc;
    }
}


pub fn lex_source(source: &mut Source) -> Option<Vec<Token>> {
    use TokenType::*;

    let mut tokens = Vec::new();

    let characters: Vec<char> = source.text.chars().collect();

    let mut index = 0;
    let mut prev_is_ws = false;
    let mut line = 0;
    let mut line_start = 0;
    while index < characters.len() {
        let chr = characters[index];
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
            }
            let loc = Location::new(source, line, line_start - start, index - start);
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
            }
            let loc = Location::new(source, line, line_start - start, index - start);
            let text = &source.text[start..index];
            let token = Token::new(text, Integer, loc, prev_is_ws);
            tokens.push(token);
            prev_is_ws = false;
        } else {
            let token_type = match chr {
                '<' => LeftAngle,
                '>' => RightAngle,
                '(' => LeftParenthesis,
                ')' => RightParenthesis,
                '[' => LeftBracket,
                ']' => RightBracket,
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
                _ => {
                    panic!("unexpected character");
                }
            };
            index += 1;
            prev_is_ws = false;
        }
    }
    return Some(tokens);
}