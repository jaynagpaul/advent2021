use std::collections::{hash_map::DefaultHasher, HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    let ans1 = part1(input);
    println!("Part 1: {}", ans1);
}

fn part1(input: &str) -> i32 {
    let mut graph = HashMap::<String, Vec<String>>::new();
    for line in input.lines() {
        let (from, to) = line.split_once("-").unwrap();
        if let Some(tos) = graph.get_mut(from) {
            tos.push(to.to_string());
        } else {
            graph.insert(from.to_string(), vec![to.to_string()]);
        }

        if let Some(froms) = graph.get_mut(to) {
            froms.push(from.to_string());
        } else {
            graph.insert(to.to_string(), vec![from.to_string()]);
        }
    }
    let mut count = 0;
    let path = vec!["start".to_string()];
    walk(&graph, &path, &mut count, false);
    count
}

fn is_uppercase(s: &str) -> bool {
    s.chars().all(|c| matches!(c, 'A'..='Z'))
}

// fn has_double(path: &[String]) -> bool {
//     let mut members = HashSet::new();
//     for mem in path.iter().filter(|s| !is_uppercase(s)) {
//         if members.contains(mem) {
//             return true;
//         }
//         members.insert(mem);
//     }

//     false
// }

fn walk(graph: &HashMap<String, Vec<String>>, path: &[String], count: &mut i32, has_double: bool) {
    fn is_full_path(path: &[String]) -> bool {
        !path.is_empty() && path[path.len() - 1] == "end"
    }

    if is_full_path(path) {
        *count += 1;
        return;
    }

    let last = &path[path.len() - 1];

    for child in graph.get(last).unwrap() {
        if child == "start" {
            continue;
        }
        if !path.contains(child) || is_uppercase(child) || !has_double {
            let mut new_path = vec![];
            for old in path {
                new_path.push(old.clone());
            }

            new_path.push(child.clone());

            let has_double =
                (path.contains(child) && !is_uppercase(child) && !has_double) || has_double;

            walk(graph, &new_path, count, has_double);
        }
    }
}
