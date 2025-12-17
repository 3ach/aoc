use adventage::day;
use std::collections::HashSet;
use std::str::FromStr;

day!(2019, 3);

#[derive(Debug)]
enum Step {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

#[derive(Debug, PartialEq, Eq)]
struct StepParseErr;

impl FromStr for Step {
    type Err = StepParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.get(0..1).ok_or(StepParseErr)?;
        let steps = s.get(1..).ok_or(StepParseErr)?;
        let steps = steps.parse::<isize>().map_err(|_| StepParseErr{} )?;

        match direction {
            "U" => Ok(Step::Up(steps)),
            "D" => Ok(Step::Down(steps)),
            "L" => Ok(Step::Left(steps)),
            "R" => Ok(Step::Right(steps)),
            _ => Err(StepParseErr{})
        }
    }
}

type Path = Vec<Step>;
type TInput = Vec<Path>;
type Point = (isize, isize);

fn points(path: &Path) -> Vec<Point> {
    let mut loc = (0, 0);
    let mut locs = vec![];

    for step in path {
        match step {
            Step::Up(n) => ((loc.1 + 1)..=(loc.1 + n)).map(|y| { loc = (loc.0, y); locs.push(loc) }).count(),
            Step::Down(n) => ((loc.1 - n)..loc.1).rev().map(|y| { loc = (loc.0, y); locs.push(loc) }).count(),
            Step::Right(n) => ((loc.0 + 1)..=(loc.0 + n)).map(|x| { loc = (x, loc.1); locs.push(loc) }).count(),
            Step::Left(n) => ((loc.0 - n)..loc.0).rev().map(|x| { loc = (x, loc.1); locs.push(loc) }).count(),
        };
    }

    locs
}

fn parse(input: &str) -> TInput {
    input.lines()
        .map(|line| line.split(',').map(|step| step.parse().expect("Couldn't parse")).collect())
        .collect()
}

fn part1(input: &TInput) -> isize {
    input.iter()
        .map(|path| points(path).iter().cloned().collect::<HashSet<Point>>())
        .reduce(|collide, points| &points & &collide)
        .unwrap()
        .iter()
        .filter_map(|collision| if collision == &(0, 0) { println!("NONE"); None } else { Some(collision.0.abs() + collision.1.abs()) })
        .min()
        .unwrap()
}

fn part2(input: &TInput) -> isize {
    let paths = input.iter().map(|path| points(path)).collect::<Vec<_>>();
    let intersects = paths.iter()
        .map(|path| path.iter().cloned().collect::<HashSet<Point>>())
        .reduce(|collide, points| &points & &collide)
        .unwrap();

    intersects.iter()
        .map(|intersect| paths.iter()
             .map(|path| path.iter().position(|pt| pt == intersect).unwrap() as isize)
             .sum::<isize>())
        .min()
        .unwrap() + (2 as isize)
}
