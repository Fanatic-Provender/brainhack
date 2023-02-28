use std::io::{stdin, stdout, Read, Write};

pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to Resume Program").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

pub fn cell_to_bin(mut cell: u8) -> [bool; 8] {
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
