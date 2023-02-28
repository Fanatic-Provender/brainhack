mod interpreter;
use interpreter::{Parser, Interpreter};

fn main() {
    let program = Parser::from_bytes(b"+++++[->++<]#")
        .unwrap()
        .optimized_parse();
    let mut interpreter = Interpreter::new(program);
    interpreter.eval().unwrap();
}
