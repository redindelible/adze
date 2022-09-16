use std::fs::File;
use std::io::Read;
use std::ops::Range;
pub use std::path::{PathBuf, Path};


pub struct Source {
    pub path: Option<PathBuf>,
    pub name: String,
    pub text: String,
    pub lines: Vec<Range<usize>>
}

impl Source {
    pub fn from_file(path: &Path) -> Option<Source> {
        let abs_path = path.canonicalize().ok()?;

        let mut file = File::open(&abs_path).ok()?;
        let name = abs_path.to_string_lossy().into_owned();
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

    // pub fn from_text(name: &str, text: &str) -> Source {
    //     return Source { path: None, name: String::from(name), text: String::from(text), lines: vec![0..text.len() as u32] };
    // }
}

#[derive(Clone, Copy)]
pub struct Location<'a> {
    source: &'a Source,
    line: usize,
    offset: usize,
    length: usize
}


impl<'a> Location<'a> {
    pub fn new(source: &'a Source, line: usize, offset: usize, length: usize) -> Location<'a> {
        return Location { source, line, offset, length }
    }
}

pub trait HasLoc<'a> {
    fn get_loc(&'a self) -> Location<'a>;
}