use std::io::BufRead;
use std::io;
use std::cmp;

#[derive(Debug,Clone,Copy)]
enum Position {
    Empty,
    Occupied,
    Floor
}

fn adjacent_occupied(layout: &Vec<Vec<Position>>, point: (usize, usize)) -> u8 {
    let (row, col) = point;

    let row_start = row.checked_sub(1).unwrap_or(0);
    let row_end = cmp::min(row + 2, layout.len());
    let col_start = col.checked_sub(1).unwrap_or(0);
    let col_end = cmp::min(col + 2, layout[0].len());

    let mut occupied = 0;
    for r in row_start..row_end {
        for c in col_start..col_end {
            if (r, c) == (row, col) {
                continue
            }

            match layout[r][c] {
                Position::Occupied => occupied += 1,
                _ => {},
            }
        }
    }

    return occupied;
}

fn visible_occupied(layout: &Vec<Vec<Position>>, point: (usize, usize)) -> u8 {
    let (row, col) = point;

    let mut occupied = 0;
    for r in (-1 as isize)..1 {
        for c in (-1 as isize)..1 {
            println!("Checking [{}][{}] at vector [{}][{}]", row, col, r, c);
            if (r, c) == (0, 0) {
                continue
            }

            'dir: loop {
                let mut scalar = 1;
                
                let r_scaled = row as isize + (r * scalar);
                let c_scaled = col as isize + (c * scalar);

                if r_scaled < 0 || r_scaled >= layout.len() as isize {
                    break
                }

                if c_scaled < 0 || c_scaled >= layout[0].len() as isize {
                    break
                }

                let r_scaled: usize = r_scaled as usize;
                let c_scaled: usize = c_scaled as usize;

                println!("Scalar {}, [{}][{}]", scalar, r_scaled, c_scaled);
                
                match layout[r_scaled][c_scaled] {
                    Position::Occupied => {
                        occupied += 1;
                        break;
                    }
                    Position::Empty => {
                        break;
                    },
                    _ => scalar += 1,
                }
            }
        }
    }

    return occupied;
}

fn part1(layout: &Vec<Vec<Position>>) -> u16 {
    let mut layout = layout.clone();

    loop {
        let mut changes: Vec<(usize, usize)> = vec![];
        for row in 0..layout.len() {
            for col in 0..layout[row].len() {
                let seat = layout[row][col];
                let adjacent = adjacent_occupied(&layout, (row, col));

                match seat {
                    Position::Empty if adjacent == 0 => changes.push((row, col)),
                    Position::Occupied if adjacent >= 4 => changes.push((row, col)),
                    _ => continue,
                }
            }
        }

        if changes.len() == 0 {
            break
        }

        for change in changes {
            let (row, col) = change;
            layout[row][col] = match layout[row][col] {
                Position::Empty => Position::Occupied,
                Position::Occupied => Position::Empty,
                _ => panic!(),
            }
        }
    }

    return layout.iter().map(|row| {
        row.iter().map(|position| {
            match position {
                Position::Occupied => 1,
                _ => 0
            }
        }).sum::<u16>()
    }).sum();
}

fn part2(layout: &Vec<Vec<Position>>) -> u16 {
    let mut layout = layout.clone();

    loop {
        let mut changes: Vec<(usize, usize)> = vec![];
        for row in 0..layout.len() {
            for col in 0..layout[row].len() {
                let seat = layout[row][col];
                let adjacent = visible_occupied(&layout, (row, col));

                match seat {
                    Position::Empty if adjacent == 0 => changes.push((row, col)),
                    Position::Occupied if adjacent >= 4 => changes.push((row, col)),
                    _ => continue,
                }
            }
        }

        if changes.len() == 0 {
            break
        }

        for change in changes {
            let (row, col) = change;
            layout[row][col] = match layout[row][col] {
                Position::Empty => Position::Occupied,
                Position::Occupied => Position::Empty,
                _ => panic!(),
            }
        }
    }

    return layout.iter().map(|row| {
        row.iter().map(|position| {
            match position {
                Position::Occupied => 1,
                _ => 0
            }
        }).sum::<u16>()
    }).sum();
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

	let layout: Vec<Vec<Position>> = reader.lines().map(|line| {
        line.unwrap().chars().map(|position| {
            match position {
                '.' => Position::Floor,
                'L' => Position::Empty,
                '#' => Position::Occupied,
                _ => panic!("uknown input char {}", position),
            }
        }).collect()
	}).collect();

	let answer1 = part1(&layout);
	let answer2 = part2(&layout);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
