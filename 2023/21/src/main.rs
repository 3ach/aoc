use adventage::{day, part1demo};
use std::collections::VecDeque;

part1demo!(
    "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
    16
);

day!(2023, 21);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Tile {
    Plot,
    Rock,
    Elf,
}

type Point = (usize, usize);

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Rock,
                    '.' => Tile::Plot,
                    'S' => Tile::Elf,
                    _ => panic!(),
                })
                .collect::<Vec<Tile>>()
        })
        .collect()
}

fn start(map: &Vec<Vec<Tile>>) -> Point {
    map.iter()
        .enumerate()
        .find_map(|(row, r)|
                  r.iter()
                  .enumerate()
                  .find_map(move |(col, t)| if *t == Tile::Elf { Some((row, col)) } else { None }))
        .unwrap()
}

fn shortest(map: &Vec<Vec<Tile>>) -> Vec<Vec<Option<usize>>> {
    let start = start(map);

    // Start all the cells with an impossible combo to sentinel unset
    let mut distances = vec![vec![None; map[0].len()]; map.len()];
    let mut unseen = VecDeque::from([(start, 0)]);

    while let Some((point, distance)) = unseen.pop_front() {
        if  distances[point.0][point.1].is_some()
            || map[point.0][point.1] == Tile::Rock {
            continue;
        }

        distances[point.0][point.1] = Some(distance);

        if point.0 > 0 && distances[point.0 - 1][point.1].is_none() {
            unseen.push_back(((point.0 - 1, point.1), distance + 1));
        }

        if point.1 > 0 && distances[point.0][point.1 - 1].is_none() {
            unseen.push_back(((point.0, point.1 - 1), distance + 1));
        }

        if point.0 < map.len() - 1 && distances[point.0 + 1][point.1].is_none() {
            unseen.push_back(((point.0 + 1, point.1), distance + 1));
        }
        
        if point.1 < map[0].len() - 1 && distances[point.0][point.1 + 1].is_none() {
            unseen.push_back(((point.0, point.1 + 1), distance + 1));
        }
    }

    distances
}

fn part1(map: &Vec<Vec<Tile>>) -> usize {
    let steps = if map[0].len() > 20 { 64 } else { 6 };
    let distances = shortest(map);

    distances.iter()
        .map(|row| row.iter().filter_map(|d| *d).filter(|d| *d <= steps && ((*d % 2) == 0)).count())
        .sum()
}

fn part2(map: &Vec<Vec<Tile>>) -> u128 {
    let steps = if map[0].len() > 20 { 26501365 } else { 6 };
    let distances = shortest(map);
    let radius = (map.len() - 1) / 2;
    let tile_radius: u128 = (steps - radius as u128) / map.len() as u128;

    let evens = distances.iter()
        .map(|row| row.iter().filter_map(|d| *d).filter(|d| *d % 2 == 0).count() as u128)
        .sum::<u128>();

    let odds = distances.iter()
        .map(|row| row.iter().filter_map(|d| *d).filter(|d| *d % 2 == 1).count() as u128)
        .sum::<u128>();

    let even_corners = distances.iter()
        .map(|row| row.iter().filter_map(|d| *d).filter(|d| *d > radius && ((*d % 2) == 0)).count() as u128)
        .sum::<u128>();

    let odd_corners = distances.iter()
        .map(|row| row.iter().filter_map(|d| *d).filter(|d| *d > radius && ((*d % 2) == 1)).count() as u128)
        .sum::<u128>();

    if tile_radius % 2 == 0 {
        (evens * tile_radius.pow(2)) + (odds * (tile_radius + 1).pow(2)) + (even_corners * tile_radius) - (odd_corners * (tile_radius + 1))
    } else {
        (evens * (tile_radius + 1).pow(2)) + (odds * tile_radius.pow(2)) - (even_corners * (tile_radius + 1)) + (odd_corners * tile_radius)
    }
}
