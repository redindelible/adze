mod source;
mod ast;
mod parser;
mod compiler;


use source::PathBuf;
use compiler::CompilerState;


fn main() {
    let mut state = CompilerState::new();
    parser::parse_program(&mut state, PathBuf::from("test.adze"));

    println!("Hello, world! {}", state.sources.get(0).unwrap().name);
}
