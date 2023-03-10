use std::io::{stdin, stdout, Read, Write};

/// Pause the program until the user presses Enter.
pub fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to Resume Program").unwrap();
    stdout.flush().unwrap();
    stdin().read_exact(&mut [0]).unwrap();
}

/// Convert a byte to an array of 8 boolean values.
pub fn cell_to_bin(mut cell: u8) -> [bool; 8] {
    let mut bits = [false; 8];

    for (i, bit) in bits.iter_mut().enumerate() {
        let power = 2u8.pow(7 - (i as u32));
        if cell >= power {
            *bit = true;
            cell -= power;
        }
    }

    bits
}
