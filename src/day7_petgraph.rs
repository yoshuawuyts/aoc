//! https://adventofcode.com/2020/day/7

use petgraph::{graph::NodeIndex, visit::EdgeRef, Directed, Direction, Graph};
use std::collections::{HashMap, HashSet};

const INPUT: &'static str = include_str!("../inputs/day7.txt");

fn main() {
    let mut tree = Tree::new();
    for line in INPUT.lines() {
        let (parent, tail) = line.split_once("bags contain").unwrap();
        let parent = parent.trim();
        for bag in tail.split(',') {
            let bag = bag
                .trim()
                .trim_end_matches(".")
                .trim_end_matches("bag")
                .trim_end_matches("bags")
                .trim();
            match bag {
                "no other" => tree.add_node(parent),
                s => {
                    let (count, child) = s.split_once(' ').unwrap();
                    let count: u16 = count.parse().unwrap();
                    tree.add_relation(parent, child, count);
                }
            };
        }
    }
    let start = "shiny gold";
    println!("parent count: {}", tree.count_parents(start));
    println!("contained count: {}", tree.count_contained(start));
}

#[derive(Debug)]
struct Tree {
    nodes: HashMap<String, NodeIndex>,
    graph: Graph<String, u16, Directed>,
}

impl Tree {
    /// Create a new instance
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            graph: Graph::new(),
        }
    }

    pub fn add_node(&mut self, name: &str) {
        if !self.nodes.contains_key(name) {
            let node = self.graph.add_node(name.into());
            self.nodes.insert(name.into(), node);
        }
    }

    pub fn get_node(&mut self, name: &str) -> NodeIndex {
        self.add_node(name);
        self.nodes.get(name).unwrap().clone()
    }

    pub fn add_relation(&mut self, parent: &str, child: &str, count: u16) {
        let parent = self.get_node(parent);
        let child = self.get_node(child);
        self.graph.add_edge(parent, child, count);
    }

    #[allow(dead_code)]
    pub fn render(&self) {
        println!("{:?}", petgraph::dot::Dot::new(&self.graph));
    }

    pub fn count_parents(&self, node_name: &str) -> usize {
        fn walk_parents(
            index: NodeIndex,
            graph: &Graph<String, u16, Directed>,
            set: &mut HashSet<NodeIndex>,
        ) {
            for edge in graph.edges_directed(index, Direction::Incoming) {
                let index = edge.source();
                if !set.contains(&index) {
                    set.insert(index);
                    walk_parents(index, &graph, set);
                }
            }
        }

        let mut set = HashSet::new();
        walk_parents(self.nodes[node_name], &self.graph, &mut set);
        set.len()
    }

    fn count_contained(&self, node_name: &str) -> usize {
        fn walk_children(
            index: NodeIndex,
            graph: &Graph<String, u16, Directed>,
            map: &mut HashMap<NodeIndex, usize>,
        ) -> usize {
            let mut total = 1;
            for edge in graph.edges_directed(index, Direction::Outgoing) {
                let weight = edge.weight();
                let index = edge.target();
                match map.get(&index) {
                    Some(count) => total += count * (*weight as usize),
                    None => {
                        let count = walk_children(index, &graph, map);
                        total += count * (*weight as usize);
                        map.insert(index, count);
                    }
                }
            }
            total
        }

        let mut set = HashMap::new();
        walk_children(self.nodes[node_name], &self.graph, &mut set) - 1
    }
}
