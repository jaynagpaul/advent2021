use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let i = part1();
    println!("Part 1: {}", i);
    let i = part2();
    println!("Part 2: {}", i);
}

fn part1() -> i32 {
    let f = File::open("input.txt").expect("input not found");
    let buffer = BufReader::new(f);

    let mut increased_counter = 0;

    let mut last_num = None;
    for line in buffer.lines() {
        let line = line.expect("Could not read line");
        if line.is_empty() {
            break;
        }

        let num: i32 = line.parse().expect("Could not parse line as integer");
        if let Some(last_num) = last_num {
            if num > last_num {
                increased_counter += 1;
            }
        }
        last_num = Some(num)
    }

    increased_counter
}

fn part2() -> i32 {
    let f = File::open("input.txt").expect("input not found");
    let buffer = BufReader::new(f);

    let mut increased_counter = 0;

    let lines = buffer.lines();
    let nums: Vec<i32> = lines.map(|s| s.unwrap().parse().unwrap()).collect();

    let mut sliding_sum = None;

    for i in 0..nums.len() - 2 {
        let new_sum: i32 = nums[i..i + 3].iter().sum();

        if let Some(old_sum) = sliding_sum {
            if new_sum > old_sum {
                increased_counter += 1;
            }
        }

        sliding_sum = Some(new_sum);
    }

    increased_counter
}
