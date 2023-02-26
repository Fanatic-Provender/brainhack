use std::io::{stdin, stdout, Read, Write};

pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to Resume Program").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn cell_to_bin(mut cell: u8) -> [bool; 8] {
    let mut bits = [false; 8];

    for i in 0..8 {
        let power = 2u8.pow(7 - (i as u32));
        if cell >= power {
            bits[i] = true;
            cell -= power;
        }
    }
    return bits;
}

fn bin_to_word(bin1: [bool; 8], bin2: [bool; 8]) -> (u8, u8) {
    let word1 = bin1
        .into_iter()
        .rev()
        .enumerate()
        .filter(|(i, bit)| *bit)
        .fold(0, |acc, (i, bit)| acc + 2u8.pow(i as u32));

    let word2 = bin2
        .into_iter()
        .rev()
        .enumerate()
        .filter(|(i, bit)| *bit)
        .fold(0, |acc, (i, bit)| acc + 2u8.pow(i as u32));

    (word1, word2)
}
