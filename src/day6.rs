//! https://adventofcode.com/2020/day/6

const INPUT: &'static str = include_str!("../inputs/day6.txt");
const OFFSET: u32 = 'a' as u32;

fn main() {
    let mut total = 0u64;
    let mut bits = u32::MAX;

    for line in INPUT.lines() {
        if line.len() == 0 {
            total += sum(bits);
            bits = u32::MAX;
        } else {
            let mut mask = 0;
            for c in line.chars() {
                mask |= 1 << ((c as u32) - OFFSET);
            }
            bits &= mask;
        }
    }
    total += sum(bits);
    println!("total: {}", total);
}

fn sum(bits: u32) -> u64 {
    (0..26)
        .map(|n| if bits & (1 << n) != 0 { 1 } else { 0 })
        .sum()
}
