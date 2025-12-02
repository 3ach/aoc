use adventage::{day, part1demo, part2demo};
use std::cmp::{max, min};
use std::collections::HashSet;

day!(2025, 2);
part1demo!("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124", 1227775554);
part2demo!("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124", 4174379265);

type TInput = Vec<(u64, u64)>;

fn parse(input: &str) -> TInput {
    input.split(',')
        .into_iter()
        .map(|s| {
            let (first, second) = s.trim().split_once('-').unwrap();
            let first: u64 = first.parse().unwrap();
            let second: u64 = second.parse().unwrap();

            (first, second)
        })
        .collect()
}

fn expand(input: &TInput) -> TInput {
    input.into_iter()
        .map(|(start, end)| {
            let start_places = start.ilog10() + 1; 
            let end_places = end.ilog10() + 1; 

            (start_places..=end_places).into_iter()
                .map(|places| 
                    (max(*start, 10_u64.pow(places - 1)), 
                     min(*end, 10_u64.pow(places) - 1))
                )
        }).flatten()
        .collect()
}

fn invalid(start: u64, end: u64, parts: u32) -> Vec<u64> {
    let places = start.ilog10() + 1;
    if places % parts == 1 {
        return vec![];
    }

    let part_length = places / parts;
    let divisor = 10_u64.pow(places - part_length) as u64;

    let first = start / divisor;
    let last = (end / divisor) + 1;


    (first..=last)
        .into_iter()
        .map(|part| (0..parts)
            .map(|section| part as u64 * 10_u64.pow(section * part_length))
            .sum())
        .filter(|half| *half >= start && *half <= end)
        .collect()
}

fn part1(input: &TInput) -> u64 {
    expand(input)
        .into_iter()
        .map(|(s, e)| invalid(s, e, 2))
        .flatten()
        .sum()
}

fn part2(input: &TInput) -> u64 {
    let invalid_ids: HashSet<u64> = expand(input)
        .into_iter()
        .map(|(s, e)| {
            let parts = s.ilog10() + 1;
            (2..=parts)
                .map(move |parts| invalid(s, e, parts))
                .flatten()
        })
        .flatten()
        .collect();

    invalid_ids
        .into_iter()
        .sum()
}
