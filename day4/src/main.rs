use common;
use std::str::FromStr;
use std::fmt;

struct BingoCell {
    value: u8,
    called: bool,
}

impl fmt::Display for BingoCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>2}{} ", self.value, if self.called { '*' } else { ' ' })
    }
}

struct BingoBoard {
    cells: Vec<BingoCell>,
    num_rows: usize,
    num_cols: usize,
    solved: bool,
}

impl BingoBoard {
    fn row(&self, row_num: usize) -> &[BingoCell] {
        &self.cells[row_num * self.num_cols..(row_num * self.num_cols) + self.num_cols]
    }
    fn col(&self, col_num: usize) -> Vec<&BingoCell> {
        let mut ret_cells: Vec<&BingoCell> = Vec::new();
        for cell in self.cells.iter().skip(col_num).step_by(self.num_cols) {
            ret_cells.push(cell);
        }
        ret_cells
    }
    fn mark_called(&mut self, value: u8) -> bool {
        for mut cell in &mut self.cells {
            if cell.value == value {
                cell.called = true;
                return true;
            }
        }
        return false;
    }
    fn calc_final_score(&self, final_num: u8) -> u32 {
        let board_sum: u32 = self.cells.iter().fold(
            0,
            |sum, cell| if cell.called { sum } else { sum + cell.value as u32 },
        );
        board_sum * final_num as u32
    }
}

impl FromStr for BingoBoard {
    type Err = ();

    fn from_str(input: &str) -> Result<BingoBoard, Self::Err> {
        /* Reads in a string that looks like a grid of numbers, rows ending in '\n' and columns spaced with ' '
            1 2  3  4
            5 6  7  8
            9 10 11 12
        */
        let mut num_rows = 0;
        let mut num_cols = 0;
        let mut cells: Vec<BingoCell> = Vec::new();
        for row in input.split('\n') {
            cells.extend(row.split_whitespace().map(|v| BingoCell {
                value: v.parse::<u8>().expect("I failed parsing!"),
                called: false,
            }));
            if num_cols == 0 {
                num_cols = cells.len();
            }
            num_rows += 1;
        }
        let bb = BingoBoard {
            cells: cells,
            num_rows: num_rows,
            num_cols: num_cols,
            solved: false,
        };
        Ok(bb)
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = String::new();

        for row_ndx in 0..self.num_rows {
            let row_str: String = self.row(row_ndx).iter().map(ToString::to_string).collect();
            rows.push_str(&row_str);
            rows.push_str("\n");
        }

        write!(f, "{}", rows)
    }
}

fn check_board_is_solved(board: &BingoBoard) -> bool {
    for row_ndx in 0..board.num_rows {
        if board.row(row_ndx).iter().all(|c| c.called) {
            return true;
        }
    }
    for col_ndx in 0..board.num_cols {
        if board.col(col_ndx).iter().all(|c| c.called) {
            return true;
        }
    }
    return false;
}

fn part1(boards: &mut Vec<BingoBoard>, bingo_nums: &[u8]) {
    for bingo_num in bingo_nums {
        for board in boards.iter_mut() {
            if board.mark_called(*bingo_num) && check_board_is_solved(&board) {
                println!("We have a winner! Final Number: {}  Score: {}", bingo_num, board.calc_final_score(*bingo_num));
                println!("{}", board);
                return;
            }
        }
    }
}

fn part2(boards: &mut Vec<BingoBoard>, bingo_nums: &[u8]) {
    let total_boards = boards.len();
    let mut solved_boards = 0;
    for bingo_num in bingo_nums {
        for board in boards.iter_mut().filter(|board| !board.solved) {
            if board.mark_called(*bingo_num) && check_board_is_solved(&board) {
                board.solved = true;
                solved_boards += 1;
                if solved_boards == total_boards {
                    println!("We have a (last) winner! Final Number: {}  Score: {}", bingo_num, board.calc_final_score(*bingo_num));
                    println!("{}", board);
                }
            }
        }
    }
}

fn main() {
    let data = common::read_input("data/input-day4.txt");
    let bingo_numbers: Vec<u8> = data
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    let mut boards: Vec<BingoBoard> = Vec::new();
    for board_str in data.split("\n\n").skip(1) {
        boards.push(BingoBoard::from_str(board_str).expect("Well that didn't work..."));
    }

    println!("Bingo Numbers:\n{:?}", bingo_numbers);
    println!("I found this many boards: {}", boards.len());

    part1(&mut boards, &bingo_numbers);
    part2(&mut boards, &bingo_numbers);
}
