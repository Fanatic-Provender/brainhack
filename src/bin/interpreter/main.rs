mod cli;

use {
    brainhack::hackfuck::{Interpreter, Parser},
    clap::Parser as _,
    cli::Cli,
};

fn main() {
    let cli = Cli::parse();
    let source_path = &cli.file;

    let program = Parser::from_file(source_path)
        .unwrap()
        .optimized_parse(true);
    let mut interpreter = Interpreter::new(program);
    interpreter.eval().unwrap();
}
