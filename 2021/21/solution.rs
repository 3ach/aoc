use std::io::BufRead;
use std::io;
use std::cmp;
use std::collections::HashMap;

fn part1(players: &mut [usize]) -> usize {
    let mut scores = [0, 0];
    let mut dice = 1;
    let mut player = 0;

    while scores[(player + 1) % 2] < 1000 {
        players[player] += (3 * dice + 3);
        players[player] %= 10;

        scores[player] += players[player] + 1;
        player = (player + 1) % 2;
        dice += 3;
    }

    scores[player] * (dice - 1)
}

const FREQUENCIES: [u128; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

#[derive(Debug,Clone)]
struct Game {
    positions: [usize; 2],
    scores: [usize; 2],
    universes: u128,
    player: usize
}

fn part2(players: &[usize]) -> u128 {
    let mut wins: [u128; 2] = [0, 0];
    let mut games: Vec<Game> = vec![Game{positions: [players[0], players[1]], scores: [0, 0], universes: 1, player: 0}];
    let mut turns = 0;

    while games.len() > 0 {
        let game = games.pop().unwrap();

        for roll in 3..=9 {
            let player = game.player;
            let frequency = FREQUENCIES[roll] * game.universes;

            let mut next_positions = game.positions;
            next_positions[player] = (next_positions[player] + roll) % 10;

            let mut next_scores = game.scores;
            next_scores[player] += next_positions[player] + 1;

            if next_scores[player] >= 21 {
                wins[player] += frequency as u128;
            } else {
                let next_player = (player + 1) % 2;
                let next_game = Game{positions: next_positions, scores: next_scores, universes: frequency, player: next_player};
                games.push(next_game);
            }
        }

        turns += 1;
    }

    cmp::max(wins[0], wins[1])
}


fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let starting: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().split(": ").nth(1).unwrap().parse::<usize>().unwrap() - 1)
        .collect();

    let players = &mut starting.clone()[..];
	let answer1 = part1(players);

    let players = &mut starting.clone()[..];
	let answer2 = part2(players);

	println!("Answer 1: {:?}", answer1);
	println!("Answer 2: {:?}", answer2);

    Ok(())
}
