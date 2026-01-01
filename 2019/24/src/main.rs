use adventage::{day, part1demo, part2demo};
use std::collections::HashSet;

day!(2019, 24);
part1demo!(
    "....#
#..#.
#..##
..#..
#....",
    2129920
);
part2demo!(
    "....#
#..#.
#..##
..#..
#....",
    99
);

type TInput = Vec<Vec<bool>>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect()
}

fn recursive_generation(map: &Vec<TInput>) -> Vec<TInput> {
    let mut neighbors = vec![vec![vec![0; 5]; 5]; map.len() + 2];
    for board in 0..(map.len()) {
        let neighbor_board = board + 1;
        for row in 0..5 {
            for col in 0..5 {
                if !map[board][row][col] {
                    continue;
                }

                // Upper neighbor
                if row == 0 {
                    neighbors[neighbor_board - 1][1][2] += 1;
                } else if row == 3 && col == 2 {
                    for neighbor in &mut neighbors[neighbor_board + 1][4] {
                        *neighbor += 1
                    }
                } else {
                    neighbors[neighbor_board][row - 1][col] += 1
                }

                // Lower neighbor
                if row == 4 {
                    neighbors[neighbor_board - 1][3][2] += 1;
                } else if row == 1 && col == 2 {
                    for neighbor in &mut neighbors[neighbor_board + 1][0] {
                        *neighbor += 1
                    }
                } else {
                    neighbors[neighbor_board][row + 1][col] += 1
                }

                // Left neighbor
                if col == 0 {
                    neighbors[neighbor_board - 1][2][1] += 1;
                } else if col == 3 && row == 2 {
                    for nr in 0..5 {
                        neighbors[neighbor_board + 1][nr][4] += 1;
                    }
                } else {
                    neighbors[neighbor_board][row][col - 1] += 1
                }

                // Right neighbor
                if col == 4 {
                    neighbors[neighbor_board - 1][2][3] += 1;
                } else if row == 2 && col == 1 {
                    for nr in 0..5 {
                        neighbors[neighbor_board + 1][nr][0] += 1;
                    }
                } else {
                    neighbors[neighbor_board][row][col + 1] += 1
                }
            }
        }
    }

    let mut next_bugs = vec![vec![vec![false; 5]; 5]; map.len() + 2];

    let mut first = false;
    let mut last = false;

    for board in 0..next_bugs.len() {
        let mut any = false;
        for row in 0..5 {
            for col in 0..5 {
                if neighbors[board][row][col] == 1 {
                    any = true;
                    next_bugs[board][row][col] = true;
                }

                if neighbors[board][row][col] == 2
                    && (board == 0 
                    || board == next_bugs.len() - 1 
                    || !map[board - 1][row][col]) {
                    any = true;
                    next_bugs[board][row][col] = true;
                }
            }
        }
        if any && board == 0 {
            first = true;
        } 

        if any && board == next_bugs.len() - 1 {
            last = true;
        }
    }

    if !last {
        next_bugs.remove(next_bugs.len() - 1);
    } 

    if !first {
        next_bugs.remove(0);
    }

    next_bugs
}

fn generation(map: &TInput) -> TInput {
    let mut next = vec![vec![false; 5]; 5];
    for row in 0..5 {
        for col in 0..5 {
            let mut neighbors = 0;

            if row > 0 && map[row - 1][col] {
                neighbors += 1;
            }

            if row < 4 && map[row + 1][col] {
                neighbors += 1;
            }

            if col > 0 && map[row][col - 1] {
                neighbors += 1;
            } 

            if col < 4 && map[row][col + 1] {
                neighbors += 1;
            }

            if neighbors == 1 {
                next[row][col] = true;
            }

            if neighbors == 2 && !map[row][col] {
                next[row][col] = true;
            }
        }
    }

    next
}

fn score(map: &TInput) -> u32 {
    (0..5)
        .into_iter()
        .map(|row|
            (0..5).into_iter().map(move |col| 
                if map[row][col] { 
                    2_u32.pow((row as u32 * 5) + col as u32)
                } else {
                    0
                }).sum::<u32>()
        ).sum()
}

fn count(map: &TInput) -> u32 {
    (0..5)
        .into_iter()
        .map(|row|
            (0..5).into_iter().map(move |col| 
                if map[row][col] { 
                    1
                } else {
                    0
                }).sum::<u32>()
        ).sum()
}

fn part1(map: &TInput) -> u32 {
    let mut seen = HashSet::new();
    let mut map = map.clone();
    loop {
        let diversity = score(&map);
        if !seen.insert(diversity) {
            return diversity;
        }

        map = generation(&map);
    }
}

fn part2(map: &TInput) -> u32 {
    let generations = if count(map) == 8 {
        10
    } else {
        200
    };


    let mut boards = vec![map.clone()];
    for _ in 0..generations {
        boards = recursive_generation(&boards);
    }

    let mut sum = 0;
    for board in boards {
        for row in board {
            for cell in row {
                if cell {
                    sum += 1;
                }
            }
        }
    }
    sum
}
