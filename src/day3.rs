//! https://adventofcode.com/2020/day/3

use std::io::{self, BufRead};

const INPUT: &'static str = include_str!("../inputs/day3.txt");

fn main() -> io::Result<()> {
    let mut tree_count = 1;
    let inputs = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for (x, y) in inputs {
        tree_count *= count(INPUT, *x, *y)?;
    }
    println!("{}", tree_count);
    Ok(())
}

fn count(input: &str, x_step: usize, y_step: usize) -> io::Result<usize> {
    let mut coord = 0;
    let mut count = 0;
    for line in io::Cursor::new(input).lines().skip(y_step).step_by(y_step) {
        let line = line?;
        coord += x_step;
        if line.chars().cycle().skip(coord).next().unwrap() == '#' {
            count += 1;
        }
    }
    Ok(count)
}
