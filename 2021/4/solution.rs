use std::io::BufRead;
use std::io;

#[derive(Debug,Clone)]
struct Board {
    squares: [[i32; 5]; 5],
    col_marks: [i32; 5],
    row_marks: [i32; 5],
    won: bool
}

trait BingoGame {
    fn mark(&mut self, square: i32) -> bool;
    fn score(&self) -> i32; 
}

impl BingoGame for Board {
    fn mark(&mut self, square: i32) -> bool {
        if self.won {
            return false;
        }

        for row in 0..5 {
            for col in 0..5 {
                if self.squares[row][col] == square {
                    self.squares[row][col] *= -1;
                    self.col_marks[col] += 1;
                    self.row_marks[row] += 1;

                    self.won = self.col_marks[col] == 5 || self.row_marks[row] == 5;
                    return self.won;
                }
            }
        }

        return false;
    }

    fn score(&self) -> i32 {
        let mut score = 0;

        for row in 0..5 {
            for col in 0..5 {
                if self.squares[row][col] > 0 {
                    score += self.squares[row][col];
                }
            }
        }

        return score;
    }
}


fn part1(boards: &mut Vec<Board>, calls: &Vec<i32>) -> i32 {
    for call in calls {
        for board in boards.iter_mut() {
            if board.mark(*call) {
                return board.score() * *call;
            }
        }
    }


    panic!("woops")
}

fn part2(boards: &mut Vec<Board>, calls: &Vec<i32>) -> i32 {
    let mut last= 0;
    for call in calls {
        for board in boards.iter_mut() {
            if board.mark(*call) {
                last = board.score() * *call;
            }
        }
    }

    return last;
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut turnbuf = String::new();

    reader.read_line(&mut turnbuf).expect("Could not parse string");
    let calls: Vec<i32> = turnbuf.trim().split(",").map(|call| call.parse::<i32>().unwrap()).collect();
    let mut boards: Vec<Board> = vec![];

    let mut lines = reader.lines();
    lines.next();

    let template =  [[-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1], [-1, -1, -1, -1, -1]];
    let mut board = template.clone();

    let mut row = 0;
    for line in lines {
        let line = line.unwrap();
        if line == "" {
            continue
        }

        for (col, square) in line.trim().split(" ").filter(|sq| sq.len() > 0).enumerate() {
            board[row][col] = square.parse::<i32>().unwrap();
        }

        row += 1;
        if row == 5 {
            row = 0;
            boards.push(Board{squares: board, col_marks: [0, 0, 0, 0, 0], row_marks: [0, 0, 0, 0, 0], won: false});
            board = template.clone();
        }
    }

	let answer1 = part1(&mut boards.clone(), &calls);
	let answer2 = part2(&mut boards.clone(), &calls);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
