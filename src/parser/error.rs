use crate::source::{Location, PathBuf};
use crate::error::{CompilerError, ErrorDisplay};
use crate::parser::lexer::TokenType;


pub enum ParseError {
    FileNotFound(PathBuf, Location),
    UnexpectedCharacter(char, Location),
    UnexpectedToken { expected: TokenType, got: TokenType, loc: Location },
    WithMessage(String, Location),
}


impl CompilerError for ParseError {
    fn render(&self, display: &mut ErrorDisplay) -> String {
        use ParseError::*;
        return match self {
            FileNotFound(file, loc) => {
                display.error_with_location("Error", format!("Could not read from file '{}'.", file.display()).as_str(), loc)
            },
            UnexpectedCharacter(chr, loc) => {
                display.error_with_location("Error", format!("Unexpected character '{}'.", chr).as_str(), loc)
            },
            UnexpectedToken { expected, got, loc } => {
                display.error_with_location("Error", format!("Unexpected token: Got {}, expected {}.", got, expected).as_str(), loc)
            },
            WithMessage(msg, loc) => {
                display.error_with_location("Error", msg, loc)
            }
        }
    }
}