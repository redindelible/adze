use crate::source::{Location, PathBuf};
use crate::error::{CompilerError, ErrorDisplay};


pub enum ParseError {
    FileNotFound(PathBuf, Location),
    UnexpectedCharacter(char, Location),
}


impl CompilerError for ParseError {
    fn render(&self, display: &mut ErrorDisplay) -> String {
        match self {
            ParseError::FileNotFound(file, loc) => {
                return display.error_with_location("Error", format!("Could not read from file '{}'.", file.display()).as_str(), loc);
            },
            ParseError::UnexpectedCharacter(chr, loc) => {
                return display.error_with_location("Error", format!("Unexpected character '{}'.", chr).as_str(), loc);
            }
        }
    }
}