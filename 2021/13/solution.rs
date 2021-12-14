use std::io::BufRead;
use std::io;
use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;
use std::convert::TryInto;

type Point = (usize, usize);

enum Fold {
    X(usize),
    Y(usize),
}

fn fold(sheet: &mut HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let folded: HashSet<Point> = HashSet::new();
    return sheet.iter().copied().map(|(x, y)| {
        match fold {
            Fold::X(line) if x < *line => (x, y),
            Fold::Y(line) if y < *line => (x, y),
            Fold::X(line) if x > *line => (line - (x - line), y),
            Fold::Y(line) if y > *line => (x, line - (y - line)),
            _ => panic!()
        }
    }).collect();
}

fn part1(points: &HashSet<Point>, folds: &Vec<Fold>) -> usize {
    let mut points = points.clone();

    points = fold(&mut points, &folds[0]);

    return points.len();
}

fn part2(points: &HashSet<Point>, folds: &Vec<Fold>) {
    let mut points = points.clone();

    for f in folds {
        points = fold(&mut points, f);
    }

    let xmax = points.iter().map(|(x, y)| y).max().unwrap();
    let ymax = points.iter().map(|(x, y)| x).max().unwrap();

    for x in 0..=*xmax {
        for y in 0..=*ymax {
            match points.get(&(y, x)) {
                Some(_) => print!("#"),
                None => print!(" "),
            }
        }
        println!("");
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();

    let mut points: HashSet<Point> = HashSet::new();
    let mut folds: Vec<Fold> = vec![];

    while stdin.read_line(&mut buf).unwrap() > 1 {
        let pts: Vec<usize> = buf.trim().split(",").map(|p| p.parse().unwrap()).collect();

        points.insert((pts[0], pts[1]));
        buf.truncate(0);
    }

    while stdin.read_line(&mut buf).unwrap() > 1 {
        let dir = &buf.trim()[11..12];
        let line: usize = buf.trim()[13..].parse().unwrap();
        
        let fold = match dir {
            "x" => Fold::X(line),
            "y" => Fold::Y(line),
            _ => panic!(),
        };

        folds.push(fold);
        buf.truncate(0);
    }

	let answer1 = part1(&points, &folds);
	println!("Answer 1: {}", answer1);

	part2(&points, &folds);

    Ok(())
}
