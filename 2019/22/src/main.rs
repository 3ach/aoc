use adventage::day;
use modinverse::modinverse;
use mod_exp::mod_exp;

day!(2019, 22);

enum Operation {
    New,
    Cut(i128),
    Deal(i128),
}

type TInput = Vec<Operation>;

fn parse_line(line: &str) -> Operation {
    let (verb, rest) = line.split_once(" ").unwrap();
    if verb == "cut" {
        Operation::Cut(rest.parse().unwrap())
    } else if rest.starts_with("into") {
        Operation::New
    } else {
        let increment = rest.split(" ").last().unwrap().parse().unwrap();
        Operation::Deal(increment)
    }
}

fn parse(input: &str) -> TInput {
    input.lines()
        .map(parse_line)
        .collect()
}

fn coalesce(input: &TInput, card_count: i128) -> (i128, i128) {
    input.iter().fold((1, 0), |(a, b), rule| {
        match rule {
            Operation::New => ((-a).rem_euclid(card_count), -(b + 1).rem_euclid(card_count)),
            Operation::Cut(d) => (a, (b + card_count - d).rem_euclid(card_count)),
            Operation::Deal(i) => ((a * i).rem_euclid(card_count), (b * i).rem_euclid(card_count)),
        }
    })
}

fn coalesce_inv(input: &TInput, card_count: i128) -> (i128, i128) {
    input.iter().rev().fold((1, 0), |(a, b), rule| {
        match rule {
            Operation::New => ((-a).rem_euclid(card_count), -(b + 1).rem_euclid(card_count)),
            Operation::Cut(d) => (a, (b + d).rem_euclid(card_count)),
            Operation::Deal(i) => ((a * modinverse(*i, card_count).unwrap()).rem_euclid(card_count), (b * modinverse(*i, card_count).unwrap()).rem_euclid(card_count)),
        }
    })
}

fn part1(ops: &TInput) -> i128 {
    let (a, b) = coalesce(ops, 10007);

    ((2019 * a) + b).rem_euclid(10007)
}

fn part2(ops: &TInput) -> i128 {
    let cards = 119315717514047;
    let (a, b) = coalesce_inv(ops, cards);

    let times = 101741582076661;

    let first_times = mod_exp(a, times, cards).rem_euclid(cards);
    let second_times = ((1 - mod_exp(a, times, cards)) * modinverse(1 - a, cards).unwrap()).rem_euclid(cards);

    (2020 * first_times + b * second_times).rem_euclid(cards)
}
