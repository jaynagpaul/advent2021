use std::{collections::HashMap, ops::RangeBounds};

fn main() {
    let input = include_str!("../input.txt");

    let ans1 = part1(input);
    println!("Part 1: {}", ans1);

    let ans2 = part2(input);
    println!("Part 2: {}", ans2);
}

#[derive(Debug)]
struct Entry {
    signals: Vec<String>,
    output: Vec<String>,
}
fn part1(input: &str) -> i32 {
    let mut entries = vec![];

    for line in input.lines() {
        let (signals, output) = line.split_once(" | ").unwrap();

        let signals = signals.split(' ').map(|s| s.to_string()).collect();
        let output = output.split(' ').map(|s| s.to_string()).collect();

        let entry = Entry { signals, output };

        entries.push(entry);
    }

    let mut count = 0;
    for entry in entries {
        for signal in entry.output {
            // if signal length is 2, 3, 4, or 7 then increment count
            if signal.len() == 2 || signal.len() == 3 || signal.len() == 4 || signal.len() == 7 {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: &str) -> i32 {
    let mut entries = vec![];

    for line in input.lines() {
        let (signals, output) = line.split_once(" | ").unwrap();

        let signals = signals.split(' ').map(|s| s.to_string()).collect();
        let output = output.split(' ').map(|s| s.to_string()).collect();

        let entry = Entry { signals, output };

        entries.push(entry);
    }
    let mut sum = 0;

    for entry in entries {
        let mut map: HashMap<i32, Vec<char>> = HashMap::new();
        for signal in &entry.signals {
            match signal.len() {
                2 => {
                    map.insert(1, signal.chars().collect());
                }
                3 => {
                    map.insert(7, signal.chars().collect());
                }
                4 => {
                    map.insert(4, signal.chars().collect());
                }
                7 => {
                    map.insert(8, signal.chars().collect());
                }
                _ => continue,
            }
        }
        for signal in &entry.signals {
            match signal.len() {
                5 => {
                    if map.get(&1).unwrap().iter().all(|c| signal.contains(*c)) {
                        // is 3
                        map.insert(3, signal.chars().collect());
                    }
                }
                6 => {
                    if map.get(&4).unwrap().iter().all(|c| signal.contains(*c)) {
                        // is 9
                        map.insert(9, signal.chars().collect());
                    } else if map.get(&1).unwrap().iter().all(|c| signal.contains(*c)) {
                        map.insert(0, signal.chars().collect());
                    } else {
                        map.insert(6, signal.chars().collect());
                    }
                }
                _ => {
                    continue;
                }
            }
        }
        for signal in &entry.signals {
            match signal.len() {
                5 => {
                    // get the char thats in the one signal but not the sixth
                    let c_char = map
                        .get(&1)
                        .unwrap()
                        .iter()
                        .find(|c| !map.get(&6).unwrap().contains(c))
                        .unwrap();

                    if !map.get(&1).unwrap().iter().all(|c| signal.contains(*c)) {
                        if signal.chars().any(|c| c == *c_char) {
                            map.insert(2, signal.chars().collect());
                        } else {
                            map.insert(5, signal.chars().collect());
                        }
                    }
                }

                _ => continue,
            }
        }

        let mut num = String::new();
        for digit in &entry.output {
            let decoded_digit = decode_digit(digit, &map, &entry);

            num.push(decoded_digit);
        }

        let num = num.parse::<i32>().unwrap();
        sum += num;
    }

    sum
}

fn decode_digit(digit: &str, map: &HashMap<i32, Vec<char>>, entry: &Entry) -> char {
    let mut chars = digit.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.dedup();

    // convert chars to string
    let digit = chars.iter().collect::<String>();

    for (key, value) in map {
        // copy value
        let mut value = value.clone();
        value.sort_unstable();

        if digit == value.iter().collect::<String>() {
            return key.to_string().chars().next().unwrap();
        }
    }
    dbg!(&digit);
    dbg!(map);
    dbg!(entry);
    panic!("No match found for {}", digit);
}

fn filter_and_append(mut v: &mut Vec<char>, new: &Vec<char>) {
    if v.is_empty() {
        for c in new {
            v.push(*c);
        }
    } else {
        v.retain(|c| new.contains(c));
    }
}
