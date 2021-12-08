use std::collections::HashSet;

const INPUT: &'static str = include_str!("../inputs/8.txt");

pub fn run() {
    println!("day 8, output 1: {}", parse1(INPUT));
    println!("day 8, output 2: {}", parse2(INPUT));
}

// 1  count: 2
// 4  count: 4
// 7  count: 3
// 8  count: 7
// 9  count: 6, check if `4` is a subset
// 3  count: 5, superset of: 1
// 0  count: 6, superset of: 1
// 6  count: 6, check for `0` first
// 5  count: 5, subset of: 6
// 2  count: 5: check for `3` and `5` first

pub fn parse1(s: &str) -> usize {
    let mut count = 0;
    for line in s.trim().lines() {
        let (_, outputs) = line.split_once('|').unwrap();
        for s in outputs.split(' ') {
            if matches!(s.len(), 2 | 3 | 4 | 7) {
                count += 1
            };
        }
    }
    count
}

pub fn parse2(s: &str) -> usize {
    let mut total = 0;
    for line in s.trim().lines() {
        let (head, outputs) = line.trim().split_once('|').unwrap();

        let head: Vec<HashSet<_>> = head
            .split_whitespace()
            .map(|s| HashSet::from_iter(s.chars()))
            .collect();

        let mut map: Vec<Option<HashSet<char>>> = vec![None; 10];

        let mut remainder = vec![];
        for set in head.into_iter() {
            match set.len() {
                2 => map[1] = Some(set),
                4 => map[4] = Some(set),
                3 => map[7] = Some(set),
                7 => map[8] = Some(set),
                _ => remainder.push(set),
            }
        }

        let head = remainder;
        let mut remainder = vec![];
        for set in head.into_iter() {
            if set.len() == 6 && set.is_superset(map[4].as_ref().unwrap()) {
                map[9] = Some(set);
            } else if set.len() == 5 && set.is_superset(map[1].as_ref().unwrap()) {
                map[3] = Some(set);
            } else {
                remainder.push(set);
            }
        }

        let head = remainder;
        let mut remainder = vec![];
        for set in head.into_iter() {
            if set.len() == 6 && set.is_superset(map[1].as_ref().unwrap()) {
                map[0] = Some(set);
            } else {
                remainder.push(set);
            }
        }

        let head = remainder;
        let mut remainder = vec![];
        for set in head.into_iter() {
            if set.len() == 6 {
                map[6] = Some(set);
            } else {
                remainder.push(set);
            }
        }

        let head = remainder;
        for set in head.into_iter() {
            if set.is_subset(map[6].as_ref().unwrap()) {
                map[5] = Some(set);
            } else {
                map[2] = Some(set);
            }
        }
        let map: Vec<_> = map.into_iter().map(|item| item.unwrap()).collect();

        let mut count = 0;
        for s in outputs.split_whitespace() {
            let set = HashSet::from_iter(s.chars());
            let n = map.iter().position(|cmp| cmp == &set).unwrap();
            count *= 10;
            count += n;
        }
        total += count;
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 26);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 61229);
    }
}
