use adventage::{day, part1demo, part2demo};

day!(2015, 6);
part1demo!("turn on 0,0 through 999,999\n", 1000000);
part1demo!("toggle 0,0 through 999,0\n", 1000);
part1demo!("turn on 0,0 through 999,999
turn off 499,499 through 500,500\n", 999996);
part2demo!("turn on 0,0 through 0,0\n", 1);
part2demo!("toggle 0,0 through 999,999", 2000000);

type Point = (usize, usize);

#[derive(Debug)]
enum Instruction {
    On(Point, Point),
    Off(Point, Point),
    Toggle(Point, Point),
}

type TInput = Vec<Instruction>;

fn parse(input: &str) -> TInput {
    input.lines().map(|line| {
        let coords = line.split(" ")
            .filter(|w| w.contains(","))
            .map(|c| c.split(",").map(|p| p.parse::<usize>().unwrap()))
            .flatten()
            .collect::<Vec<usize>>();
        let from = (coords[0], coords[1]);
        let to = (coords[2], coords[3]);

        if line.starts_with("turn on") {
            Instruction::On(from, to)
        } else if line.starts_with("turn off") {
            Instruction::Off(from, to)
        } else if line.starts_with("toggle") {
            Instruction::Toggle(from, to)
        } else {
            panic!()
        }
    }).collect()
}

fn part1(input: &TInput) -> usize {
    let mut lights = [[false; 1000]; 1000];

    for instruction in input {
        let (op, from, to): (fn(bool) -> bool, Point, Point) = match instruction {
            Instruction::On(from, to) => (|_| true, *from, *to),
            Instruction::Off(from, to) => (|_| false, *from, *to),
            Instruction::Toggle(from, to) => (|p| !p, *from, *to),
        };

        for row in from.0..=to.0 {
            for col in from.1..=to.1 {
                lights[row][col] = op(lights[row][col])
            }
        }
    }

    lights.iter()
        .map(|row| row.iter().filter(|l| **l).count())
        .sum()
}

fn part2(input: &TInput) -> usize {
    let mut lights = vec![vec![0; 1000]; 1000];

    for instruction in input {
        let (op, from, to): (fn(usize) -> usize, Point, Point) = match instruction {
            Instruction::On(from, to) => (|v| v + 1, *from, *to),
            Instruction::Off(from, to) => (|v| if v > 0 { v - 1 } else { v }, *from, *to),
            Instruction::Toggle(from, to) => (|v| v + 2, *from, *to),
        };

        for row in from.0..=to.0 {
            for col in from.1..=to.1 {
                lights[row][col] = op(lights[row][col])
            }
        }
    }

    lights.iter()
        .map(|row| row.iter().sum::<usize>())
        .sum()
}
