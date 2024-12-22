use adventage::{day, part1demo, part2demo};
use std::collections::{HashMap, HashSet};

day!(2024, 22);
part1demo!(
    "1
10
100
2024",
    37327623
);
part2demo!(
    "1
2
3
2024",
    23
);

type TInput = Vec<i64>;

fn parse(input: &str) -> TInput {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn mix(num: i64, secret: i64) -> i64 {
    num ^ secret
}

fn prune(num: i64) -> i64 {
    num % 16777216
}

fn evolve(num: i64) -> i64 {
    let num = prune(mix(num * 64, num));
    let num = prune(mix(num / 32, num));
    prune(mix(num * 2048, num))
}

fn forecast(seed: i64) -> i64 {
    (0..2000).fold(seed, |secret, _| evolve(secret))
}

fn changes(seed: i64) -> Vec<(i64, i64)> {
    (0..2000)
        .scan(seed, |secret, _| {
            let next_secret = evolve(*secret);
            let next_price = next_secret % 10;
            let diff = next_price - (*secret % 10);
            *secret = next_secret;
            Some((diff, next_price))
        })
        .collect()
}

fn part1(starts: &TInput) -> i64 {
    starts.iter().map(|seed| forecast(*seed)).sum()
}

fn part2(starts: &TInput) -> i64 {
    let mut bananas = HashMap::new();
    for start in starts {
        let monkey = changes(*start);
        let mut monkey_maxes: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
        for (idx, window) in monkey.windows(4).enumerate() {
            let changes = (window[0].0, window[1].0, window[2].0, window[3].0);
            if !monkey_maxes.contains_key(&changes) {
                monkey_maxes.insert(changes, window[3].1);
            }
        }
        for (sequence, max) in monkey_maxes {
            *bananas.entry(sequence).or_insert(0) += max;
        }
    }

    *bananas.iter().map(|(_, total)| total).max().unwrap()
}
