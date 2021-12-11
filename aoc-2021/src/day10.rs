const INPUT: &'static str = include_str!("../inputs/10.txt");

pub fn run() {
    println!("day 10, output 1: {}", parse1(INPUT));
    println!("day 10, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let mut count = 0;
    let mut tokens = vec![];
    for line in s.lines() {
        for char in line.chars() {
            match char {
                '{' => {
                    let token = Token::LCurl;
                    tokens.push(token);
                }
                '<' => {
                    let token = Token::LPoint;
                    tokens.push(token);
                }
                '(' => {
                    let token = Token::LBrace;
                    tokens.push(token);
                }
                '[' => {
                    let token = Token::LBrack;
                    tokens.push(token);
                }
                ']' => match tokens.pop() {
                    None => break,
                    Some(prev) => {
                        if prev != Token::LBrack {
                            count += 57;
                            break;
                        }
                    }
                },
                '}' => match tokens.pop() {
                    None => break,
                    Some(prev) => {
                        if prev != Token::LCurl {
                            count += 1197;
                            break;
                        }
                    }
                },
                ')' => match tokens.pop() {
                    None => break,
                    Some(prev) => {
                        if prev != Token::LBrace {
                            count += 3;
                            break;
                        }
                    }
                },
                '>' => match tokens.pop() {
                    None => break,
                    Some(prev) => {
                        if prev != Token::LPoint {
                            count += 25137;
                            break;
                        }
                    }
                },
                _ => unreachable!(),
            };
        }
    }
    count
}

#[derive(Debug, PartialEq)]
enum Token {
    LCurl,  // {
    // RCurl,  // }
    LBrack, // [
    // RBrack, // ]
    LBrace, // (
    // RBrace, // )
    LPoint, // <
    // RPoint, // >
}

pub fn parse2(s: &str) -> usize {
    let mut scores = vec![];
    for line in s.lines() {
        score_line(line, &mut scores);
    }
    let index = scores.len() / 2;
    *scores.select_nth_unstable(index).1
}

fn score_line(line: &str, scores: &mut Vec<usize>) {
    let mut tokens = vec![];
    for char in line.chars() {
        match char {
            '{' => {
                let token = Token::LCurl;
                tokens.push(token);
            }
            '<' => {
                let token = Token::LPoint;
                tokens.push(token);
            }
            '(' => {
                let token = Token::LBrace;
                tokens.push(token);
            }
            '[' => {
                let token = Token::LBrack;
                tokens.push(token);
            }
            ']' => match tokens.pop() {
                None => break,
                Some(prev) => {
                    if prev != Token::LBrack {
                        return;
                    }
                }
            },
            '}' => match tokens.pop() {
                None => break,
                Some(prev) => {
                    if prev != Token::LCurl {
                        return;
                    }
                }
            },
            ')' => match tokens.pop() {
                None => break,
                Some(prev) => {
                    if prev != Token::LBrace {
                        return;
                    }
                }
            },
            '>' => match tokens.pop() {
                None => break,
                Some(prev) => {
                    if prev != Token::LPoint {
                        return;
                    }
                }
            },
            _ => unreachable!(),
        };
    }
    let mut score = 0;
    for token in tokens.into_iter().rev() {
        score *= 5;
        match token {
            Token::LCurl => score += 3,
            Token::LBrack => score += 2,
            Token::LBrace => score += 1,
            Token::LPoint => score += 4,
            // _ => unreachable!(),
        }
    }
    scores.push(score);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT), 26397);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT), 288957);
    }
}
