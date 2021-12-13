fn main() {
    let input = include_str!("../input.txt");

    let ans1 = part1(input);
    println!("Part 1: {}", ans1);

    let ans2 = part2(input);
    println!("Part 2: {}", ans2);
}

fn part1(input: &str) -> i64 {
    let mut score = 0;
    for line in input.lines() {
        let mut stack = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {}
            }

            match c {
                ')' | ']' | '}' | '>' => {
                    let top = stack.pop().unwrap();
                    if top != '(' && c == ')' {
                        score += 3;
                        break;
                    }
                    if top != '[' && c == ']' {
                        score += 57;
                        break;
                    }
                    if top != '{' && c == '}' {
                        score += 1197;
                        break;
                    }
                    if top != '<' && c == '>' {
                        score += 25137;
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    score
}

fn part2(input: &str) -> i64 {
    let mut scores = vec![];
    let lines = input.lines().filter(|line| {
        let mut stack = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {}
            }

            match c {
                ')' | ']' | '}' | '>' => {
                    let top = stack.pop().unwrap();
                    if top != '(' && c == ')'
                        || top != '[' && c == ']'
                        || top != '{' && c == '}'
                        || top != '<' && c == '>'
                    {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    });

    for line in lines {
        let mut stack = vec![];

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    stack.pop();
                }
                _ => unreachable!(),
            }
        }

        dbg!(&stack);
        let mut score = 0;
        for c in stack.iter().rev() {
            score *= 5;
            match c {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,
                _ => unreachable!(),
            }
        }

        scores.push(score);
    }

    scores.sort_unstable();

    dbg!(&scores);

    // return middle score
    scores[scores.len() / 2]
}
