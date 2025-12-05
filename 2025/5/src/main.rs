#![feature(get_disjoint_mut_helpers)]

use adventage::{day, part1demo, part2demo};
use core::slice::GetDisjointMutIndex;
use std::cmp::{max, min};
use std::ops::RangeInclusive;

day!(2025, 5);
part1demo!(
    "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    3
);
part2demo!(
    "3-5
10-14
16-20
12-18

1
5
8
11
17
32",
    14
);

struct Inventory {
    ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

type TInput = Inventory;

fn parse(problem: &str) -> TInput {
    let mut ranges = vec![];
    let mut ingredients = vec![];
    let mut in_ingredients = false;

    for line in problem.lines() {
        if line.len() == 0 {
            in_ingredients = true;
        } else if in_ingredients {
            ingredients.push(line.parse().unwrap());
        } else {
            let (from, to) = line.split_once("-").unwrap();
            let from = from.parse().unwrap();
            let to = to.parse().unwrap();

            ranges.push(from..=to)
        }
    }

    Inventory {
        ranges,
        ingredients,
    }
}

fn part1(input: &TInput) -> usize {
    input
        .ingredients
        .iter()
        .filter(|ingredient| input.ranges.iter().any(|range| range.contains(ingredient)))
        .count()
}

fn coalesce(ranges: &Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    let mut coalesced = vec![];

    'per_range: for range in ranges {
        for c_range in coalesced.iter_mut() {
            if range.is_overlapping(c_range) {

                *c_range =
                    min(*range.start(), *c_range.start())..=max(*range.end(), *c_range.end());
                continue 'per_range;
            }
        }

        coalesced.push(range.clone());
    }

    coalesced
}

fn part2(input: &TInput) -> usize {
    let mut current = input.ranges.clone();

    loop {
        let next = coalesce(&current);
        if next.len() == current.len() {
            break;
        }

        current = next;
    }

    current 
        .into_iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}
