const INPUT1: &'static str = include_str!("../inputs/1.txt");

pub fn run() {
    let mut total1 = 0;
    let mut total2 = 0;
    for num in INPUT1.lines() {
        total1 += calc1(num);
        total2 += calc2(num);
    }
    println!("day 1.1: {}", total1);
    println!("day 1.2: {}", total2);
}

fn calc1(num: &str) -> u64 {
    let num: u64 = num.parse().unwrap();
    (num / 3) - 2
}

fn calc2(num: &str) -> u64 {
    let mut last: u64 = num.parse().unwrap();
    let mut total = 0;

    while last != 0 {
        last = match (last / 3).checked_sub(2) {
            Some(n) => n,
            None => break,
        };
        total += last;
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(calc1("12"), 2);
        assert_eq!(calc1("14"), 2);
        assert_eq!(calc1("1969"), 654);
        assert_eq!(calc1("100756"), 33583);
    }

    #[test]
    fn part2() {
        assert_eq!(calc2("1969"), 966);
        assert_eq!(calc2("100756"), 50346);
    }
}
