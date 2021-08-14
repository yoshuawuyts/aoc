//! https://adventofcode.com/2020/day/6

const INPUT: &'static str = include_str!("../inputs/day8_ex2.txt");
use std::collections::HashSet;

fn main() {
    let mut ops: Vec<OpCode> = vec![];
    for line in INPUT.lines() {
        let op = match line.split_once(' ').unwrap() {
            ("nop", n) => OpCode::Nop(n.parse().unwrap()),
            ("acc", n) => OpCode::Acc(n.parse().unwrap()),
            ("jmp", n) => OpCode::Jmp(n.parse().unwrap()),
            _ => panic!("uknown op code"),
        };
        ops.push(op);
    }

    let visited: HashSet<usize> = HashSet::new();
    let cursor: usize = 0;
    let acc: isize = 0;

    let acc = run(ops, cursor, acc, visited, true).unwrap();
    println!("acc: {}", acc);
}

fn attempt_repair(ops: &[OpCode], cursor: usize) -> Option<Vec<OpCode>> {
    match ops[cursor] {
        OpCode::Nop(n) => {
            let mut ops = ops.to_vec();
            ops[cursor] = OpCode::Jmp(n);
            Some(ops)
        }
        OpCode::Acc(_) => None,
        OpCode::Jmp(n) => {
            let mut ops = ops.to_vec();
            ops[cursor] = OpCode::Nop(n);
            Some(ops)
        }
    }
}

fn run(
    ops: Vec<OpCode>,
    mut cursor: usize,
    mut acc: isize,
    mut visited: HashSet<usize>,
    mut should_checkpoint: bool,
) -> Option<isize> {
    loop {
        println!("{:?}\t| {:?}", ops.get(cursor), visited.len() + 1);
        if cursor == ops.len() {
            return Some(acc);
        }

        let mut checkpoint = None;
        if should_checkpoint {
            if let Some(ops) = attempt_repair(&ops, cursor) {
                checkpoint = Some((ops, cursor, acc, visited.clone()));
            }
        };

        if visited.contains(&cursor) {
            println!("ResetFrom({})", visited.len() + 1);
            return None;
        } else {
            visited.insert(cursor);
        }

        match ops[cursor] {
            OpCode::Nop(_) => cursor += 1,
            OpCode::Acc(n) => {
                acc += n;
                cursor += 1;
            }
            OpCode::Jmp(n) => cursor = cursor.wrapping_add(n as usize),
        };

        if let Some((ops, cursor, acc, visited)) = checkpoint {
            if let Some(acc) = run(ops, cursor, acc, visited.clone(), false) {
                return Some(acc);
            }
        }
        should_checkpoint = true;
    }
}

#[derive(Debug, Clone)]
pub enum OpCode {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}
