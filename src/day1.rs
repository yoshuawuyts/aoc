use std::io;
use std::io::BufRead;

const INPUT: &'static str = include_str!("../inputs/day1.txt");

fn main() -> io::Result<()> {
    for (i, left) in io::Cursor::new(INPUT).lines().enumerate() {
        let left: u64 = left?.parse().unwrap();
        for right in io::Cursor::new(INPUT).lines().skip(i) {
            let right: u64 = right?.parse().unwrap();
            if left + right == 2020 {
                println!("{}", left * right);
            }
        }
    }
    Ok(())
}
