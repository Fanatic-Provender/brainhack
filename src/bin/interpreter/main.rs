use brainhack::interpreter::{Interpreter, Parser};

fn main() {
    let program = Parser::from_file("src/examples/add.bf".to_owned())
        .unwrap()
        .optimized_parse();
    let mut interpreter = Interpreter::new(program);
    interpreter.eval().unwrap();
}
