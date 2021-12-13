use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/12.txt");

pub fn run() {
    println!("day 12, output 1: {}", parse1(INPUT));

    // NOTE: this takes a long time to compute, disabled for performance
    // println!("day 12, output 2: {}", parse2(INPUT));
}

pub fn parse1(s: &str) -> usize {
    let mut graph = Graph::new();
    for line in s.lines() {
        let (left, right) = line.split_once('-').unwrap();
        let left = left.parse().unwrap();
        let right = right.parse().unwrap();
        graph.insert(left, right);
    }

    let visited: HashSet<Node> = HashSet::new();
    count_paths(&graph, Node::Start, visited)
}

fn count_paths(graph: &Graph, current: Node, mut visited: HashSet<Node>) -> usize {
    if current == Node::End {
        return 1;
    } else if current.is_small() && visited.contains(&current) {
        return 0;
    } else {
        if current.is_small() {
            visited.insert(current.clone());
        }
        graph
            .get(&current)
            .unwrap()
            .into_iter()
            .map(|adj| count_paths(&graph, adj.clone(), visited.clone()))
            .sum()
    }
}

fn count_paths2(
    graph: &Graph,
    current: Node,
    mut visited: HashSet<Node>,
    mut twice: bool,
) -> usize {
    if current == Node::End {
        return 1;
    } else if current.is_small() && visited.contains(&current) {
        if twice {
            return 0;
        } else {
            twice = true;
        }
    }
    if current.is_small() {
        visited.insert(current.clone());
    }
    graph
        .get(&current)
        .unwrap()
        .into_iter()
        .map(|adj| count_paths2(&graph, adj.clone(), visited.clone(), twice))
        .sum()
}

#[allow(unused)]
pub fn parse2(s: &str) -> usize {
    let mut graph = Graph::new();
    for line in s.lines() {
        let (left, right) = line.split_once('-').unwrap();
        let left = left.parse().unwrap();
        let right = right.parse().unwrap();
        graph.insert(left, right);
    }

    let visited: HashSet<Node> = HashSet::new();
    count_paths2(&graph, Node::Start, visited, false)
}

#[derive(Debug)]
struct Graph {
    map: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, left: Node, right: Node) {
        let values = self.map.entry(left.clone()).or_default();
        if !right.is_start() {
            values.push(right.clone());
        }

        let values = self.map.entry(right).or_default();
        if !left.is_start() {
            values.push(left);
        }
    }

    fn get(&self, node: &Node) -> Option<&Vec<Node>> {
        self.map.get(node)
    }
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Clone, Hash)]
enum Node {
    Start,
    End,
    Large(String),
    Small(String),
}

impl Node {
    /// Returns `true` if the node is [`Small`].
    ///
    /// [`Small`]: Node::Small
    fn is_small(&self) -> bool {
        matches!(self, Self::Small(..))
    }

    /// Returns `true` if the node is [`Start`].
    ///
    /// [`Start`]: Node::Start
    fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }
}

impl FromStr for Node {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "start" {
            return Ok(Self::Start);
        } else if s == "end" {
            return Ok(Self::End);
        } else if s.chars().all(|c| c.is_ascii_lowercase()) {
            return Ok(Self::Small(s.to_owned()));
        } else {
            return Ok(Self::Large(s.to_owned()));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const INPUT2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const INPUT3: &str = "fs-end
he-D12
fs-he
start-D12
pj-D12
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-D12
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn first() {
        assert_eq!(parse1(INPUT1), 10);
        assert_eq!(parse1(INPUT2), 19);
        assert_eq!(parse1(INPUT3), 226);
    }

    #[test]
    fn second() {
        assert_eq!(parse2(INPUT1), 36);
        assert_eq!(parse2(INPUT2), 103);
        assert_eq!(parse2(INPUT3), 3509);
    }
}
