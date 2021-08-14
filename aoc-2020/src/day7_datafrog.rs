//! https://adventofcode.com/2020/day/7

const INPUT: &'static str = include_str!("../inputs/day7_ex1.txt");

use datafrog::Iteration;

fn main() {
    let mut nodes = vec![];
    let mut edges = vec![];

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
                "no other" => nodes.push((parent.to_string(), parent.to_string())),
                s => {
                    let (count, child) = s.split_once(' ').unwrap();
                    let _count: u16 = count.parse().unwrap();
                    nodes.push((parent.to_string(), parent.to_string()));
                    nodes.push((child.to_string(), child.to_string()));
                    edges.push((parent.to_string(), child.to_string()));
                }
            };
        }
    }

    // Compute the graph
    // N(c,a) <- N(b,a), E(b,c)
    let mut iteration = Iteration::new();
    let edges_var = iteration.variable::<(String, String)>("edges");
    let nodes_var = iteration.variable::<(String, String)>("nodes");
    edges_var.insert(edges.into());
    nodes_var.insert(nodes.into());
    while iteration.changed() {
        // join tuples (b, a) + (b, c) into (c, a)
        nodes_var.from_join(&nodes_var, &edges_var, |b, a, c| {
            println!("{} -> {} -> {}\t\t{} <- {}", a, b, c, c, a);
            (c.clone(), a.clone())
        });
    }
    let graph = nodes_var.complete();
    let count = graph
        .elements
        .iter()
        .filter(|(child, parent)| parent != child)
        .filter(|(child, _)| child == "shiny gold")
        .count();
    println!("{:?}", count);
}
