use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let ans1 = part1(input);
    println!("Part 1: {}", ans1);

    let ans2 = part2(input);
    println!("Part 2: {}", ans2);
}

#[derive(Hash, Clone, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn parse(s: &str) -> Self {
        let (x, y) = s.split_once(",").unwrap();

        let x = x.parse().unwrap();
        let y = y.parse().unwrap();

        Point(x, y)
    }
}

struct PointsIterator(Line, Option<Point>);

impl Iterator for PointsIterator {
    type Item = Point;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = &self.1 {
            let next = self.0.next(&cur);
            self.1 = next.clone();
            next
        } else {
            self.1 = Some(self.0.start.clone());
            Some(self.0.start.clone())
        }
    }
}

#[derive(Hash, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn parse(s: &str) -> Self {
        let (start, end) = s.split_once(" -> ").unwrap();

        let start = Point::parse(start);
        let end = Point::parse(end);

        Line { start, end }
    }

    fn next(&self, cur: &Point) -> Option<Point> {
        if cur == &self.end {
            None
        } else {
            Some(Point(cur.0 + self.dx(), cur.1 + self.dy()))
        }
    }

    fn horizontal_or_vertical(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn dx(&self) -> i32 {
        // end x > start x
        if self.end.0 > self.start.0 {
            1
        } else if self.end.0 < self.start.0 {
            -1
        } else {
            0
        }
    }

    fn dy(&self) -> i32 {
        // end x > start x
        match self.end.1.cmp(&self.start.1) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        }
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        PointsIterator(self.clone(), None)
    }

    fn increment_points(&self, map: &mut HashMap<Point, usize>) {
        for point in self.points() {
            let num = map.entry(point).or_insert(0);
            *num += 1;
        }
    }
}

fn part1(input: &str) -> i32 {
    let lines: Vec<Line> = input.lines().map(Line::parse).collect();

    let mut map = HashMap::new();

    lines
        .iter()
        .filter(|l| l.horizontal_or_vertical())
        .for_each(|line| line.increment_points(&mut map));

    let mut count = 0;
    for val in map.values() {
        if val >= &2 {
            count += 1;
        }
    }

    count
}

fn part2(input: &str) -> i32 {
    let lines: Vec<Line> = input.lines().map(Line::parse).collect();

    let mut map = HashMap::new();

    lines
        .iter()
        .for_each(|line| line.increment_points(&mut map));

    let mut count = 0;
    for val in map.values() {
        if val >= &2 {
            count += 1;
        }
    }

    count
}
