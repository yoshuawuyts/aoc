use std::{collections::HashSet, num::ParseIntError, str::FromStr};

const INPUT: &'static str = include_str!("../inputs/3.txt");

pub fn run() {
    println!("day 3.1: {}", compute(parse(INPUT)));
}

fn compute(ops: (Vec<Op>, Vec<Op>)) -> i64 {
    let left = ops.0;
    let right = ops.1;

    let left = populate_positions(left);
    let right = populate_positions(right);
    let intersect = left.intersection(&right);

    let mut lowest = i64::MAX;
    for (mut l, mut r) in intersect.into_iter() {
        if l.is_negative() {
            l = -l;
        }
        if r.is_negative() {
            r = -r;
        }
        lowest = (l + r).min(lowest);
    }
    lowest
}

fn populate_positions(left: Vec<Op>) -> HashSet<(i64, i64)> {
    let mut cursor: (i64, i64) = (0, 0);
    let mut output = HashSet::new();
    for op in left {
        match op {
            Op::Up(n) => {
                for _ in 0..n {
                    cursor.1 += 1;
                    output.insert(cursor);
                }
            }
            Op::Right(n) => {
                for _ in 0..n {
                    cursor.0 += 1;
                    output.insert(cursor);
                }
            }
            Op::Down(n) => {
                for _ in 0..n {
                    cursor.1 -= 1;
                    output.insert(cursor);
                }
            }
            Op::Left(n) => {
                for _ in 0..n {
                    cursor.0 -= 1;
                    output.insert(cursor);
                }
            }
        }
    }
    output
}

fn parse(s: &str) -> (Vec<Op>, Vec<Op>) {
    let mut iter = s.split('\n');
    let left = iter
        .next()
        .unwrap()
        .split(',')
        .map(|s| Op::from_str(s).unwrap())
        .collect();
    let right = iter
        .next()
        .unwrap()
        .split(',')
        .map(|s| Op::from_str(s).unwrap())
        .collect();
    (left, right)
}

#[derive(Debug)]
enum Op {
    Up(usize),
    Right(usize),
    Down(usize),
    Left(usize),
}

impl FromStr for Op {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..1] {
            "U" => Ok(Self::Up(s[1..].parse()?)),
            "R" => Ok(Self::Right(s[1..].parse()?)),
            "D" => Ok(Self::Down(s[1..].parse()?)),
            "L" => Ok(Self::Left(s[1..].parse()?)),
            c => panic!("unknown char: {}", c),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        assert_eq!(compute(parse(input)), 6);

        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(compute(parse(input)), 159);

        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(compute(parse(input)), 135);
    }
}
