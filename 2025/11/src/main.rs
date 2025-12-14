use adventage::{day, part1demo, part2demo};
use std::collections::{HashMap, HashSet};

day!(2025, 11);
part1demo!(
"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out", 5);
part2demo!(
"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out", 2);

type TInput = HashMap<String, HashSet<String>>;

fn parse(input: &str) -> TInput {
    input.lines()
        .map(|line| {
            let (from, to) = line.split_once(": ").unwrap();
            (from.to_string(), to.split(" ").map(String::from).collect())
        })
    .collect()
}

fn paths(machines: &TInput, from: &str, to: &str, exclude: &HashSet<String>, known: &mut HashMap<String, usize>) -> usize {
    if from == to {
        return 1;
    }

    if let Some(known) = known.get(from) {
        return *known;
    }

    if let Some(children) = machines.get(from) {
        let recursive = children.iter()
            .filter(|c| !exclude.contains(*c))
            .map(|c| paths(machines, c, to, exclude, known))
            .sum();

        known.insert(String::from(from), recursive);
        return recursive;
    }

    0
}

fn part1(machines: &TInput) -> usize {
    paths(
        machines, 
        "you", 
        "out", 
        &HashSet::new(), 
        &mut HashMap::from([("out".to_string(), 1)])
    )
}

fn part2(machines: &TInput) -> usize {
    let fft_dac = paths(
        machines, 
        "svr", 
        "fft", 
        &HashSet::from(["dac".to_string()]), 
        &mut HashMap::from([("fft".to_string(), 1)])
    ) *
    paths(
        machines, 
        "fft", 
        "dac", 
        &HashSet::new(), 
        &mut HashMap::from([("dac".to_string(), 1)])
    ) *
    paths(
        machines, 
        "dac", 
        "out", 
        &HashSet::new(), 
        &mut HashMap::from([("out".to_string(), 1)])
    );

    let dac_fft = paths(
        machines, 
        "svr", 
        "dac", 
        &HashSet::from(["fft".to_string()]), 
        &mut HashMap::from([("dac".to_string(), 1)])
    ) *
    paths(
        machines, 
        "dac", 
        "fft", 
        &HashSet::new(), 
        &mut HashMap::from([("fft".to_string(), 1)])
    ) *
    paths(
        machines, 
        "fft", 
        "out", 
        &HashSet::new(), 
        &mut HashMap::from([("out".to_string(), 1)])
    );

    fft_dac + dac_fft
}
