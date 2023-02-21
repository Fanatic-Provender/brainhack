mod brainfuck;
mod instruction;
mod tape;

use anyhow;
use sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::collections::VecDeque;
use std::time::Duration;


fn cell_to_bin(mut cell: u8) -> [bool; 8] {
    let mut bits = [false; 8];

    for i in 0..8 {
        let power = 2u8.pow(7 - (i as u32));
        if cell >= power {
            bits[i] = true;
            cell -= power;
        }
    }
    return bits
}

fn bin_to_word(bin1: [bool; 8], bin2: [bool; 8]) -> (u8, u8) {
    (
    bin1.into_iter()
        .enumerate()
        .filter(|(i, bit)| *bit)
        .fold(0, |acc, (i, bit)| 2u8.pow(i as u32)),

    bin1.into_iter()
        .enumerate()
        .filter(|(i, bit)| *bit)
        .fold(0, |acc, (i, bit)| 2u8.pow(i as u32))
    )
}

#[cfg(test)]
mod UtilsTest {
    use super::*;

    #[test]
    fn test_cell_to_bin() {
        assert_eq!(cell_to_bin(0), [false; 8]);
        assert_eq!(cell_to_bin(1), [false, false, false, false, false, false, false, true]);
        assert_eq!(cell_to_bin(2), [false, false, false, false, false, false, true, false]);
        assert_eq!(cell_to_bin(3), [false, false, false, false, false, false, true, true]);
        assert_eq!(cell_to_bin(4), [false, false, false, false, false, true, false, false]);
        assert_eq!(cell_to_bin(5), [false, false, false, false, false, true, false, true]);
        assert_eq!(cell_to_bin(6), [false, false, false, false, false, true, true, false]);
        assert_eq!(cell_to_bin(7), [false, false, false, false, false, true, true, true]);
        assert_eq!(cell_to_bin(8), [false, false, false, false, true, false, false, false]);
        assert_eq!(cell_to_bin(9), [false, false, false, false, true, false, false, true]);
        assert_eq!(cell_to_bin(10), [false, false, false, false, true, false, true, false]);
        assert_eq!(cell_to_bin(11), [false, false, false, false, true, false, true, true]);
        assert_eq!(cell_to_bin(12), [false, false, false, false, true, true, false, false]);
        assert_eq!(cell_to_bin(13), [false, false, false, false, true, true, false, true]);
        assert_eq!(cell_to_bin(14), [false, false, false, false, true, true, true, false]);
        assert_eq!(cell_to_bin(15), [false, false, false, false, true, true, true, true]);
        assert_eq!(cell_to_bin(16), [false, false, false, true, false, false, false, false]);
        assert_eq!(cell_to_bin(17), [false, false, false, true, false, false, false, true]);
    }

    #[test]
    fn test_bin_to_cell() {
        assert!(bin_to_word([false; 8], [false; 8]) == (0, 0));
        assert!(bin_to_word([false, false, false, false, false, false, false, true], [false; 8]) == (1, 0));
        assert!(bin_to_word([false, false, false, false, false, false, true, false], [false; 8]) == (2, 0));
        assert!(bin_to_word([false, false, false, false, false, false, false, true], [false, false, false, false, false, false, false, true]) == (1, 1));
        assert!(bin_to_word([false, false, false, false, false, false, true, false], [false, false, false, false, false, false, true, false]) == (2, 2));
        assert!(bin_to_word([false, false, false, false, false, false, true, true], [false, false, false, false, false, false, true, true]) == (3, 3));
        assert!(bin_to_word([false, false, false, false, false, true, false, false], [false, false, false, false, false, true, false, false]) == (4, 4));
        assert!(bin_to_word([false, false, false, false, false, true, false, true], [false, false, false, false, false, true, false, true]) == (5, 5));
        assert!(bin_to_word([false, false, false, false, false, true, true, false], [false, false, false, false, false, true, true, false]) == (6, 6));
        assert!(bin_to_word([false, false, false, false, false, true, true, true], [false, false, false, false, false, true, true, true]) == (7, 7));
        assert!(bin_to_word([false, false, false, false, true, false, false, false], [false, false, false, false, true, false, false, false]) == (8, 8));
    }
}