const INPUT: &'static str = include_str!("../inputs/7.txt");

pub fn run() {
    println!("day 7, output 1: {}", parse1(INPUT));
    println!("day 7, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let mut nums: Vec<i32> = s.trim().split(',').map(|s| s.parse().unwrap()).collect();
    let nth = nums.len() / 2;
    let (_, median, _) = nums.select_nth_unstable(nth);
    let median = *median;
    let distance: i32 = nums.into_iter().map(|n| (median - n).abs()).sum();
    distance as usize
}

pub fn parse2(s: &str) -> usize {
    let nums: Vec<i32> = s.trim().split(',').map(|s| s.parse().unwrap()).collect();
    let min = nums.iter().min().unwrap();
    let max = nums.iter().max().unwrap();

    let min: i32 = (*min..=*max)
        .into_iter()
        .map(|pos| {
            nums.iter()
                .map(|num| {
                    let dist = (pos - num).abs();
                    (1 + dist) * dist / 2
                })
                .sum()
        })
        .min()
        .unwrap();

    min as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 37);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 168);
    }
}
