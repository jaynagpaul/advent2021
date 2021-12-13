fn main() {
    let input = include_str!("../input.txt");

    let ans1 = part1(input);
    println!("Part 1: {}", ans1);

    let ans2 = part2(input);
    println!("Part 2: {}", ans2);
}

struct Octo {
    flashed: bool,
    val: i32,
}

impl Octo {
    fn new(val: i32) -> Octo {
        Octo {
            flashed: false,
            val,
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut octos: Vec<Vec<Octo>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .map(Octo::new)
                .collect()
        })
        .collect();
    let mut count = 0;
    for i in 0..100 {
        count += simulate(&mut octos);
    }

    count
}

fn part2(input: &str) -> i32 {
    let mut octos: Vec<Vec<Octo>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .map(Octo::new)
                .collect()
        })
        .collect();

    let mut i = 1;
    let max = octos.len() * octos[0].len();
    loop {
        let count = simulate(&mut octos);
        if count == max.try_into().unwrap() {
            break;
        }
        i += 1;
    }

    i
}
fn increment(octos: &mut Vec<Vec<Octo>>) {
    octos.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|o| o.val += 1);
    });
}

fn flash(octos: &mut Vec<Vec<Octo>>, row: usize, col: usize) {
    let octo = &mut octos[row][col];

    if octo.val > 9 && !octo.flashed {
        octo.flashed = true;
        increment_surrounding(octos, row, col)
    }
}

fn increment_surrounding(octos: &mut Vec<Vec<Octo>>, row: usize, col: usize) {
    // increment octo above
    if row > 0 {
        octos[row - 1][col].val += 1;
        flash(octos, row - 1, col);
    }
    // increment octo below
    if row < octos.len() - 1 {
        octos[row + 1][col].val += 1;
        flash(octos, row + 1, col);
    }
    // increment octo left
    if col > 0 {
        octos[row][col - 1].val += 1;
        flash(octos, row, col - 1);
    }
    // increment octo right
    if col < octos[0].len() - 1 {
        octos[row][col + 1].val += 1;
        flash(octos, row, col + 1);
    }
    // increment octo above left
    if row > 0 && col > 0 {
        octos[row - 1][col - 1].val += 1;
        flash(octos, row - 1, col - 1);
    }
    // increment octo above right
    if row > 0 && col < octos[0].len() - 1 {
        octos[row - 1][col + 1].val += 1;
        flash(octos, row - 1, col + 1);
    }
    // increment octo below left
    if row < octos.len() - 1 && col > 0 {
        octos[row + 1][col - 1].val += 1;
        flash(octos, row + 1, col - 1);
    }
    // increment octo below right
    if row < octos.len() - 1 && col < octos[0].len() - 1 {
        octos[row + 1][col + 1].val += 1;
        flash(octos, row + 1, col + 1);
    }
}

fn simulate(octos: &mut Vec<Vec<Octo>>) -> i32 {
    increment(octos);

    for row in 0..octos.len() {
        for col in 0..octos[0].len() {
            let mut octo = &mut octos[row][col];

            if octo.val > 9 {
                flash(octos, row, col);
            }
        }
    }

    let mut count = 0;
    for row in octos {
        for octo in row {
            if octo.val > 9 {
                octo.val = 0;
                octo.flashed = false;
                count += 1;
            }
        }
    }

    count
}
