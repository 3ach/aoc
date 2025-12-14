use adventage::{day, part1demo};

day!(2025, 12);

part1demo!(
"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2", 2);

#[derive(Debug)]
struct Region {
    dimensions: (usize, usize),
    counts: Vec<usize>,
}

type Present = [[bool; 3]; 3];

#[derive(Debug)]
struct Problem {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

type TInput = Problem;

fn parse(input: &str) -> TInput {
    let mut presents = vec![];
    let mut regions = vec![];
    let mut present = [[false; 3]; 3];
    let mut present_row = 0;

    for line in input.lines() {
        if line == "" {
            presents.push(present);
            present_row = 0;
            present = [[false; 3]; 3];
        } else if line.contains("#") {
            for (idx, c) in line.chars().enumerate() {
                present[present_row][idx] = c == '#';
            }
            present_row += 1;
        } else if line.len() > 2 {
            let (dimensions, counts) = line.split_once(": ").unwrap();
            let (row, col) = dimensions.split_once("x").unwrap();
            let region = Region {
                dimensions: (row.parse().unwrap(), col.parse().unwrap()),
                counts: counts.split(" ").map(str::parse).map(Result::unwrap).collect()
            };

            regions.push(region);
        }
    }
    
    Problem { presents, regions }
}

fn part1(input: &TInput) -> usize {
    let areas: Vec<usize> = input.presents.iter()
        .map(|p| p.iter().flatten().filter(|p| **p).count())
        .collect();

    input.regions.iter()
        .filter(|region| {
            let blocks = (region.dimensions.0 / 3) * (region.dimensions.1 / 3);
            let blocks_needed = region.counts.iter().sum();

            let area = region.dimensions.0 * region.dimensions.1;
            let area_needed = region.counts.iter().zip(&areas).map(|(count, area)| count * area).sum();

            if blocks >= blocks_needed {
                true
            } else if area <= area_needed {
                false
            } else {
                panic!()
            }
        }).count()
}

fn part2(input: &TInput) -> usize {
    0
}
