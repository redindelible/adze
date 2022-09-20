use crate::source::Location;

pub trait CompilerError {
    fn render(&self, display: &mut ErrorDisplay) -> String;
}

pub struct ErrorDisplay {
    indent: usize
}

impl ErrorDisplay {
    pub fn with_indent<F>(&mut self, func: F)
        where F: FnOnce() -> () {
        self.indent += 1;
        func();
        self.indent -= 1;
    }

    pub fn error_with_location(&self, level: &str, message: &str, loc: &Location) -> String {
        let indent = "  | ".repeat(self.indent);
        return indent;
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
}