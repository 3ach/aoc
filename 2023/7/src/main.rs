use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

#[derive(Eq, Clone, Debug)]
struct Turn {
    cards: Vec<u32>,
    bet: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_to_kind(hand: &[u32]) -> Kind {
    let distinct: HashSet<u32> = hand.iter().cloned().collect();
    let jokers = hand.iter().filter(|c| **c == 1).count();

    if distinct.len() == 1 {
        return Kind::FiveOfAKind;
    }

    if distinct.len() == 2 {
        let min_count = distinct
            .iter()
            .map(|d| hand.iter().filter(|c| *c == d).count())
            .min()
            .unwrap();

        if min_count == 1 {
            if jokers >= 1 {
                return Kind::FiveOfAKind;
            } else {
                return Kind::FourOfAKind;
            }
        } else if min_count == 2 {
            if jokers >= 2 {
                return Kind::FiveOfAKind;
            } else if jokers == 1 {
                return Kind::FourOfAKind;
            }
            
            return Kind::FullHouse;
        } else {
            panic!();
        }
    }

    if distinct.len() == 3 {
        let max_count = distinct
            .iter()
            .map(|d| hand.iter().filter(|c| *c == d).count())
            .max()
            .unwrap();
        if max_count == 3 {
            if jokers == 3 || jokers == 1 {
                return Kind::FourOfAKind;
            } else if jokers == 2 {
                panic!();
            }
            
            return Kind::ThreeOfAKind;
        } else if max_count == 2 {
            if jokers == 2 {
                return Kind::FourOfAKind;
            } else if jokers == 1 {
                return Kind::FullHouse;
            } else {
                return Kind::TwoPair;
            }
        } else {
            panic!();
        }
    }

    if distinct.len() == 4 {
        if jokers > 0 {
            return Kind::ThreeOfAKind;
        }

        return Kind::OnePair;
    }

    if jokers > 0 {
        return Kind::OnePair;
    } else {
        return Kind::HighCard;
    }
}

impl PartialOrd for Turn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Turn {
    fn cmp(&self, other: &Self) -> Ordering {
        let kind = hand_to_kind(&self.cards);
        let other_kind = hand_to_kind(&other.cards);

        if kind != other_kind {
            kind.cmp(&other_kind)
        } else {
            let mut idx = 0;
            while self.cards[idx] == other.cards[idx] {
                idx += 1;
            }

            self.cards[idx].cmp(&other.cards[idx])
        }
    }
}

impl PartialEq for Turn {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bet == other.bet
    }
}

fn part1(turns: &[Turn]) -> u32 {
    let mut turns: Vec<Turn> = turns.iter().cloned().collect();
    turns.sort();

    turns
        .iter()
        .enumerate()
        .map(|(idx, turn)| (idx + 1) as u32 * turn.bet)
        .sum()
}

fn part2(turns: &[Turn]) -> u32 {
    let mut turns: Vec<Turn> = turns.iter().cloned().collect();
    turns.sort();

    turns
        .iter()
        .enumerate()
        .map(|(idx, turn)| (idx + 1) as u32 * turn.bet)
        .sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let turns = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            let (cards, bet) = line.split_once(" ").unwrap();
            let cards = cards
                .chars()
                .map(|c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => c.to_digit(10).unwrap(),
                })
                .collect();

            let bet = bet.parse::<u32>().unwrap();

            Turn { cards, bet }
        })
        .collect::<Vec<Turn>>();

    let answer1 = part1(&turns);

    let turns: Vec<Turn> = turns
        .iter()
        .map(|turn| Turn {
            bet: turn.bet,
            cards: turn.cards
                .iter()
                .map(|c| if *c == 11 { 1 } else { *c })
                .collect(),
        })
        .collect();

    let answer2 = part2(&turns);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}
