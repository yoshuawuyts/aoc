const INPUT: &'static str = include_str!("../inputs/6.txt");

pub fn run() {
    println!("day 6, output 1: {}", parse1(INPUT));
    println!("day 6, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let mut buckets = Buckets::new();
    for num in s.split(',') {
        buckets.insert_fish(num.trim().parse().unwrap());
    }
    (0..80).for_each(|_| buckets.step_day());
    buckets.count_fish()
}

pub fn parse2(s: &str) -> usize {
    let mut buckets = Buckets::new();
    for num in s.split(',') {
        buckets.insert_fish(num.trim().parse().unwrap());
    }
    (0..256).for_each(|_| buckets.step_day());
    buckets.count_fish()
}

#[derive(Debug)]
struct Buckets {
    buckets: [usize; 9],
}

impl Buckets {
    fn new() -> Self {
        Self { buckets: [0; 9] }
    }

    fn insert_fish(&mut self, index: usize) {
        self.buckets[index] += 1;
    }

    fn step_day(&mut self) {
        self.buckets.rotate_left(1);
        self.buckets[6] += self.buckets[8];
    }

    fn count_fish(&self) -> usize {
        self.buckets.into_iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 5934);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 26984457539);
    }
}
