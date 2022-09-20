mod source;
mod ast;
mod parser;
mod compiler;
mod error;

use source::PathBuf;
use compiler::CompilerState;


fn main() {
    let mut state = CompilerState::new();
    match parser::parse_program(&mut state, PathBuf::from("test.adze")) {
        Ok(a) => a,
        Err(errors) => {
            for error in &errors {

            }
        }
    }

    println!("Hello, world! {}", state.sources.get(0).unwrap().name);
}
