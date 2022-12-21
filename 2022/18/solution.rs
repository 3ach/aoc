use std::collections::HashSet;
use std::io;
use std::io::BufRead;

type Point = (isize, isize, isize);
type TInput = HashSet<Point>;

fn neighbors(point: &Point) -> HashSet<Point> {
    HashSet::from([
        (point.0, point.1, point.2),
        (point.0 + 1, point.1, point.2),
        (point.0 - 1, point.1, point.2),
        (point.0, point.1 + 1, point.2),
        (point.0, point.1 - 1, point.2),
        (point.0, point.1, point.2 + 1),
        (point.0, point.1, point.2 - 1),
    ])
}

fn part1(input: &TInput) -> usize {
    input
        .iter()
        .map(|point| 7 - input.intersection(&neighbors(point)).count())
        .sum()
}

fn part2(input: &TInput) -> usize {
    let xmin = input.iter().map(|(x, _, _)| x).min().unwrap() - 1;
    let xmax = input.iter().map(|(x, _, _)| x).max().unwrap() + 1;
    let ymin = input.iter().map(|(_, y, _)| y).min().unwrap() - 1;
    let ymax = input.iter().map(|(_, y, _)| y).max().unwrap() + 1;
    let zmin = input.iter().map(|(_, _, z)| z).min().unwrap() - 1;
    let zmax = input.iter().map(|(_, _, z)| z).max().unwrap() + 1;

    let mut airs: HashSet<Point> = input
        .iter()
        .map(|point| {
            neighbors(point)
                .difference(&input)
                .cloned()
                .collect::<HashSet<Point>>()
        })
        .flatten()
        .map(|point| {
            neighbors(&point)
                .difference(&input)
                .cloned()
                .collect::<HashSet<Point>>()
        })
        .flatten()
        .collect();

    airs.insert((xmin, ymin, zmin));

    let mut shell = HashSet::new();
    let mut air_to_check = vec![(xmin, ymin, zmin)];
    let mut checked = 0;
    while let Some(air) = air_to_check.pop() {
        checked += 1;
        for neighbor in neighbors(&air) {
            if neighbor.0 < xmin
                || neighbor.0 > xmax
                || neighbor.1 < ymin
                || neighbor.1 > ymax
                || neighbor.2 < zmin
                || neighbor.2 > zmax
                || !shell.insert(neighbor)
            {
                continue;
            }

            if !input.contains(&neighbor) {
                air_to_check.push(neighbor);
            }
        }
    }

    let all_surface = part1(input);
    let interior_surface: usize = airs
        .difference(&shell)
        .map(|air| input.intersection(&neighbors(air)).count())
        .sum();

    all_surface - interior_surface
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|nums| (nums[0], nums[1], nums[2]))
        .collect();

    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}
