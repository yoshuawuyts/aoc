const INPUT: &'static str = include_str!("../inputs/11.txt");

pub fn run() {
    println!("day 11, output 1: {}", parse1(INPUT));
    println!("day 11, output 2: {}", parse2(INPUT));
}

type Grid = Vec<Vec<State>>;

pub fn parse1(s: &str) -> usize {
    let mut grid = make_grid(s);

    let mut flash_count = 0;
    for _ in 0..100 {
        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                match cell {
                    State::Flagged => continue,
                    State::Wall => continue,
                    State::Value(num) => *num += 1,
                }
            }
        }

        for y in 0..grid.len() {
            let row_len = grid[y].len();
            for x in 0..row_len {
                let cell = &mut grid[y][x];
                match cell {
                    State::Flagged => continue,
                    State::Wall => continue,
                    State::Value(num) => {
                        if *num > 9 {
                            update(&mut grid, x, y, &mut flash_count);
                        }
                    }
                }
            }
        }
        //
        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                match cell {
                    State::Flagged => {
                        *cell = State::Value(0);
                    }
                    State::Wall => continue,
                    State::Value(_) => continue,
                }
            }
        }
    }
    flash_count
}

fn update(mut grid: &mut Grid, x: usize, y: usize, flash_count: &mut usize) {
    let cell = &mut grid[y][x];
    *flash_count += 1;
    *cell = State::Flagged;

    for y1 in (y - 1)..=(y + 1) {
        for x1 in (x - 1)..=(x + 1) {
            let cell = &mut grid[y1][x1];
            match cell {
                State::Flagged => continue,
                State::Wall => continue,
                State::Value(num) => {
                    *num += 1;
                    if *num > 9 {
                        update(&mut grid, x1, y1, flash_count);
                    }
                }
            }
        }
    }
}

fn make_grid(s: &str) -> Grid {
    let mut grid: Grid = s
        .lines()
        .map(|line| {
            let mut row: Vec<_> = line
                .chars()
                .map(|char| State::Value(char.to_string().parse().unwrap()))
                .collect();
            row.insert(0, State::Wall);
            row.push(State::Wall);
            row
        })
        .collect();
    grid.insert(0, vec![State::Wall; grid[0].len()]);
    grid.push(vec![State::Wall; grid[0].len()]);
    grid
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    /// This octopus has been flagged this turn.
    Flagged,
    /// This is a wall as part of the halo.
    Wall,
    /// This octopus holds a value.
    Value(u8),
}

pub fn parse2(s: &str) -> usize {
    let mut grid = make_grid(s);

    let mut flash_count = 0;
    for step in 1.. {
        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                match cell {
                    State::Flagged => continue,
                    State::Wall => continue,
                    State::Value(num) => *num += 1,
                }
            }
        }

        for y in 0..grid.len() {
            let row_len = grid[y].len();
            for x in 0..row_len {
                let cell = &mut grid[y][x];
                match cell {
                    State::Flagged => continue,
                    State::Wall => continue,
                    State::Value(num) => {
                        if *num > 9 {
                            update(&mut grid, x, y, &mut flash_count);
                        }
                    }
                }
            }
        }

        let complete = grid.iter().all(|line| {
            line.iter()
                .all(|cell| matches!(*cell, State::Flagged | State::Wall))
        });

        if complete {
            return step;
        }

        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                match cell {
                    State::Flagged => {
                        *cell = State::Value(0);
                    }
                    State::Wall => continue,
                    State::Value(_) => continue,
                }
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 1656);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 195);
    }
}
