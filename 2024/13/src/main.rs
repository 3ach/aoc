#![feature(iter_array_chunks)]
use adventage::{day, part1demo, part2demo};

day!(2024, 13);
part1demo!(
    "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    480
);

type Button = (i128, i128);
type Prize = (i128, i128);
type Game = (Button, Button, Prize);

type TInput = Vec<Game>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .filter(|l| *l != "")
        .array_chunks()
        .map(|[a, b, prize]| {
            let mut a = a
                .split(&[' ', '\n', ','])
                .filter(|c| c.len() > 0)
                .skip(2)
                .map(|s| s[2..].parse::<i128>().unwrap());
            let mut b = b
                .split(&[' ', '\n', ','])
                .filter(|c| c.len() > 0)
                .skip(2)
                .map(|s| s[2..].parse::<i128>().unwrap());
            let mut prize = prize
                .split(&[' ', '\n', ','])
                .filter(|c| c.len() > 0)
                .skip(1)
                .map(|s| s[2..].parse::<i128>().unwrap());

            (
                (a.next().unwrap(), a.next().unwrap()),
                (b.next().unwrap(), b.next().unwrap()),
                (prize.next().unwrap(), prize.next().unwrap()),
            )
        })
        .collect()
}

fn cheapest(game: &Game) -> (i128, i128) {
    let determinant = (game.0.0 * game.1.1) - (game.0.1 * game.1.0);
    let determinant_a = (game.2.0 * game.1.1) - (game.2.1 * game.1.0);
    let determinant_b = (game.0.0 * game.2.1) - (game.0.1 * game.2.0);

    let a = determinant_a / determinant;
    let b = determinant_b / determinant;

    if a < 0 || b < 0 || determinant_a % determinant != 0 || determinant_b % determinant != 0 {
        (0, 0)
    } else {
        (a, b)
    }
}

fn part1(games: &TInput) -> i128 {
    games
        .iter()
        .map(cheapest)
        .filter(|(a, b)| *a <= 100 && *b <= 100)
        .map(|(a, b)| (3 * a) + b)
        .sum()
}

fn part2(games: &TInput) -> i128 {
    games
        .iter()
        .map(|game| {
            let mut game = game.clone();
            game.2.0 += 10000000000000; 
            game.2.1 += 10000000000000;
            game
        })
        .map(|game| cheapest(&game))
        .map(|(a, b)| (3 * a) + b)
        .sum()
}
