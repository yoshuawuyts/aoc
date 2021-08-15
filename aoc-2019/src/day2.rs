const INPUT: &'static str = include_str!("../inputs/2.txt");

use const_combinations::IterExt;

pub fn run() {
    let base = str_to_buf(INPUT);
    let mut data = base.clone();
    data[1] = 12;
    data[2] = 2;
    println!("day 2.1: {}", calc1(data)[0]);

    for [i, j] in (0..99).permutations() {
        let mut data = base.clone();
        data[1] = i;
        data[2] = j;
        if calc1(data)[0] == 19690720 {
            println!("day 2.2: {}", 100 * i + j);
            break;
        }
    }
}

fn calc1(mut data: Vec<usize>) -> Vec<usize> {
    let mut cursor = 0;
    loop {
        match Ops::from(data[cursor]) {
            Ops::Add => {
                let lhs = data[cursor + 1];
                let rhs = data[cursor + 2];
                let out = data[cursor + 3];
                data[out] = data[lhs] + data[rhs];
                cursor += 4;
            }
            Ops::Mult => {
                let lhs = data[cursor + 1];
                let rhs = data[cursor + 2];
                let out = data[cursor + 3];
                data[out] = data[lhs] * data[rhs];
                cursor += 4;
            }
            Ops::Eof => break,
        };
    }
    data
}

#[derive(Debug)]
enum Ops {
    Add,
    Mult,
    Eof,
}

impl From<usize> for Ops {
    fn from(target: usize) -> Self {
        match target {
            1 => Self::Add,
            2 => Self::Mult,
            99 => Self::Eof,
            n => panic!("Unknown opcode: {}", n),
        }
    }
}

fn str_to_buf(s: &str) -> Vec<usize> {
    let mut output = Vec::new();
    for chunk in s.split(',') {
        output.push(chunk.trim().parse().unwrap());
    }
    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(calc1(str_to_buf("1,0,0,0,99")), vec![2, 0, 0, 0, 99]);
        assert_eq!(calc1(str_to_buf("2,3,0,3,99")), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            calc1(str_to_buf("2,4,4,5,99,0")),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            calc1(str_to_buf("1,1,1,4,99,5,6,0,99")),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
