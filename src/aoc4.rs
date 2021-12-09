use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

const INPUT: &str = "src/aoc4.txt";
const BOARD_SIZE: usize = 5;

#[derive(Debug, Clone, Copy)]
struct BoardCell {
    pub marked: bool,
    pub value: i32,
}

impl BoardCell {
    fn new(marked: bool, value: i32) -> Self {
        BoardCell {
            marked: marked,
            value: value,
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    pub winner: bool,
    pub cells: Vec<Vec<BoardCell>>,
}

impl Board {
    fn new() -> Self {
        Board {
            winner: false,
            cells: vec![],
        }
    }

    fn find_and_mark(&mut self, number: i32) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                if cell.value == number {
                    cell.marked = true;
                }
            }
        }
    }

    fn is_winner(&mut self) -> bool {
        // Scan rows
        for row in self.cells.iter_mut() {
            if row.iter().filter(|cell| cell.marked).count() == BOARD_SIZE {
                self.winner = true;
                break;
            }
        }
        // Scan columns
        for j in 0..BOARD_SIZE {
            let mut marked_count = 0;
            for i in 0..BOARD_SIZE {
                if self.cells[i][j].marked {
                    marked_count += 1;
                }
            }
            if marked_count == BOARD_SIZE {
                self.winner = true;
                break;
            }
        }
        self.winner
    }

    fn sum(&self) -> i32 {
        if !self.winner {
            return -1;
        }
        let mut sum: i32 = 0;
        for row in self.cells.iter() {
            sum += row
                .iter()
                .filter(|cell| !cell.marked)
                .map(|cell| cell.value)
                .sum::<i32>();
        }
        sum
    }
}

pub fn aoc4_1() {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    let mut is_first_line = true;
    let mut boards: Vec<Board> = vec![];
    let mut rows: Vec<Vec<BoardCell>> = vec![];
    let mut numbers: VecDeque<i32> = VecDeque::new();
    for line in reader.lines().into_iter() {
        if let Ok(l) = line {
            if is_first_line {
                numbers = l
                    .split(",")
                    .map(|s| if let Ok(n) = s.parse::<i32>() { n } else { -1 })
                    .filter(|n| n != &-1)
                    .collect::<VecDeque<i32>>();
                is_first_line = false;
            } else {
                let line_values = l
                    .split(" ")
                    .filter(|v| v != &"")
                    .map(|v| {
                        if let Ok(n) = v.parse::<i32>() {
                            BoardCell::new(false, n)
                        } else {
                            BoardCell::new(false, 0)
                        }
                    })
                    .collect::<Vec<BoardCell>>();
                if line_values.len() > 0 {
                    rows.push(line_values);
                }
            }
        }
    }

    let mut board = Board::new();

    for row in rows {
        board.cells.push(row);
        if board.cells.len() == BOARD_SIZE {
            boards.push(board.clone());
            board.cells = vec![]
        }
    }

    // START BINGO GAME!
    let mut winner_board: Option<Board> = None;
    let mut bingo_number: i32 = -1;
    if let Some(front) = numbers.pop_front() {
        bingo_number = front;
    }
    while winner_board.is_none() && numbers.len() > 0 {
        for board in boards.iter_mut() {
            board.find_and_mark(bingo_number);
            if board.is_winner() {
                winner_board = Some(board.clone());
            }
        }
        if let Some(n) = numbers.pop_front() {
            if winner_board.is_none() {
                bingo_number = n;
            }
        }
    }
    if let Some(winner) = winner_board {
        println!("AOC 4 Part 1 Result {:?}", bingo_number * winner.sum());
    }
}

fn aoc4_2() {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    let mut is_first_line = true;
    let mut boards: Vec<Board> = vec![];
    let mut rows: Vec<Vec<BoardCell>> = vec![];
    let mut numbers: VecDeque<i32> = VecDeque::new();
    for line in reader.lines().into_iter() {
        if let Ok(l) = line {
            if is_first_line {
                numbers = l
                    .split(",")
                    .map(|s| if let Ok(n) = s.parse::<i32>() { n } else { -1 })
                    .filter(|n| n != &-1)
                    .collect::<VecDeque<i32>>();
                is_first_line = false;
            } else {
                let line_values = l
                    .split(" ")
                    .filter(|v| v != &"")
                    .map(|v| {
                        if let Ok(n) = v.parse::<i32>() {
                            BoardCell::new(false, n)
                        } else {
                            BoardCell::new(false, 0)
                        }
                    })
                    .collect::<Vec<BoardCell>>();
                if line_values.len() > 0 {
                    rows.push(line_values);
                }
            }
        }
    }

    let mut board = Board::new();

    for i in 0..rows.len() {
        board.cells.push(rows[i].clone());
        if board.cells.len() == BOARD_SIZE {
            let mut board_clone = board.clone();
            boards.push(board_clone);
            board.cells = vec![]
        }
    }
    // START BINGO GAME!
    let mut winners: Vec<Board> = vec![];
    let mut bingo_number: i32 = -1;
    let mut last_winner_number: i32 = -1;
    if let Some(front) = numbers.pop_front() {
        bingo_number = front;
    }
    while numbers.len() > 0 && boards.len() > 0 {
        for board in boards.iter_mut() {
            board.find_and_mark(bingo_number);
            if board.is_winner() {
                winners.push(board.clone());
                last_winner_number = bingo_number
            }
        }
        boards = boards.iter().filter(|b| !b.winner).cloned().collect();
        if let Some(n) = numbers.pop_front() {
            bingo_number = n;
        }
    }
    if let Some(last_winner) = winners.pop() {
        println!(
            "AOC 4 Part 2 Result {:?}",
            last_winner_number * last_winner.sum()
        );
    }
}

pub fn aoc4() {
    aoc4_1();
    aoc4_2();
}
