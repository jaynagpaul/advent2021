use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    ops::Deref,
    str::FromStr,
};

fn main() {
    let input = include_str!("../input.txt");

    let ans1 = part1(input);
    println!("Part 1: {}", ans1);
}

struct Graph {
    map: HashMap<String, Vec<String>>,
}

struct Cave;

impl Cave {
    fn is_big(s: &str) -> bool {
        s.chars().all(|c| matches!(c, 'A'..='Z'))
    }

    fn is_smal(s: &str) -> bool {
        !Cave::is_big(s)
    }
}

impl Deref for Graph {
    type Target = HashMap<String, Vec<String>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<'a> FromStr for Graph {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();

        for line in s.lines() {
            let (node1, node2) = line.split_once("-").unwrap();

            map.entry(node1.to_string())
                .or_insert_with(Vec::new)
                .push(node2.to_string());

            map.entry(node2.to_string())
                .or_insert_with(Vec::new)
                .push(node1.to_string());
        }

        Ok(Graph { map })
    }
}

fn part1(input: &str) -> i64 {
    let graph = Graph::from_str(input).unwrap();
    let visited = &mut HashSet::new();

    dfs(&graph, &"start".to_string(), visited)
}

fn dfs<'a>(graph: &'a Graph, node: &'a str, visited: &mut HashSet<&'a str>) -> i64 {
    let mut count = 0;
    visited.insert(node);
    for child in graph.get(node).unwrap() {
        if child == "end" {
            count += 1;
        } else if Cave::is_big(child) || !visited.contains(child as &str) {
            count += dfs(graph, child, visited);
        }
    }

    visited.remove(node);
    count
}
