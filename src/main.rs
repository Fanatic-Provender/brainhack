mod interpreter;
use interpreter::{Parser, Interpreter};

fn main() {
    let program = Parser::from_file(&"src/add.bf".to_owned())
        .unwrap()
        .optimized_parse();
    let mut interpreter = Interpreter::new(program);
    interpreter.eval().unwrap();
}