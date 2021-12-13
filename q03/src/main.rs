use std::{char, panic};

fn main() {
    let input = include_str!("../input.txt");
    // let ans1 = part1(input);
    // println!("Part 1: {}", ans1);

    let ans2 = part2(input);
    println!("Part 2: {}", ans2);
}

fn part1(input: &str) -> i32 {
    let lines: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();
    let mut one_bits = vec![];

    for i in 0..12 {
        one_bits.push(0);
    }

    for line in lines {
        let chars: Vec<_> = line.chars().collect();
        one_bits[0] += chars[0].to_digit(10).unwrap();
        one_bits[1] += chars[1].to_digit(10).unwrap();
        one_bits[2] += chars[2].to_digit(10).unwrap();
        one_bits[3] += chars[3].to_digit(10).unwrap();
        one_bits[4] += chars[4].to_digit(10).unwrap();
        one_bits[5] += chars[5].to_digit(10).unwrap();
        one_bits[6] += chars[6].to_digit(10).unwrap();
        one_bits[7] += chars[7].to_digit(10).unwrap();
        one_bits[8] += chars[8].to_digit(10).unwrap();
        one_bits[9] += chars[9].to_digit(10).unwrap();
        one_bits[10] += chars[10].to_digit(10).unwrap();
        one_bits[11] += chars[11].to_digit(10).unwrap();
    }
    dbg!(&one_bits);
    let mut s = "".to_string();

    for b in one_bits {
        if b > 500 {
            s.push('1');
        } else if b < 500 {
            s.push('0');
        } else {
            panic!("HEY EVEN")
        }
    }

    let mut epsilon = "".to_string();
    let mut gamma = "".to_string();
    for c in s.chars() {
        epsilon.push(c);
        if c == '0' {
            gamma.push('1');
        } else {
            gamma.push('0');
        }
    }

    dbg!(&epsilon);
    dbg!(&gamma);

    let a = i32::from_str_radix(&epsilon, 2).unwrap();
    let b = i32::from_str_radix(&gamma, 2).unwrap();

    a * b
}

fn part2(input: &str) -> i32 {
    let mut lines: Vec<String> = input.split('\n').map(|x| x.to_string()).collect();
    let mut lines_copy = lines.clone();
    let len = lines[0].len();

    for i in 0..len {
        let c = get_most_common(&lines, i);

        lines = lines
            .into_iter()
            .filter(|s| s.chars().nth(i).unwrap() == c)
            .collect();
        dbg!(&lines);

        if lines.len() == 1 {
            break;
        }
    }

    let oxy = i32::from_str_radix(&lines[0], 2).unwrap();

    dbg!(&oxy);
    for i in 0..len {
        let c = get_least_common(&lines_copy, i);

        lines_copy = lines_copy
            .into_iter()
            .filter(|s| s.chars().nth(i).unwrap() == c)
            .collect();

        if lines_copy.len() == 1 {
            break;
        }
    }

    let co2 = i32::from_str_radix(&lines_copy[0], 2).unwrap();

    oxy * co2
}

fn get_least_common(lines: &[String], idx: usize) -> char {
    let len = lines.len();
    let mut count = 0;

    for l in lines {
        let c = l.chars().nth(idx).unwrap();
        if c == '1' {
            count += 1;
        }
    }

    if 2 * count >= len {
        '0'
    } else {
        '1'
    }
}
fn get_most_common(lines: &[String], idx: usize) -> char {
    dbg!(idx);
    let len = lines.len();
    let mut count = 0;

    for l in lines {
        let c = l.chars().nth(idx).unwrap();
        if dbg!(c) == '1' {
            count += 1;
        }
    }
    if 2 * count >= len {
        '1'
    } else {
        '0'
    }
}
