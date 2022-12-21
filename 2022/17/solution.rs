use std::io::BufRead;
use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Jet {
    Left,
    Right
}

type TInput = Vec<Jet>;
type Point = (usize, usize);

fn print(map: &HashSet<Point>) {
    let mut highest = map.iter().map(|(_, y)| y).max().unwrap();

    for y in (0..=highest+3).rev() {
        for x in 0..7 {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    println!("\n\n")
}

fn shape(turn: usize, lowest: usize) -> HashSet<Point> {
    let mut shape = match turn % 5 {
        0 => HashSet::from([(2, 0 + lowest), (3, 0 + lowest), (4, 0 + lowest), (5, 0 + lowest)]),
        1 => HashSet::from([(3, 2 + lowest), (2, 1 + lowest), (3, 1 + lowest), (4, 1 + lowest), (3, 0 + lowest)]),
        2 => HashSet::from([(4, 2 + lowest), (4, 1 + lowest), (2, 0 + lowest), (3, 0 + lowest), (4, 0 + lowest)]),
        3 => HashSet::from([(2, 3 + lowest), (2, 2 + lowest), (2, 1 + lowest), (2, 0 + lowest)]),
        4 => HashSet::from([(2, 1 + lowest), (3, 1 + lowest), (2, 0 + lowest), (3, 0 + lowest)]),
        _ => panic!()
    };

    shape
}

fn side(shape: &HashSet<Point>, direction: Jet) -> HashSet<Point> {
    let leftest = *shape.iter().map(|(x, _)| x).min().unwrap(); 
    let rightest = *shape.iter().map(|(x, _)| x).max().unwrap(); 

    if direction == Jet::Left && leftest == 0 {
        return shape.clone();
    }

    if direction == Jet::Right && rightest == 6 {
        return shape.clone();
    }

    shape.iter()
        .map(|(x, y)| match direction {
            Jet::Left => (x - 1, *y),
            Jet::Right => (x + 1, *y)
        }).collect()
}

fn down(shape: &HashSet<Point>) -> HashSet<Point> {
    shape.iter()
        .map(|(x, y)| (*x, y - 1))
        .collect()
}


fn run(input: &TInput, rocks: usize) -> usize {
    let mut points = HashSet::new();
    let mut highest = 0;
    let mut turns = input.iter().enumerate().cycle();
    let mut restings = HashMap::new();
    let mut rocksleft = rocks;
    let mut skip = 0;

    loop {
        let rock = rocks - rocksleft;
        rocksleft -= 1;
        let mut leftest = 7;
        let mut shape = shape(rock, highest + 3);

        loop {
            let (turnidx, turn) = turns.next().unwrap();
            let shifted = side(&shape, *turn);
            if shifted.intersection(&points).count() == 0 {
                shape = shifted;
            }

            let lowest = *shape.iter().map(|(_, y)| y).min().unwrap();
            let down = down(&shape);
            if down.intersection(&points).count() == 0 && lowest > 0 {
                shape = down;
            } else {
                for point in shape {
                    if point.1 >= highest {
                        highest = point.1 + 1;
                    }
                    if point.0 < leftest {
                        leftest = point.0;
                    }
                
                    points.insert(point);
                }
                if let Some((prev_highest, prev_rock)) = restings.insert((rock % 5, turnidx, leftest), (highest, rock)) {
                    let cycle_time = rock - prev_rock;
                    let cycle_height = highest - prev_highest;
                    skip = cycle_height  * (rocksleft / cycle_time);
                    rocksleft %= cycle_time;
                    restings.clear();
                    //println!("Duplicate detected. {:?}", (rock % 5, turnidx, leftest, rock, prev_highest, restings.len(), highest));
                }
                break;
            }
        }

        if rocksleft == 0 {
            break;
        }
    }

    highest + skip
}

fn part1(input: &TInput) -> usize {
    run(input, 2022)
}

fn part2(input: &TInput) -> usize {
    run(input, 1000000000000)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.chars()
             .map(|c|
                match c {
                    '>' => Jet::Right, 
                    '<' => Jet::Left,
                    _ => panic!(),
                }).collect::<Vec<Jet>>()
             )
        .next()
        .unwrap();

	let answer1 = part1(&input);
	println!("Answer 1: {}", answer1);

	let answer2 = part2(&input);
	println!("Answer 2: {}", answer2);

    Ok(())
}
