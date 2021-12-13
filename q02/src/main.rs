use core::num;
use std::io::BufReader;

fn main() {
    let input = include_str!("../input.txt");
    let ans1 = part1(input);
    println!("Part 1: {}", ans1);
    let ans2 = part2(input);
    println!("Part 2: {}", ans2);
}

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn part1(input: &str) -> usize {
    use Command::*;
    let lines = input.split("\n");

    let mut depth = 0;
    let mut horiz = 0;
    for line in lines {
        let (command_str, num) = line.split_once(" ").unwrap();
        let num: usize = num.parse().unwrap();

        let cmd = match command_str {
            "forward" => Forward(num),
            "down" => Down(num),
            "up" => Up(num),
            s => panic!("Unexpected command {}", s),
        };

        match cmd {
            Forward(n) => horiz += n,
            Down(n) => depth += n,
            Up(n) => depth -= n,
        }
    }

    depth * horiz
}

fn part2(input: &str) -> usize {
    use Command::*;
    let lines = input.split("\n");

    let mut aim = 0;
    let mut horiz = 0;
    let mut depth = 0;
    for line in lines {
        let (command_str, num) = line.split_once(" ").unwrap();
        let num: usize = num.parse().unwrap();

        let cmd = match command_str {
            "forward" => Forward(num),
            "down" => Down(num),
            "up" => Up(num),
            s => panic!("Unexpected command {}", s),
        };

        match cmd {
            Forward(n) => {
                horiz += n;
                depth += aim * n;
            }
            Down(n) => aim += n,
            Up(n) => aim -= n,
        }
    }

    depth * horiz
}
