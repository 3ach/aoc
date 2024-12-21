use adventage::{day, part1demo};
use std::collections::HashMap;

day!(2024, 21);
part1demo!(
    "029A
980A
179A
456A
379A
",
    126384
);

type TInput = Vec<String>;
type Coordinate = (usize, usize);

fn parse(input: &str) -> TInput {
    input.lines().map(|l| String::from(l)).collect()
}

fn number_coordinate(key: char) -> Coordinate {
    match key {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!("{} is not a direction key", key),
    }
}

fn move_between_number_keys(start: &Coordinate, end: &Coordinate) -> Vec<char> {
    let mut moves = vec![];
    for _ in end.0..start.0 {
        moves.push('<');
    }

    for _ in start.1..end.1 {
        moves.push('v');
    }

    for _ in end.1..start.1 {
        moves.push('^');
    }

    for _ in start.0..end.0 {
        moves.push('>');
    }

    if (start.0 == 0 && end.1 == 3) || (end.0 == 0 && start.1 == 3) {
        moves.reverse();
    }

    moves.push('A');
    moves
}

fn move_between_direction_keys(start: &Coordinate, end: &Coordinate) -> Vec<char> {
    let mut moves = vec![];
    for _ in end.0..start.0 {
        moves.push('<');
    }

    for _ in start.1..end.1 {
        moves.push('v');
    }

    for _ in end.1..start.1 {
        moves.push('^');
    }

    for _ in start.0..end.0 {
        moves.push('>');
    }

    if start.0 == 0 || end.0 == 0 {
        moves.reverse()
    }

    moves.push('A');
    moves
}

fn direction_coordinate(key: char) -> Coordinate {
    match key {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("{} is not a direction key", key),
    }
}

fn code_to_directions(code: &str) -> Vec<String> {
    code.chars()
        .map(number_coordinate)
        .scan(number_coordinate('A'), |start, next| {
            let moves = move_between_number_keys(start, &next);
            *start = next;
            Some(moves)
        })
        .map(|s| s.iter().collect())
        .collect()
}

fn directions_to_directions(directions: &str) -> Vec<String> {
    directions
        .chars()
        .map(direction_coordinate)
        .scan(direction_coordinate('A'), |start, next| {
            let moves = move_between_direction_keys(start, &next);
            *start = next;
            Some(moves)
        })
        .map(|sequence| sequence.iter().collect())
        .collect()
}

fn expand(code: &str, steps: usize) -> usize {
    let directions = code_to_directions(code);

    let mut to_expand = directions.clone();
    let mut expansions: HashMap<String, Vec<String>> = HashMap::new();

    while let Some(sequence) = to_expand.pop() {
        if expansions.contains_key(&sequence) {
            continue;
        }

        let subsequences = directions_to_directions(&sequence);

        for subsequence in &subsequences {
            if !expansions.contains_key(subsequence) {
                to_expand.push(subsequence.clone());
            }
        }

        expansions.insert(sequence.clone(), subsequences.clone());
    }

    let mut counts: HashMap<String, usize> = HashMap::new();
    for step in &directions {
        *counts.entry(step.clone()).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut next = HashMap::new();

        for (sequence, count) in &counts {
            let children = expansions.get(sequence).unwrap();
            for child in children {
                *next.entry(child.clone()).or_insert(0) += count;
            }
        }

        counts = next;
    }

    counts.iter().map(|(s, c)| s.len() * *c).sum()
}

fn part1(codes: &TInput) -> usize {
    codes
        .iter()
        .map(|code| expand(code, 2) * code[0..3].parse::<usize>().unwrap())
        .sum()
}

fn part2(codes: &TInput) -> usize {
    codes
        .iter()
        .map(|code| expand(code, 25) * code[0..3].parse::<usize>().unwrap())
        .sum()
}
