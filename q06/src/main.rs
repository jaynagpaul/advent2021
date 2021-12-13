#[derive(Debug)]
struct Fish(usize);

impl Fish {
    fn simulate(state: &mut Vec<Fish>) {
        for i in 0..state.len() {
            if state[i].0 == 0 {
                state[i].0 = 6;
                state.push(Fish(8));
            } else {
                state[i].0 -= 1;
            }
        }
    }
}

struct Simulation {
    state: [usize; 9],
}

impl Simulation {
    fn new(initial: Vec<Fish>) -> Self {
        let mut state = [0; 9];

        for fish in initial {
            state[fish.0] += 1;
        }

        Simulation { state }
    }

    fn get_num_fish_with_timer(&self, timer: usize) -> usize {
        self.state[timer]
    }

    fn set_num_fish_with_timer(&mut self, timer: usize, to: usize) {
        self.state[timer] = to;
    }

    fn simulate(&mut self) {
        let seven_fish = self.get_num_fish_with_timer(8);
        let mut six_fish = self.get_num_fish_with_timer(7);
        let five_fish = self.get_num_fish_with_timer(6);
        let four_fish = self.get_num_fish_with_timer(5);
        let three_fish = self.get_num_fish_with_timer(4);
        let two_fish = self.get_num_fish_with_timer(3);
        let one_fish = self.get_num_fish_with_timer(2);
        let zero_fish = self.get_num_fish_with_timer(1);
        let eight_fish = self.get_num_fish_with_timer(0);
        six_fish += eight_fish;

        self.set_num_fish_with_timer(8, eight_fish);
        self.set_num_fish_with_timer(7, seven_fish);
        self.set_num_fish_with_timer(6, six_fish);
        self.set_num_fish_with_timer(5, five_fish);
        self.set_num_fish_with_timer(4, four_fish);
        self.set_num_fish_with_timer(3, three_fish);
        self.set_num_fish_with_timer(2, two_fish);
        self.set_num_fish_with_timer(1, one_fish);
        self.set_num_fish_with_timer(0, zero_fish);
    }

    fn len(&self) -> usize {
        self.state.iter().sum()
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let ans1 = part1(input);
    println!("Part 1: {}", ans1);
    let ans2 = part2(input);
    println!("Part 2: {}", ans2);
}

fn part1(input: &str) -> usize {
    let mut initial: Vec<Fish> = input
        .split(',')
        .map(|s| s.to_string().parse().unwrap())
        .map(Fish)
        .collect();

    for _ in 0..80 {
        Fish::simulate(&mut initial);
    }

    initial.len()
}

fn part2(input: &str) -> usize {
    let initial: Vec<Fish> = input
        .split(',')
        .map(|s| s.to_string().parse().unwrap())
        .map(Fish)
        .collect();

    let mut simulation = Simulation::new(initial);
    for _ in 0..256 {
        simulation.simulate();
    }

    simulation.len()
}
