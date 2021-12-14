use std::collections::HashMap;

use crate::stdx::IterExt;

const INPUT: &'static str = include_str!("../inputs/14.txt");

pub fn run() {
    println!("day 14, output 1: {}", parse1(INPUT));
    println!("day 14, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let (input, rules) = s.split_once("\n\n").unwrap();
    let mut input: Vec<_> = input.chars().collect();

    let map: HashMap<_, _> = rules
        .lines()
        .map(|line| {
            let (pat, val) = line.split_once(" -> ").unwrap();
            let val = val.chars().next().unwrap();
            let mut pat = pat.chars();
            let lpat = pat.next().unwrap();
            let rpat = pat.next().unwrap();
            ((lpat, rpat), val)
        })
        .collect();

    for _ in 0..10 {
        let mut output = Vec::with_capacity(input.len());
        let last = input.len() - 2;
        for (i, [l, r]) in input.into_iter().array_windows().enumerate() {
            match map.get(&(l, r)) {
                Some(new) => output.extend([l, *new]),
                None => output.extend([l]),
            }

            if i == last {
                output.push(r);
            }
        }
        input = output;
    }

    let map: HashMap<char, usize> = input.into_iter().fold(HashMap::new(), |mut acc, val| {
        let count = acc.entry(val).or_default();
        *count += 1;
        acc
    });

    let mut max = usize::MIN;
    let mut min = usize::MAX;
    for (_, val) in map.into_iter() {
        min = val.min(min);
        max = val.max(max);
    }

    max - min
}

pub fn parse2(s: &str) -> usize {
    let (input, rules) = s.split_once("\n\n").unwrap();
    let input: Vec<_> = input.chars().collect();
    let last_char = input[input.len() - 1];

    let rules_map: HashMap<_, _> = rules
        .lines()
        .map(|line| {
            let (pat, val) = line.split_once(" -> ").unwrap();
            let val = val.chars().next().unwrap();
            let mut pat = pat.chars();
            let lpat = pat.next().unwrap();
            let rpat = pat.next().unwrap();
            ((lpat, rpat), val)
        })
        .collect();

    let mut count_map: HashMap<(char, char), usize> = HashMap::with_capacity(input.len());
    for [l, r] in input.into_iter().array_windows() {
        *count_map.entry((l, r)).or_default() += 1;
    }

    for _ in 0..40 {
        let mut new_count_map: HashMap<(char, char), usize> = HashMap::new();
        for ((l, r), count) in count_map {
            let new = rules_map.get(&(l, r)).unwrap();
            *new_count_map.entry((l, *new)).or_default() += count;
            *new_count_map.entry((*new, r)).or_default() += count;
        }
        count_map = new_count_map;
    }

    let mut map: HashMap<char, usize> =
        count_map
            .into_iter()
            .fold(HashMap::new(), |mut acc, ((l, _), count)| {
                *acc.entry(l).or_default() += count;
                acc
            });

    *map.get_mut(&last_char).unwrap() += 1;

    let mut max = usize::MIN;
    let mut min = usize::MAX;
    for (_, val) in map.into_iter() {
        min = val.min(min);
        max = val.max(max);
    }

    max - min
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 1588);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 2188189693529);
    }
}
