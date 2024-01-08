#![feature(isqrt)]
use adventage::{day, part1demo, part2demo};
use std::collections::HashSet;

part1demo!(
    ".#..#
.....
#####
....#
...##",
    8
);

part1demo!(
    "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
    33
);

part1demo!(
    "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.",
    35
);

part1demo!(
    ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..",
    41
);

part1demo!(
    ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
    210
);

part2demo!(
    ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
    802
);

day!(2019, 10);

type Asteroid = (usize, usize);

fn central(asteroids: &HashSet<Asteroid>) -> (Asteroid, usize, Vec<(f32, Asteroid)>) {
    let mut max_visible = 0;
    let mut asteroid = (0, 0);
    let mut all_slopes = vec![];

    for station in asteroids {
        let mut slopes = HashSet::new();
        let mut slope_list = vec![];
        let mut visible = 0;

        for asteroid in asteroids {
            if station == asteroid {
                continue;
            }

            let rise = station.1 as f32 - asteroid.1 as f32;
            let run = station.0 as f32 - asteroid.0 as f32;
            let slope = rise / run;

            let slope = if slope.is_infinite() {
                String::from("Infinity")
            } else {
                format!("{:?}", slope)
            };
            let above =
                asteroid.1 < station.1 || (asteroid.1 == station.1 && asteroid.0 < station.0);

            if slopes.insert((slope.clone(), above)) {
                visible += 1;
            }

            let relative_asteroid = (
                asteroid.0 as f32 - station.0 as f32,
                asteroid.1 as f32 - station.1 as f32,
            );
            let rise = -relative_asteroid.1;
            let run = relative_asteroid.0;

            slope_list.push((rise / run, *asteroid));
        }

        if visible > max_visible {
            max_visible = visible;
            asteroid = *station;
            all_slopes = slope_list.clone();
        }
    }

    (asteroid, max_visible, all_slopes)
}

fn part1(asteroids: &HashSet<Asteroid>) -> usize {
    central(asteroids).1
}

fn part2(asteroids: &HashSet<Asteroid>) -> usize {
    let (station, _, mut slopes) = central(asteroids);
    let mut incinerated: HashSet<Asteroid> = HashSet::new();

    slopes.sort_by(|a, b| b.0.total_cmp(&a.0));
    let mut last_slope = None;
    let mut right = false;

    loop {
        let slope = if last_slope.is_none() {
            slopes[0]
        } else {
            *slopes
                .iter()
                .cycle()
                .skip_while(|s| {
                    s.0 >= last_slope.unwrap() && last_slope.unwrap() != f32::NEG_INFINITY
                })
                .next()
                .unwrap()
        }
        .0;

        if slope == f32::INFINITY {
            right = !right;
        }

        let target = slopes
            .iter()
            .filter(|(other_slope, _)| *other_slope == slope)
            .filter(|(_, asteroid)| !incinerated.contains(asteroid))
            .filter(|(_, asteroid)| if right { asteroid.0 >= station.0 } else { asteroid.0 < station.0 })
            .min_by_key(|(_, asteroid)| {
                (((asteroid.0 as f32 - station.0 as f32).powi(2)
                    + (asteroid.1 as f32 - station.1 as f32).powi(2))
                .sqrt()
                    * 1000.0) as u32
            })
            .cloned();

        if let Some((_, destroyed)) = target {
            incinerated.insert(destroyed);
            if incinerated.len() == 200 {
                return destroyed.0 * 100 + destroyed.1;
            }
        }

        last_slope = Some(slope);
    }
}

fn parse(input: &str) -> HashSet<Asteroid> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, cell)| match cell {
                    '#' => Some((col, row)),
                    _ => None,
                })
                .collect::<HashSet<Asteroid>>()
        })
        .collect()
}
