use std::io::{self, BufRead};

const INPUT: &'static str = include_str!("../inputs/day2.txt");

// todos: comparing `char` with `str` is difficult
// todos: `str` cannot be indexed by `usize

fn main() -> io::Result<()> {
    let mut valid_count = 0;
    for line in io::Cursor::new(INPUT).lines() {
        let line = line?;
        let mut line = line.split(' ');
        let range = line.next().unwrap();
        let mut range = range.split('-');
        let n1: usize = range.next().unwrap().parse().unwrap();
        let n2: usize = range.next().unwrap().parse().unwrap();
        let pat: char = line.next().unwrap().trim_end_matches(':').parse().unwrap();
        let src = line.next().unwrap();

        // Part 1
        // let count = src.chars().filter(|b| *b == pat).count();
        // if count >= min && count <= max {
        //     valid_count += 1;
        // }

        // Part 2
        let lhs = match src.chars().skip(n1 - 1).next() {
            Some(c) => c == pat,
            None => false,
        };
        let rhs = match src.chars().skip(n2 - 1).next() {
            Some(c) => c == pat,
            None => false,
        };
        if lhs ^ rhs {
            valid_count += 1;
        }
    }
    println!("{}", valid_count);
    Ok(())
}
