use crate::source::{Location, PathBuf};


pub enum ParseError {
    FileNotFound(PathBuf, Location),
    UnexpectedCharacter(char, Location),
}