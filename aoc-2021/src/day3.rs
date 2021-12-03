use std::collections::VecDeque;

const INPUT: &'static str = include_str!("../inputs/3.txt");
pub fn run() {
    println!("day 3, output 1: {}", parse1(INPUT));
    println!("day 3, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let mut len = None;
    let numbers: Vec<_> = s
        .lines()
        .map(|line| {
            if len.is_none() {
                len = Some(line.len());
            }
            usize::from_str_radix(line, 2).unwrap()
        })
        .collect();

    let mut counters = vec![0; len.unwrap()];

    let line_count = numbers.len();
    for mut num in numbers {
        let mut idx = 0;
        while num != 0 {
            if (num & 1) == 1 {
                counters[idx] += 1;
            }
            idx += 1;
            num >>= 1;
        }
    }

    let gamma = counters
        .iter()
        .map(|count| if *count >= (line_count / 2) { 1 } else { 0 })
        .rfold(0, |acc, x| (acc << 1) | x);

    let epsilon = counters
        .into_iter()
        .map(|count| if count >= (line_count / 2) { 0 } else { 1 })
        .rfold(0, |acc, x| (acc << 1) | x);

    gamma * epsilon
}

pub fn parse2(s: &str) -> usize {
    let numbers: Vec<VecDeque<usize>> = s
        .lines()
        .map(|line| line.chars().map(|char| (char == '1') as usize).collect())
        .collect();

    let mut oxygen_nums = numbers.clone();
    for counter in 0.. {
        let mut count_ones = 0; // how many instances of the number have we seen?
        for number in oxygen_nums.clone().into_iter() {
            let bit = number[counter];
            if bit == 1 {
                count_ones += 1;
            }
        }

        let should_keep = count_ones >= (oxygen_nums.len() - count_ones);

        oxygen_nums = oxygen_nums
            .clone()
            .into_iter()
            .filter(|num| {
                let num = num[counter];
                if should_keep {
                    num == 1
                } else {
                    num == 0
                }
            })
            .collect();

        if (&oxygen_nums).len() <= 1 {
            break;
        }
    }

    let oxygen_rating = oxygen_nums[0]
        .clone()
        .into_iter()
        .fold(0, |acc, x| (acc << 1) | x);
    // println!("oxygen_rating: {:?}", oxygen_rating);

    let mut co2_nums = numbers.clone();
    for counter in 0.. {
        let mut count_zeros = 0; // how many instances of the number have we seen?
        for number in co2_nums.clone().into_iter() {
            let bit = number[counter];
            if bit == 0 {
                count_zeros += 1;
            }
        }

        let should_keep = count_zeros <= (co2_nums.len() - count_zeros);

        co2_nums = co2_nums
            .clone()
            .into_iter()
            .filter(|num| {
                let num = num[counter];
                if should_keep {
                    num == 0
                } else {
                    num == 1
                }
            })
            .collect();

        if (&co2_nums).len() <= 1 {
            break;
        }
    }

    let c02_rating = co2_nums[0]
        .clone()
        .into_iter()
        .fold(0, |acc, x| (acc << 1) | x);
    // println!("c02 rating: {:?}", c02_rating);

    oxygen_rating * c02_rating
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 198);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 230);
    }
}
