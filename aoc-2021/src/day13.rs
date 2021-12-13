use std::collections::HashSet;

const INPUT: &'static str = include_str!("../inputs/13.txt");

pub fn run() {
    println!("day 13, output 1: {}", parse1(INPUT));
    println!("day 13, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let (head, tail) = s.split_once("\n\n").unwrap();
    let mut set = create_set(head);
    let folds = create_folds(tail);

    // just the first fold
    fold(folds.into_iter().take(1), &mut set);
    set.iter().count()
}

pub fn parse2(s: &str) -> usize {
    let (head, tail) = s.split_once("\n\n").unwrap();
    let mut set = create_set(head);
    let folds = create_folds(tail);
    fold(folds.into_iter(), &mut set);

    let set: Vec<_> = set.into_iter().collect();
    let max_x = set.iter().map(|(x, _)| x).max().unwrap();
    let max_y = set.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=(*max_y as usize) {
        for x in 0..=(*max_x as usize) {
            if set.contains(&(x as u16, y as u16)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    0
}

fn fold(folds: impl Iterator<Item = Fold>, set: &mut HashSet<(u16, u16)>) {
    for fold in folds {
        *set = set
            .iter()
            .map(|(mut x, mut y)| match fold {
                Fold::X(line) => {
                    if x > line {
                        let delta = x - line;
                        x = line - delta;
                    }
                    (x, y)
                }
                Fold::Y(line) => {
                    if y > line {
                        let delta = y - line;
                        y = line - delta;
                    }
                    (x, y)
                }
            })
            .collect();
    }
}

fn create_folds(tail: &str) -> Vec<Fold> {
    let folds: Vec<_> = tail
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace().skip(2);
            let s = iter.next().unwrap();
            let (head, tail) = s.split_once('=').unwrap();
            match head {
                "x" => Fold::X(tail.parse().unwrap()),
                "y" => Fold::Y(tail.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect();
    folds
}

fn create_set(head: &str) -> HashSet<(u16, u16)> {
    head.lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x: u16 = x.parse().unwrap();
            let y: u16 = y.parse().unwrap();
            (x, y)
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Fold {
    X(u16),
    Y(u16),
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 17);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 0);
    }
}
