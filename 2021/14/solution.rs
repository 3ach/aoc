use std::io::BufRead;
use std::io;
use std::cmp;
use std::collections::HashMap;
use std::convert::TryInto;

fn expand(pairs: &HashMap<(char, char), usize>, rules: &HashMap<(char, char), char>) -> HashMap<(char, char), usize> {
    let mut next_pairs: HashMap<(char, char), usize> = HashMap::new();

    for ((first, second), count) in pairs {
        let (first, second) = (*first, *second);

        let middle = *rules.get(&(first, second)).unwrap();
        let first_pair = next_pairs.entry((first, middle)).or_insert(0);
        *first_pair += *count;

        let second_pair = next_pairs.entry((middle, second)).or_insert(0);
        *second_pair += *count;
    }

    return next_pairs;
}

fn part1(template: &Vec<char>, rules: &HashMap<(char, char), char>) -> usize {
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();

    for i in 1..template.len() {
        let pair = (template[i - 1], template[i]);
        let count = pairs.entry(pair).or_insert(0);

        *count += 1;
    }

    for _ in 0..10 {
        pairs = expand(&pairs, rules);
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for ((first, second), count) in pairs {
        let first_count = counts.entry(first).or_insert(0);
        *first_count += count;

        let second_count = counts.entry(second).or_insert(0);
        *second_count += count;
    }

    for (c, count) in counts.iter_mut() {
        if template[0] == *c || template[template.len() - 1] == *c {
            *count += 1;
        }

        *count /= 2;
    }

    let least_common = counts.iter().min_by_key(|(_, count)| *count).unwrap();
    let most_common = counts.iter().max_by_key(|(_, count)| *count).unwrap();

    return (*most_common.1 - *least_common.1);
}

fn part2(template: &Vec<char>, rules: &HashMap<(char, char), char>) -> usize {
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();

    for i in 1..template.len() {
        let pair = (template[i - 1], template[i]);
        let count = pairs.entry(pair).or_insert(0);

        *count += 1;
    }

    for _ in 0..40 {
        pairs = expand(&pairs, rules);
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for ((first, second), count) in pairs {
        let first_count = counts.entry(first).or_insert(0);
        *first_count += count;

        let second_count = counts.entry(second).or_insert(0);
        *second_count += count;
    }

    for (c, count) in counts.iter_mut() {
        if template[0] == *c || template[template.len() - 1] == *c {
            *count += 1;
        }

        *count /= 2;
    }

    let least_common = counts.iter().min_by_key(|(_, count)| *count).unwrap();
    let most_common = counts.iter().max_by_key(|(_, count)| *count).unwrap();

    return (*most_common.1 - *least_common.1);
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let template: Vec<char> = lines.next().unwrap().unwrap().chars().collect();
    lines.next();

    let rules: HashMap<(char, char), char> = lines.map(|line| {
        let line = line.unwrap();
        let mut chars = line.chars();

        ((chars.next().unwrap(), chars.next().unwrap()), chars.nth(4).unwrap())
    }).collect();

	let answer1 = part1(&template, &rules);
	let answer2 = part2(&template, &rules);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
