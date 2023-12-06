use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Conversion {
    source: i128,
    destination: i128,
    count: i128,
}

fn apply(num: i128, map: &[Conversion]) -> i128 {
    for conversion in map {
        let offset = num - conversion.source;
        if offset < 0 || offset >= conversion.count {
            continue;
        }

        return conversion.destination + offset;
    }

    num
}

fn apply_range(start: i128, end: i128, map: &[Conversion]) -> Vec<(i128, i128)> {
    let mut results = vec![];
    let mut candidates = vec![(start, end)];

    for conversion in map {
        let mut unprocessed = vec![];

        while let Some(candidate) = candidates.pop() {
            let range_start = conversion.source;
            let range_end = conversion.source + conversion.count;

            if candidate.0 >= range_end || candidate.1 < range_start {
                unprocessed.push(candidate);
                continue;
            }

            let processed_start = if candidate.0 < range_start {
                unprocessed.push((candidate.0, range_start));
                range_start
            } else {
                candidate.0
            };

            let processed_end = if candidate.1 > range_end {
                unprocessed.push((range_end, candidate.1));
                range_end
            } else {
                candidate.1
            };

            let offset = conversion.destination - conversion.source;
            results.push((processed_start + offset, processed_end + offset));
        }

        candidates = unprocessed;
    }

    results.append(&mut candidates);

    results
}

fn part1(seeds: &Vec<i128>, map: &[Vec<Conversion>]) -> i128 {
    *map.iter()
        .fold(seeds.clone(), |vals, step| {
            vals.iter()
                .map(|val| apply(*val, &step))
                .collect::<Vec<i128>>()
        })
        .iter()
        .min()
        .unwrap()
}

fn part2(seeds: &[i128], map: &[Vec<Conversion>]) -> i128 {
    let mut seeds: Vec<(i128, i128)> = seeds.chunks(2)
        .map(|range| (range[0], range[0] + range[1]))
        .collect();

    for conversion in map {
        seeds = seeds.iter().map(|(start, end)| apply_range(*start, *end, conversion)).flatten().collect(); 
    }


    *seeds.iter()
        .map(|(start, _)| start)
        .min()
        .unwrap()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut steps = vec![];
    let mut map = vec![];
    let mut seeds = vec![];

    for line in reader.lines() {
        let line = line.expect("Couldn't read stdin");
        let line = line.trim();

        if line.contains("map:") {
            continue;
        }

        if line.len() == 0 {
            if !map.is_empty() {
                steps.push(map);
                map = vec![];
            }
            continue;
        }

        if line.starts_with("seeds: ") {
            seeds = line
                .split_whitespace()
                .filter_map(|x| match x.parse::<i128>() {
                    Ok(num) => Some(num),
                    _ => None,
                })
                .collect();
            continue;
        }

        let parts: Vec<i128> = line
            .split_whitespace()
            .map(|part| part.parse().unwrap())
            .collect();

        map.push(Conversion {
            source: parts[1],
            destination: parts[0],
            count: parts[2],
        });
    }

    steps.push(map);

    let answer1 = part1(&seeds, &steps);
    let answer2 = part2(&seeds, &steps);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}
