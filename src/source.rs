extern crate dunce;

use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::ops::Range;
pub use std::path::{PathBuf, Path};
use std::rc::Rc;
use std::cmp::{min, max};


#[derive(Debug)]
pub struct Source {
    pub path: Option<PathBuf>,
    pub name: String,
    pub text: String,
    pub lines: Vec<Range<usize>>
}

impl PartialEq<Self> for Source {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name
    }
}


impl Source {
    pub fn from_file(path: &Path) -> Option<Source> {
        let abs_path = dunce::canonicalize(path).ok()?;

        let mut file = File::open(&abs_path).ok()?;
        let name = abs_path.display().to_string();
        let mut text = String::new();
        file.read_to_string(&mut text).ok()?;

        let mut lines = Vec::new();
        let mut line_start = 0;
        for (i, chr) in text.chars().enumerate() {
            if chr == '\n' {
                lines.push(line_start..i+1);
                line_start = i+1;
            }
        }
        lines.push(line_start..text.len());

        return Some(Source { path: Some(abs_path), name, text, lines });
    }

    pub fn get_line(&self, index: usize) -> String {
        self.text[self.lines[index].clone()].to_owned()
    }
}


#[derive(Clone, Debug)]
pub struct Location {
    pub source: Rc<Source>,
    pub line: usize,
    pub offset: usize,
    pub length: usize,
    pub multiline: bool
}


impl Location {
    pub fn new(source: Rc<Source>, line: usize, offset: usize, length: usize) -> Location {
        Location { source, line, offset, length, multiline: false }
    }

    pub fn new_multiline(source: Rc<Source>, line: usize, offset: usize, length: usize) -> Location {
        Location { source, line, offset, length, multiline: true}
    }

    pub fn new_eof(source: Rc<Source>) -> Location {
        Location { line: source.lines.len()-1, offset: source.lines.last().unwrap().end, source, length: 1, multiline: false }
    }

    pub fn combine(&self, other: &Location) -> Location {
        if self.source != other.source { panic!("Sources must be the same") }

        return Location {
            source: Rc::clone(&self.source),
            line: min(self.line, other.line),
            offset: if (self.line, self.offset) <= (other.line, other.offset) { self.offset } else { other.offset },
            length: max(self.offset + self.length, other.offset + self.length) - min(self.offset, other.offset),
            multiline: self.line != other.line
        }
    }
}

pub trait HasLoc {
    fn get_loc(&self) -> &Location;
}