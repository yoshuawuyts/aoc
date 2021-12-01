#![allow(unused)]
use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    str::FromStr,
};

const INPUT: &'static str = include_str!("../inputs/3.txt");

type Coords = (i64, i64);

pub fn run() {
    let (l, r) = compute(parse(INPUT));
    println!("day 3.1: {}", l);
    println!("day 3.2: {}", r);
}

fn compute(ops: (Vec<Op>, Vec<Op>)) -> (i64, usize) {
    let left = ops.0;
    let right = ops.1;

    let (l_set, l_map) = populate_positions(left);
    let (r_set, r_map) = populate_positions(right);
    let intersect = l_set.intersection(&r_set);

    let mut lowest_manhattan = i64::MAX;
    let mut lowest_steps = usize::MAX;
    for (mut l, mut r) in intersect.into_iter() {
        lowest_steps = (l_map[&(l, r)] + r_map[&(l, r)]).min(lowest_steps);
        if l.is_negative() {
            l = -l;
        }
        if r.is_negative() {
            r = -r;
        }
        lowest_manhattan = (l + r).min(lowest_manhattan);
    }
    (lowest_manhattan, lowest_steps)
}

fn populate_positions(ops: Vec<Op>) -> (HashSet<Coords>, HashMap<Coords, usize>) {
    let mut cursor: (i64, i64) = (0, 0);
    let mut distance = 0;
    let mut set = HashSet::with_capacity(ops.len());
    let mut map = HashMap::with_capacity(ops.len());
    for op in ops.into_iter() {
        match op {
            Op::Up(n) => {
                for _ in 0..n {
                    cursor.1 += 1;
                    set.insert(cursor);
                    distance += 1;
                    map.insert(cursor, distance);
                }
            }
            Op::Right(n) => {
                for _ in 0..n {
                    cursor.0 += 1;
                    set.insert(cursor);
                    distance += 1;
                    map.insert(cursor, distance);
                }
            }
            Op::Down(n) => {
                for _ in 0..n {
                    cursor.1 -= 1;
                    set.insert(cursor);
                    distance += 1;
                    map.insert(cursor, distance);
                }
            }
            Op::Left(n) => {
                for _ in 0..n {
                    cursor.0 -= 1;
                    set.insert(cursor);
                    distance += 1;
                    map.insert(cursor, distance);
                }
            }
        }
    }
    (set, map)
}

fn parse(s: &str) -> (Vec<Op>, Vec<Op>) {
    let mut iter = s.split('\n');
    let left = parse(iter.next().unwrap());
    let right = parse(iter.next().unwrap());
    return (left, right);

    fn parse(s: &str) -> Vec<Op> {
        s.split(',').map(|s| Op::from_str(s).unwrap()).collect()
    }
}

enum Op {
    Up(usize),
    Right(usize),
    Down(usize),
    Left(usize),
}

impl std::fmt::Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up(arg1) => write!(f, "Up"),
            Self::Right(arg1) => write!(f, "Right"),
            Self::Down(arg1) => write!(f, "Down"),
            Self::Left(arg1) => write!(f, "Left"),
        }
    }
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
        assert_eq!(compute(parse(input)).0, 6);

        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(compute(parse(input)).0, 159);

        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(compute(parse(input)).0, 135);
    }

    #[test]
    fn part2() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        assert_eq!(compute(parse(input)).1, 610);

        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        assert_eq!(compute(parse(input)).1, 410);
    }
}
