//! https://adventofcode.com/2020/day/7
#![feature(str_split_once)]

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
                    edges.push((child.to_string(), parent.to_string()));
                }
            };
        }
    }
    nodes.dedup();

    // Create a new iteration context, ...
    let mut iteration = Iteration::new();

    // .. some variables, ..
    let nodes_var = iteration.variable::<(String, String)>("nodes");
    let edges_var = iteration.variable::<(String, String)>("edges");

    // .. load them with some initial values, ..
    nodes_var.insert(nodes.into());
    edges_var.insert(edges.into());

    // .. and then start iterating rules!
    while iteration.changed() {
        // E(a,c) <-  E(a,b), E(b,c)
        nodes_var.from_join(&nodes_var, &edges_var, |_b, a, c| {
            // println!("E({a},{c}) <- E({a},{b}), E({b},{c})", b = b, a = a, c = c);
            (c.clone(), a.clone())
        });
    }

    // let reachable = edges_var.complete();
    let count = nodes_var
        .complete()
        .elements
        .iter()
        .filter(|(from, to)| from != to)
        .filter(|(_from, to)| to == "shiny gold")
        .count();
    println!("{:?}", count);
}
