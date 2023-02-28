mod interpreter;
use interpreter::{Parser, Interpreter};

fn main() {
    let program = Parser::from_file("test/add.bf".to_owned())
        .unwrap()
        .optimized_parse();
    let mut interpreter = Interpreter::new(program);
    interpreter.run().unwrap();
}
