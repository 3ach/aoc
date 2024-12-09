use adventage::{day, part1demo, part2demo};

day!(2024, 7);
part1demo!(
    "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    3749
);
part2demo!(
    "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    11387
);
part2demo!("156: 15 6", 156);
part2demo!("45260: 3 30 47 5 5", 45260);

type TInput = Vec<(u64, Vec<u64>)>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .map(|l| {
            let (target, terms) = l.split_once(": ").unwrap();
            let target = target.parse().unwrap();
            let terms = terms
                .split_whitespace()
                .map(|t| t.parse().unwrap())
                .collect();

            (target, terms)
        })
        .collect()
}

fn possible(target: u64, terms: &[u64], allow_concat: bool) -> bool {
    if terms.len() == 1 {
        target == terms[0]
    } else {
        let last = *terms.last().unwrap();
        let rest = &terms[0..terms.len() - 1];
        let magnitude = last.ilog10() as u32 + 1;

        (target % last == 0 && possible(target / last, rest, allow_concat))
            || (target > last && possible(target - last, rest, allow_concat))
            || (allow_concat
                && target > last
                && (target - last) % 10_u64.pow(magnitude) == 0
                && possible((target - last) / 10_u64.pow(magnitude), rest, allow_concat))
    }
}

fn part1(equations: &TInput) -> u64 {
    equations
        .iter()
        .filter(|(target, terms)| possible(*target, terms, false))
        .map(|(target, _)| target)
        .sum()
}

fn part2(equations: &TInput) -> u64 {
    equations
        .iter()
        .filter(|(target, terms)| possible(*target, terms, true))
        .map(|(target, _)| target)
        .sum()
}
