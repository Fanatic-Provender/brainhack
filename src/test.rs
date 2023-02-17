use {
    brainfuck::{program::Program, tape::ModArrayTape, Interpreter},
    std::io::Cursor,
};

#[track_caller]
pub fn compare_tape(
    code: &[u8],
    initial_tape: &[u8],
    initial_location: usize,
    final_tape: &[u8],
    final_location: usize,
) {
    let program: Vec<u8> =
        // read tape
        itertools::repeat_n([b',', b'>'].into_iter(), initial_tape.len())
        .flatten()
        .chain(itertools::repeat_n(b'<', initial_tape.len()))
        // execute program
        .chain(itertools::repeat_n(b'>', initial_location))
        .chain(code.iter().copied())
        // write tape
        .chain(itertools::repeat_n(b'<', final_location))
        .chain(itertools::repeat_n([b'.', b'>'].into_iter(), final_tape.len()).flatten())
        .collect();
    let program = String::from_utf8(program).expect("brainfuck code is not valid UTF-8");
    let program = Program::parse(&program).expect("invalid brainfuck program");

    let mut buffer = vec![];
    Interpreter::<ModArrayTape>::new(program, &mut Cursor::new(initial_tape), &mut buffer)
        .run()
        .expect("brainfuck runtime error");

    assert_eq!(buffer, final_tape);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_cell() {
        compare_tape(b">[-<+>]", &[27, 18], 0, &[45], 1);
    }
    #[test]
    fn multiply_cell() {
        compare_tape(b"[->[->+>+<<]>>[-<<+>>]<<<]", &[2, 71], 0, &[0, 71, 142], 0)
    }
}
