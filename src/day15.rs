//! https://adventofcode.com/2020/day/15

#![feature(str_split_once)]

// const INPUT: &'static str = include_str!("../inputs/day15_ex1.txt");
use std::collections::HashMap;

fn main() {
    let input: Vec<u64> = vec![1, 3, 2];
    let mut nums: HashMap<u64, Vec<usize>> = HashMap::new();
    let mut count: usize = 0;
    let mut last = 0;

    for (i, n) in input.iter().enumerate() {
        count += 1;
        last = *n;
        let num = nums.entry(*n).or_default();
        num.push(i);
    }

    let mut i = count;
    let mut n = last;

    loop {
        i += 1;

        let num = nums.entry(n).or_default();
        num.push(i);

        if count == 2020 {
            break;
        }
    }
}
