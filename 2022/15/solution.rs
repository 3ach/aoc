use std::io::BufRead;
use std::io;
use std::collections::HashSet;

type Point = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Reading {
    sensor: Point,
    beacon: Point,
}

type TInput = HashSet<Reading>;

fn manhattan(a: &Point, b: &Point) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn excludes(reading: &Reading, point: &Point) -> bool {
    manhattan(&reading.sensor, &reading.beacon) >= manhattan(&reading.sensor, &point)
}

fn row(reading: &Reading) -> isize {
    if reading.sensor.0 > 30 {
        2000000
    } else {
        10
    }
}

fn part1(input: &TInput) -> usize {
    let relevant = input.iter()
        .filter(|reading| excludes(reading, &(reading.sensor.0, row(reading))));

    let furthest = relevant.clone()
        .map(|reading| manhattan(&reading.sensor, &reading.beacon))
        .max()
        .unwrap();

    let min = relevant.clone()
        .map(|reading| reading.sensor.0 - furthest as isize)
        .min()
        .unwrap();
        
    let max = relevant.clone()
        .map(|reading| reading.sensor.0 + furthest as isize)
        .max()
        .unwrap();

    (min..=max)
        .filter(|x| relevant.clone().any(|reading| excludes(reading, &(*x, row(reading)))))
        .filter(|x| !relevant.clone().any(|reading| reading.beacon == (*x, row(reading))))
        .count()
}

fn part2(input: &TInput) -> isize {
    let max = row(input.iter().next().unwrap()) * 2;

    let loc = input.iter()
        .map(|reading| {
            let scope = manhattan(&reading.beacon, &reading.sensor) as isize + 1;
            (-scope..=scope)
                .map(|rowdiff| (rowdiff, scope - rowdiff))
                .map(|(rowdiff, coldiff)| (reading.sensor.0 + rowdiff, reading.sensor.1 + coldiff))
                .filter(|(r, c)| *r >= 0 && *r <= max && *c >= 0 && *c <= max)
                .filter(|point| !input.iter().any(|r| excludes(r, point)))
                .collect::<Vec<Point>>()
        })
        .flatten()
        .next()
        .unwrap();

    loc.0 * 4000000 + loc.1
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.split(" ")
             .filter(|word| word.contains("="))
             .map(|word| word.trim_matches(':').trim_matches(','))
             .filter_map(|word| word.split_once('='))
             .map(|(_, idx)| idx.parse::<isize>().unwrap())
             .collect::<Vec<isize>>()
        )
        .map(|idxs| Reading { 
            sensor: (idxs[0], idxs[1]),
            beacon: (idxs[2], idxs[3])
        })
        .collect();

	let answer1 = part1(&input);
	println!("Answer 1: {}", answer1);

	let answer2 = part2(&input);
	println!("Answer 2: {}", answer2);

    Ok(())
}
