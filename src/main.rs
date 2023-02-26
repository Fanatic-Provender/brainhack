mod interpreter;
use interpreter::{pause, Interpreter, Parser};

fn main() {
    let program = b"+>++++*++>->>>>>-------->>++++<<>++++++++[-]>>+++++-->>>>>><<-";
    let mut interpreter = Interpreter::new(Parser::from_bytes(program).unwrap().parse());
    interpreter.run();
}
