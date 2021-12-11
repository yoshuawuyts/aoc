const INPUT: &'static str = include_str!("../inputs/9.txt");

pub fn run() {
    println!("day 9, output 1: {}", parse1(INPUT));
    // println!("day 9, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let grid: Vec<Vec<u8>> = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let c_top = 0;
    let c_right = grid[0].len() - 1;
    let c_bottom = grid.len() - 1;
    let c_left = 0;

    let mut sum: usize = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, num) in line.into_iter().enumerate() {
            if y != c_top && *num >= grid[y - 1][x].into() {
                continue;
            }

            if y != c_bottom && *num >= grid[y + 1][x].into() {
                continue;
            }

            if x != c_left && *num >= grid[y][x - 1].into() {
                continue;
            }

            if x != c_right && *num >= grid[y][x + 1].into() {
                continue;
            }

            sum += *num as usize + 1;
        }
    }
    sum
}

// pub fn parse2(_s: &str) -> usize {
//     todo!()
// }

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 15);
    }

    // #[test]
    // fn second() {
    //     assert_eq!(parse2(INPUT), 0);
    // }
}
