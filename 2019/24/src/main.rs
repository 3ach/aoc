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
part1demo!(
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
        99
    };

    0
}
