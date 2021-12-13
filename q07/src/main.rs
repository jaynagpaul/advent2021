fn main() {
    let input = include_str!("../input.txt");

    let ans1 = part1(input);
    println!("Part 1: {}", ans1);

    let ans2 = part2(input);
    println!("Part 2: {}", ans2);
}

fn part1(input: &str) -> i32 {
    let mut poss: Vec<i32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    poss.sort_unstable();

    let mid = poss.len() / 2;

    let most_eff = poss[mid];

    poss.iter().map(|x| (x - most_eff).abs()).sum()
}

fn part2(input: &str) -> i32 {
    let mut poss: Vec<i32> = input.split(',').map(|s| s.parse().unwrap()).collect();
    poss.sort_unstable();

    let sum = poss.iter().sum::<i32>() as f64;
    let fuel = sum / poss.len() as f64;

    // round fuel
    let fuel = (fuel + 0.5).floor() as i32;

    let mut totals = vec![];

    for fuel in poss[0]..poss[poss.len() - 1] {
        let mut total = 0;
        for x in poss.iter() {
            let diff = (x - fuel).abs();

            total += calc_fuel(diff);
        }

        totals.push(total);
    }

    *totals.iter().min().unwrap()
}

fn calc_fuel(n: i32) -> i32 {
    (n) * (n + 1) / 2
}
