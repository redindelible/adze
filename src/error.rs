use std::fmt::{Display, Formatter};
use crate::source::Location;

pub trait CompilerError {
    fn render(&self, display: &mut ErrorDisplay) -> String;
}

pub struct ErrorDisplay {
    indent: usize
}

impl ErrorDisplay {
    pub fn new() -> ErrorDisplay {
        ErrorDisplay { indent: 0 }
    }

    pub fn with_indent<F>(&mut self, func: F)
        where F: FnOnce() -> () {
        self.indent += 1;
        func();
        self.indent -= 1;
    }

    pub fn error_with_location(&self, level: &str, message: &str, loc: &Location) -> String {
        let indent = "  | ".repeat(self.indent);
        let mut msg = format!("{indent}{level}: {message}\n");
        msg.push_str(&format!("     |> In {}\n", loc.source.name));
        msg.push_str(&format!("{: >4} | {}", loc.line+1, loc.source.get_line(loc.line)));
        if loc.multiline {
            msg.push_str(&format!("       {}{}>\n", " ".repeat(loc.offset), "^".repeat(loc.length)));
        } else {
            msg.push_str(&format!("       {}{}\n", " ".repeat(loc.offset), "^".repeat(loc.length)));
        }
        return msg;
    }
}


pub struct ErrorSet<E> where E: CompilerError {
    errors: Vec<E>
}

impl<E> ErrorSet<E> where E: CompilerError {
    pub fn new() -> ErrorSet<E>{
        ErrorSet {
            errors: Vec::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.errors.is_empty();
    }

    pub fn add_errors(&mut self, errors: &mut ErrorSet<E>) {
        self.errors.append(&mut errors.errors);
    }

    pub fn add_error(&mut self, error: E) {
        self.errors.push(error);
    }
}

impl<E> Display for ErrorSet<E> where E: CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut display = ErrorDisplay::new();
        for error in &self.errors {
            write!(f, "{}\n", error.render(&mut display))?;
        };
        Ok(())
    }
}