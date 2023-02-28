mod hackfuck;
use hackfuck::{Interpreter, Parser};

fn main() {
    let program = Parser::from_file("test/add.bf".to_owned())
        .unwrap()
        .optimized_parse(false);
    let mut interpreter = Interpreter::new(program).init_screen();
    interpreter.run().unwrap();
}
