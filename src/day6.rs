//! https://adventofcode.com/2020/day/6

const INPUT: &'static str = include_str!("../inputs/day6.txt");

fn main() {
    let mut total = 0u32;
    let mut bits = u32::MAX;

    for line in INPUT.lines() {
        if line.len() == 0 {
            total += bits.count_ones();
            bits = u32::MAX;
        } else {
            let mut mask = 0;
            for c in line.chars() {
                mask |= 1 << ((c as u32) - 'a' as u32);
            }
            bits &= mask;
        }
    }
    total += bits.count_ones();
    println!("total: {}", total);
}
