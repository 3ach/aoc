use std::io::BufRead;
use std::io;


enum Shape {
    Rock,
    Paper,
    Scissors
}

type TInput = Vec<(Shape, Shape)>;

fn part1(input: &TInput) -> u32 {
    input.iter()
        .map(|round| {
        let shape = match round.1 {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        let win = match round {
            (Shape::Rock, Shape::Rock) | (Shape::Paper, Shape::Paper) | (Shape::Scissors, Shape::Scissors) => 3,
            (Shape::Rock, Shape::Paper) | (Shape::Paper, Shape::Scissors) | (Shape::Scissors, Shape::Rock) => 6,
            _ => 0
        };

        win + shape
    }).sum()
}

fn part2(input: &TInput) -> u32 {
    input.iter()
        .map(|round| {
        let win = match round.1 {
            Shape::Rock => 0,
            Shape::Paper => 3,
            Shape::Scissors => 6,
        };

        let shape = match round {
            (Shape::Scissors, Shape::Scissors) | (Shape::Rock, Shape::Paper) | (Shape::Paper, Shape::Rock) => 1,
            (Shape::Rock, Shape::Scissors) | (Shape::Paper, Shape::Paper) | (Shape::Scissors, Shape::Rock) => 2,
            (Shape::Paper, Shape::Scissors) | (Shape::Scissors, Shape::Paper) | (Shape::Rock, Shape::Rock) => 3,
        };

        win + shape
    }).sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            let (theirs, mine) = line.split_once(" ").unwrap();
            let theirs = match theirs {
                "A" => Shape::Rock,
                "B" => Shape::Paper,
                "C" => Shape::Scissors,
                _ => panic!(),
            };

            let mine = match mine {
                "X" => Shape::Rock,
                "Y" => Shape::Paper,
                "Z" => Shape::Scissors,
                _ => panic!(),
            };

            (theirs, mine)
        })
        .collect();

	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
