use std::collections::HashSet;
use std::io;
use std::io::BufRead;

type Asteroid = (usize, usize);
type Line = ((i32, i32), bool);

fn part1(asteroids: &HashSet<Asteroid>) -> usize {
    let mut max_visible = 0;

    for station in asteroids {
        let mut slopes = HashSet::new();
        let mut visible = 0;

        for asteroid in asteroids {
            if station == asteroid {
                continue;
            }

            let mut rise = (station.1 as f32 - asteroid.1 as f32);
            let mut run = (station.0 as f32 - asteroid.0 as f32);
            let slope = rise / run;

            let slope = if slope.is_infinite() {
                String::from("Infinity")
            } else {
                format!("{:?}", slope)
            };
            let above = asteroid.1 < station.1 || (asteroid.1 == station.1 && asteroid.0 < station.0);
       
            if slopes.insert((slope.clone(), above)) {
                visible += 1;
            }
        }

        if visible > max_visible {
            max_visible = visible;
        }
    }

    max_visible 
}

fn part2(asteroids: &HashSet<Asteroid>) -> usize {
    0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let asteroids = reader
        .lines()
        .map(|line| line.expect("Can't read stdin"))
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
        .collect();

    let answer1 = part1(&asteroids);
    let answer2 = part2(&asteroids);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}
