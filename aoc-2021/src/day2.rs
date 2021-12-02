const INPUT: &'static str = include_str!("../inputs/2.txt");

pub fn run() {
    let count = parse1(INPUT);
    println!("day 2, output 1: {}", count);

    let count = parse2(INPUT);
    println!("day 2, output 2: {}", count);
}

pub fn parse1(s: &str) -> usize {
    let mut depth = 0usize;
    let mut hor_pos = 0usize;

    for line in s.lines() {
        let cmd: Command = line.parse().unwrap();
        match cmd {
            Command::Forward(num) => hor_pos += num,
            Command::Down(num) => depth += num,
            Command::Up(num) => depth -= num,
        }
    }

    depth as usize * hor_pos
}

pub fn parse2(s: &str) -> usize {
    let mut depth = 0usize;
    let mut hor_pos = 0usize;
    let mut aim = 0usize;

    for line in s.lines() {
        let cmd: Command = line.parse().unwrap();
        match cmd {
            Command::Forward(num) => {
                hor_pos += num;
                depth += aim * num;
            }
            Command::Down(num) => aim += num,
            Command::Up(num) => aim -= num,
        }
    }

    depth as usize * hor_pos
}

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl std::str::FromStr for Command {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_once(' ').unwrap();
        let num = tail.parse().unwrap();

        match head {
            "forward" => Ok(Command::Forward(num)),
            "up" => Ok(Command::Up(num)),
            "down" => Ok(Command::Down(num)),
            _ => panic!("unreachable"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test1() {
        assert_eq!(parse1(INPUT), 150);
    }

    #[test]
    fn test2() {
        assert_eq!(parse2(INPUT), 900);
    }
}
