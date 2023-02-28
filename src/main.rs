mod interpreter;

fn main() { 
    let program = interpreter::Parser::from_bytes(b"++++[->++<]*").unwrap().optimized_parse();
    let mut interpreter = interpreter::Interpreter::new(program);
    interpreter.eval().unwrap();
}
