use std::{ops::Not, str::FromStr};

const INPUT: &'static str = include_str!("../inputs/4.txt");
pub fn run() {
    println!("day 4, output 1: {}", parse1(INPUT));
    println!("day 4, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let mut items = s.split("\n\n");
    let drawn = parse_drawn(&mut items);
    let mut boards: Vec<Board> = items.map(|board| board.parse().unwrap()).collect();

    for num in drawn {
        for board in boards.iter_mut() {
            board.mark_number(num);
            if board.validate() {
                return board.sum_unmarked() * num as usize;
            }
        }
    }

    panic!("no answer found!");
}

pub fn parse2(s: &str) -> usize {
    let mut items = s.split("\n\n");
    let drawn = parse_drawn(&mut items);
    let mut boards: Vec<Board> = items.map(|board| board.parse().unwrap()).collect();

    let board_count = boards.len();
    let mut done_count = 0;

    for num in drawn {
        for board in boards.iter_mut() {
            if board.is_done() {
                continue;
            }
            board.mark_number(num);
            if board.validate() {
                board.mark_done(true);
                done_count += 1;
                if done_count == board_count {
                    return board.sum_unmarked() * num as usize;
                }
            }
        }
    }

    panic!("no answer found!");
}

fn parse_drawn(items: &mut std::str::Split<&str>) -> Vec<u8> {
    let drawn: Vec<u8> = items
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();
    drawn
}

#[derive(Debug)]
struct Board {
    is_done: bool,
    rows: Vec<Vec<(u8, bool)>>,
    columns: Vec<Vec<(u8, bool)>>,
}

impl Board {
    fn mark_number(&mut self, target: u8) {
        'row: for row in self.rows.iter_mut() {
            for (num, bool) in row.iter_mut() {
                if target == *num {
                    *bool = true;
                    break 'row;
                }
            }
        }
        'column: for col in self.columns.iter_mut() {
            for (num, bool) in col.iter_mut() {
                if target == *num {
                    *bool = true;
                    break 'column;
                }
            }
        }
    }

    fn validate(&self) -> bool {
        for row in self.rows.iter() {
            if row.iter().all(|(_, marked)| *marked) {
                return true;
            }
        }
        for col in self.columns.iter() {
            if col.iter().all(|(_, marked)| *marked) {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> usize {
        let count: usize = self
            .rows
            .iter()
            .map(|row| {
                row.into_iter()
                    .filter_map(|(num, marked)| marked.not().then(|| *num as usize))
            })
            .flatten()
            .sum();
        count
    }

    /// Mark the board as done.
    fn mark_done(&mut self, is_done: bool) {
        self.is_done = is_done;
    }

    /// Get a reference to the board's is done.
    fn is_done(&self) -> bool {
        self.is_done
    }
}

impl FromStr for Board {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<(u8, bool)>> = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| (s.parse().unwrap(), false))
                    .collect()
            })
            .collect();

        let mut columns: Vec<Vec<(u8, bool)>> = vec![vec![(0, false); rows.len()]; rows.len()];
        for (i, row) in rows.iter().enumerate() {
            for (j, (num, _)) in row.iter().enumerate() {
                columns[j][i].0 = *num;
            }
        }
        Ok(Self {
            rows,
            columns,
            is_done: false,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 4512);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 1924);
    }
}
