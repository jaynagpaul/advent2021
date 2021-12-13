fn main() {
    let input = include_str!("../input.txt");

    let ans1 = part1(input);
    println!("Part 1: {}", ans1);

    let ans2 = part2(input);
    println!("Part 2: {}", ans2)
}

#[derive(Debug)]
struct Piece {
    val: i32,
    selected: bool,
}

struct Board {
    winning: bool,
    board: Vec<Vec<Piece>>,
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.board {
            for piece in row {
                if piece.selected {
                    write!(f, "*{}*, ", piece.val)?;
                } else {
                    write!(f, "{}, ", piece.val)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Board {
    fn new(lines: &[&str]) -> Self {
        let mut b = Board {
            board: Vec::new(),
            winning: false,
        };

        for line in lines {
            if line.is_empty() {
                continue;
            }
            let row: Vec<Piece> = line
                .split(' ')
                .filter(|&s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .map(|val| Piece {
                    val,
                    selected: false,
                })
                .collect();

            b.board.push(row)
        }

        b
    }

    fn mark_as_visited(&mut self, val: i32) {
        for row in &mut self.board {
            for mut piece in row {
                if piece.val == val {
                    piece.selected = true;
                }
            }
        }
    }

    fn has_won(&mut self) -> bool {
        // horizontal wins
        for row in &self.board {
            let mut all_selected = true;

            for piece in row {
                if !piece.selected {
                    all_selected = false;
                }
            }

            if all_selected {
                self.winning = true;
                return true;
            }
        }

        // vertical wins
        for col in 0..self.board[0].len() {
            let mut all_selected = true;

            for row in &self.board {
                if !row[col].selected {
                    all_selected = false;
                }
            }

            if all_selected {
                self.winning = true;
                return true;
            }
        }

        // diagonal wins
        // let mut all_selected = true;

        // for row in 0..self.board.len() {
        //     if !self.board[row][row].selected {
        //         all_selected = false;
        //     }
        // }

        // if all_selected {
        //     self.winning = true;
        //     return true;
        // }

        // let mut all_selected = true;

        // for row in 0..self.board.len() {
        //     if !self.board[row][self.board.len() - row - 1].selected {
        //         all_selected = false;
        //     }
        // }

        // if all_selected {
        //     self.winning = true;
        //     return true;
        // }

        false
    }

    fn unmarked_sum(&self) -> i32 {
        let mut sum = 0;

        for row in &self.board {
            for piece in row {
                if !piece.selected {
                    sum += piece.val;
                }
            }
        }

        sum
    }
}

struct Boards {
    boards: Vec<Board>,
}

impl Boards {
    fn len(&self) -> usize {
        self.boards.len()
    }

    fn new() -> Self {
        Boards { boards: vec![] }
    }

    fn push(&mut self, b: Board) {
        self.boards.push(b);
    }

    fn mark_as_visited(&mut self, val: i32) {
        for board in &mut self.boards {
            board.mark_as_visited(val);
        }
    }

    fn has_winning_board(&mut self) -> bool {
        let mut t = false;

        for board in &mut self.boards {
            if board.has_won() {
                t = true;
            }
        }

        t
    }

    fn winning_board(self) -> Option<Board> {
        for board in self.boards {
            if board.winning {
                return Some(board);
            }
        }

        None
    }

    fn losing_board(self) -> Option<Board> {
        for board in self.boards {
            if !board.winning {
                return Some(board);
            }
        }

        None
    }

    fn num_winning(&self) -> usize {
        let mut num = 0;

        for board in &self.boards {
            if board.winning {
                num += 1;
            }
        }

        num
    }
}

fn part1(input: &str) -> i32 {
    let called: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let raw_boards = input.lines().skip(2).collect::<Vec<&str>>();

    let mut boards = Boards::new();
    for raw_board in raw_boards.chunks(6) {
        let board = Board::new(raw_board);

        boards.push(board);
    }

    for call in called {
        boards.mark_as_visited(call);

        if boards.has_winning_board() {
            let win = boards.winning_board().unwrap();
            return win.unmarked_sum() * call;
        }
    }

    unreachable!()
}

fn part2(input: &str) -> i32 {
    let called: Vec<i32> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let raw_boards = input.lines().skip(2).collect::<Vec<&str>>();

    let mut boards = Boards::new();
    for raw_board in raw_boards.chunks(6) {
        let board = Board::new(raw_board);

        boards.push(board);
    }

    let mut lose = None;
    dbg!(called.len());
    let mut called_iter = called.into_iter();

    let mut count = 0;
    for call in called_iter.by_ref() {
        boards.mark_as_visited(call);
        boards.has_winning_board();

        if boards.num_winning() == boards.len() - 1 {
            lose = Some(boards.losing_board().unwrap());
            break;
        }

        count += 1;
    }

    let mut lose = lose.unwrap();
    for call in called_iter {
        lose.mark_as_visited(call);

        if lose.has_won() {
            return lose.unmarked_sum() * call;
        }
    }

    unreachable!()
}
