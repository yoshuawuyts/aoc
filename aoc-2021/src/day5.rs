use std::collections::HashSet;

const INPUT: &'static str = include_str!("../inputs/5.txt");
pub fn run() {
    println!("day 5, output 1: {}", parse1(INPUT));
    println!("day 5, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let mut intersects: HashSet<(usize, usize)> = HashSet::new();
    let mut items: HashSet<(usize, usize)> = HashSet::new();

    for line in s.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        let (x1, y1) = parse_coord(left.split_once(',').unwrap());
        let (x2, y2) = parse_coord(right.split_once(',').unwrap());

        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                let coord = (x1, y);
                if !items.insert(coord) {
                    intersects.insert(coord);
                }
            }
        }
        if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                let coord = (x, y1);
                if !items.insert(coord) {
                    intersects.insert(coord);
                }
            }
        }
    }
    intersects.len()
}

fn parse_coord((l, r): (&str, &str)) -> (usize, usize) {
    (l.parse().unwrap(), r.parse().unwrap())
}

pub fn parse2(s: &str) -> usize {
    let mut intersects: HashSet<(usize, usize)> = HashSet::new();
    let mut items: HashSet<(usize, usize)> = HashSet::new();

    for line in s.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        let (x1, y1) = parse_coord(left.split_once(',').unwrap());
        let (x2, y2) = parse_coord(right.split_once(',').unwrap());

        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                let coord = (x1, y);
                if !items.insert(coord) {
                    intersects.insert(coord);
                }
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                let coord = (x, y1);
                if !items.insert(coord) {
                    intersects.insert(coord);
                }
            }
        } else {
            let x_direction: isize = if x1 < x2 { 1 } else { -1 };
            let y_direction: isize = if y1 < y2 { 1 } else { -1 };

            for distance in 0..=(x2 as isize - x1 as isize).abs() {
                let coord = (
                    x1 as isize + distance * x_direction,
                    y1 as isize + distance * y_direction,
                );

                let coord = (coord.0 as usize, coord.1 as usize);
                if !items.insert(coord) {
                    intersects.insert(coord);
                }
            }
        }
    }
    intersects.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 5);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 12);
    }
}
