use crate::source::{Location, PathBuf};


pub enum ParseError<'a> {
    FileNotFound(PathBuf, Location<'a>)
}