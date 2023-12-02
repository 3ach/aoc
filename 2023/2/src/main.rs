use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::cmp::max;

#[derive(Debug)]
struct Round {
    red: usize,
    blue: usize,
    green: usize,
}

type Game = Vec<Round>;

fn part1(games: &HashMap<u32, Game>) -> u32 {
    games.iter()
        .filter(|(_, game)| game.iter().all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14))
        .map(|(id, _)| id)
        .sum()
}

fn part2(games: &HashMap<u32, Game>) -> usize {
    games.iter()
        .map(|(_, game)| game.iter().fold(Round {red: 0, blue: 0, green: 0}, |needed, round| Round { red: max(needed.red, round.red), blue: max(needed.blue, round.blue), green: max(needed.green, round.green) }))
        .map(|needed| needed.red * needed.blue * needed.green)
        .sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let games = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            let parts = line.split_once(": ").unwrap();
            (String::from(parts.0), String::from(parts.1))
        })
        .map(|(id, rounds)| {
            (
                id.split_once(" ").unwrap().1.parse::<u32>().unwrap(),
                rounds,
            )
        })
        .map(|(id, rounds)| {
            (
                id,
                rounds.split("; ").map(|round| {
                    round
                        .split(", ")
                        .map(|round| round.split_once(" ").unwrap())
                        .map(|(num, color)| (num.parse::<usize>().unwrap(), color))
                        .map(|(num, color)| match color {
                            "red" => Round {
                                red: num,
                                blue: 0,
                                green: 0,
                            },
                            "blue" => Round {
                                red: 0,
                                blue: num,
                                green: 0,
                            },
                            "green" => Round {
                                red: 0,
                                blue: 0,
                                green: num,
                            },
                            _ => panic!(),
                        })
                        .reduce(|round, hand| Round {
                            red: round.red + hand.red,
                            green: round.green + hand.green,
                            blue: round.blue + hand.blue,
                        }).unwrap()
                }).collect::<Game>(),
            )
        })
        .collect::<HashMap<u32, Game>>();

    let answer1 = part1(&games);
    let answer2 = part2(&games);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}
