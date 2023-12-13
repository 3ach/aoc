use std::io;
use std::io::BufRead;
use std::cmp::min;

#[derive(Debug)]
enum Axis {
    Horizontal(usize),
    Vertical(usize),
}

fn horizontal(image: &[String], axis: usize, allowed: usize) -> bool {
    let to_check = min(axis, image.len() - axis);
    let mut smudges = 0;

    for distance in 0..to_check {
        for (a, b) in image[axis - distance - 1].chars().zip(image[axis + distance].chars()) {
            if a != b {
                smudges += 1;
            }
        }
    }

    return smudges == allowed;
}

fn vertical(image: &[String], axis: usize, allowed: usize) -> bool {
    let to_check = min(axis, image[0].len() - axis);

    let mut smudges = 0;

    for distance in 0..to_check {
        for row in image {
            if row.chars().nth(axis - distance - 1) != row.chars().nth(axis + distance) {
                smudges += 1;
            }
        }
    }

    return smudges == allowed;
}

fn axis(image: &[String], allowed: usize) -> Axis {
    for axis in 1..image.len() {
        if horizontal(image, axis, allowed) {
            return Axis::Horizontal(axis);
        }
    }

    for axis in 1..image[0].len() {
        if vertical(image, axis, allowed) {
            return Axis::Vertical(axis);
        }
    }

    panic!()
}

fn part1(input: &[Vec<String>]) -> usize {
    input.iter()
        .map(|image| match axis(image, 0) {
            Axis::Vertical(left) => left,
            Axis::Horizontal(above) => 100 * above
        })
        .sum()
}

fn part2(input: &[Vec<String>]) -> usize {
    input.iter()
        .map(|image| match axis(image, 1) {
            Axis::Vertical(left) => left,
            Axis::Horizontal(above) => 100 * above
        })
        .sum()
}


fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let rows = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .fold(vec![vec![]], |mut vecs, row| { 
            if row.is_empty() {
                vecs.push(vec![]);
            } else {
                vecs.last_mut().unwrap().push(row);
            } 
            vecs
        });

    let answer1 = part1(&rows);
    println!("Answer 1: {}", answer1);

    let answer2 = part2(&rows);
    println!("Answer 2: {}", answer2);

    Ok(())
}
