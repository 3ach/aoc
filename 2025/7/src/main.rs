use adventage::{day, part1demo, part2demo};

day!(2025, 7);
part1demo!(
".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............", 21
);
part2demo!(
".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............", 40
);

type TInput = Vec<Vec<bool>>;

fn parse(map: &str) -> TInput {
    map.lines()
        .map(|line| line.chars()
            .map(|c| c != '.')
            .collect())
        .filter(|r: &Vec<bool>| r.contains(&true))
        .collect()
}

fn part1(input: &TInput) -> usize {
    let mut rays = input[0].clone();
    let mut count = 0;

    for row in &input[1..] {
        for idx in 0..rays.len() {
            if !rays[idx] {
                continue;
            }

            if row[idx] {
                count += 1;
                rays[idx - 1] = true;
                rays[idx + 1] = true;
                rays[idx] = false;
            } else {
                rays[idx] = rays[idx];
            }
        }
    }

    count
}

fn part2(input: &TInput) -> usize {
    let mut rays: Vec<(usize, bool)> = input[0]
        .iter()
        .map(|t| (if *t { 1 } else { 0 }, *t))
        .collect();

    for row in &input[1..] {
        for idx in 0..rays.len() {
            let (universes, ray) = rays[idx];
            if !ray {
                continue;
            }

            if row[idx] {
                rays[idx - 1].0 += universes;
                rays[idx + 1].0 += universes;

                rays[idx - 1].1 = true;
                rays[idx + 1].1 = true;

                rays[idx] = (0, false);
            } else {
                rays[idx] = (universes, ray);
            }
        }
    }

    rays.iter().map(|(universes, _)| universes).sum()
}
