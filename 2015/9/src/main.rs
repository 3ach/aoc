use adventage::{day, part1demo, part2demo};
use std::collections::HashMap;
use itertools::Itertools;

day!(2015, 9);
part1demo!("London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141", 605);
part2demo!("London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141", 982);

type TInput = HashMap<String, HashMap<String, usize>>;

fn parse(input: &str) -> TInput {
    let mut map = HashMap::new(); 
    for line in input.lines() {
        let parts: Vec<_> = line.split(" ").collect();
        map.entry(parts[0].to_string())
            .or_insert(HashMap::new())
            .insert(parts[2].to_string(), parts[4].parse().unwrap());
        map.entry(parts[2].to_string())
            .or_insert(HashMap::new())
            .insert(parts[0].to_string(), parts[4].parse().unwrap());
    }

    map
}

fn part1(map: &TInput) -> usize {
    map.keys()
        .permutations(map.len())
        .map(|path| 
            path.windows(2)
                .map(|path| map[path[0]][path[1]])
                .sum::<usize>()
        ).min().unwrap()
}

fn part2(map: &TInput) -> usize {
    map.keys()
        .permutations(map.len())
        .map(|path| 
            path.windows(2)
                .map(|path| map[path[0]][path[1]])
                .sum::<usize>()
        ).max().unwrap()
}
