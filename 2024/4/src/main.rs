use adventage::{day, part1demo, part2demo};
use std::collections::HashMap;

day!(2024, 4);
part1demo!(
    "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    18
);
part2demo!(
    "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    9
);

type Point = (isize, isize);
type TInput = HashMap<Point, char>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(move |(col, c)| ((col as isize, row as isize), c))
        })
        .flatten()
        .collect()
}

fn end_words(pos: Point, len: isize) -> Vec<Vec<Point>> {
    vec![
        (0..len).map(|d| (pos.0 + d, pos.1)).collect(),
        (0..len).map(|d| (pos.0 - d, pos.1)).collect(),
        (0..len).map(|d| (pos.0, pos.1 + d)).collect(),
        (0..len).map(|d| (pos.0, pos.1 - d)).collect(),
        (0..len).map(|d| (pos.0 + d, pos.1 + d)).collect(),
        (0..len).map(|d| (pos.0 + d, pos.1 - d)).collect(),
        (0..len).map(|d| (pos.0 - d, pos.1 - d)).collect(),
        (0..len).map(|d| (pos.0 - d, pos.1 + d)).collect(),
    ]
}

fn center_words(pos: Point, len: isize) -> Vec<Vec<Point>> {
    vec![
        (-len..=len).map(|d| (pos.0 + d, pos.1 + d)).collect(),
        (-len..=len).map(|d| (pos.0 + d, pos.1 + d)).rev().collect(),
        (-len..=len).map(|d| (pos.0 - d, pos.1 + d)).collect(),
        (-len..=len).map(|d| (pos.0 - d, pos.1 + d)).rev().collect(),
    ]
}

fn part1(wordsearch: &TInput) -> usize {
    wordsearch
        .iter()
        .filter_map(|(pos, c)| if *c == 'X' { Some(pos) } else { None })
        .map(|candidate| end_words(*candidate, 4))
        .map(|words| {
            words
                .iter()
                .map(|word| {
                    word.iter()
                        .map(|pos| wordsearch.get(pos).unwrap_or(&' '))
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
        })
        .flatten()
        .filter(|s| s == "XMAS")
        .count()
}

fn part2(wordsearch: &TInput) -> usize {
    wordsearch
        .iter()
        .filter_map(|(pos, c)| if *c == 'A' { Some(pos) } else { None })
        .map(|candidate| center_words(*candidate, 1))
        .map(|words| {
            words
                .iter()
                .map(|word| {
                    word.iter()
                        .map(|pos| wordsearch.get(pos).unwrap_or(&' '))
                        .collect::<String>()
                })
                .all(|s| s == "SAM" || s == "MAS")
        })
        .filter(|s| *s)
        .count()
}
