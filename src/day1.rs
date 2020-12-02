use std::io::{self, BufRead};

const INPUT: &'static str = include_str!("../inputs/day1.txt");

// todos: the stdlib is missing a `combinations` adapter; a "perfect" solution could use const generics.

fn main() -> io::Result<()> {
    'main: for (i, n1) in io::Cursor::new(INPUT).lines().enumerate() {
        let n1: u64 = n1?.parse().unwrap();
        for (j, n2) in io::Cursor::new(INPUT).lines().skip(i).enumerate() {
            let n2: u64 = n2?.parse().unwrap();
            for n3 in io::Cursor::new(INPUT).lines().skip(j) {
                let n3: u64 = n3?.parse().unwrap();
                if n1 + n2 + n3 == 2020 {
                    println!("{}", n1 * n2 * n3);
                    break 'main;
                }
            }
        }
    }
    Ok(())
}
