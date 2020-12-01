use std::io;
use std::io::BufRead;

const INPUT: &'static str = include_str!("../inputs/day1.txt");

fn main() -> io::Result<()> {
    for line in io::Cursor::new(INPUT).lines() {
        dbg!(line?);
    }
    Ok(())
}
