const INPUT: &'static str = include_str!("../inputs/1.txt");

pub(crate) fn run() {
    let count = parse1(INPUT);
    println!("output 1: {}", count);

    let count = parse2(INPUT);
    println!("output 2: {}", count);
}

fn parse1(input: &str) -> usize {
    let lines: Vec<u16> = input.lines().map(|line| line.parse().unwrap()).collect();
    lines.array_windows().filter(|[n1, n2]| n2 > n1).count()
}

fn parse2(input: &str) -> usize {
    let lines: Vec<u16> = input.lines().map(|line| line.parse().unwrap()).collect();
    let temp: Vec<_> = lines
        .array_windows()
        .map(|[n1, n2, n3]| n1 + n2 + n3)
        .collect();
    temp.array_windows().filter(|[n1, n2]| n2 > n1).count()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "199
200
208
210
200
207
240
269
260
263";
    #[test]
    fn first() {
        assert_eq!(parse1(INPUTS), 7);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUTS), 5);
    }
}
