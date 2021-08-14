//! https://adventofcode.com/2020/day/5

use std::io::{self, BufRead};

const INPUT: &'static str = include_str!("../inputs/day5.txt");

fn main() -> io::Result<()> {
    let mut seats = [false; 128 * 8];
    for line in io::Cursor::new(INPUT).lines() {
        let line = line?;
        let (row, _) = line[0..7].chars().fold((0, 127u8), |(l, u), c| match c {
            'F' => (l, u - (u - l + 1) / 2),
            'B' => (l + (u - l + 1) / 2, u),
            c => panic!("invalid character: '{}'", c),
        });

        let (_, col) = line[7..10].chars().fold((0, 7u8), |(l, u), c| match c {
            'L' => (l, u - (u - l + 1) / 2),
            'R' => (l + (u - l + 1) / 2, u),
            c => panic!("invalid character: '{}'", c),
        });

        let id = (row as u64 * 8) + col as u64;
        seats[id as usize] = true;
    }

    let (head, _) = seats.iter().enumerate().find(|(_, s)| **s).unwrap();
    let (tail, _) = seats.iter().rev().enumerate().find(|(_, s)| **s).unwrap();
    let seats = &seats[head..(seats.len() - tail)];

    let (i, _) = seats.iter().enumerate().find(|(_, s)| !*s).unwrap();
    println!("free seat: {:?}", i + head);
    Ok(())
}
