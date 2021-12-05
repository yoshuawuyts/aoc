const INPUT: &'static str = include_str!("../inputs/5.txt");

pub fn run() {
    println!("day 5, output 1: {}", parse1(INPUT));
    println!("day 5, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    todo!()
}

pub fn parse2(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 0);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 0);
    }
}
