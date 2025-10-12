use adventage::{day, part1demo, part2demo};

day!(2024, 25);
part1demo!(
    "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####",
    3
);

#[derive(Debug, Copy, Clone)]
enum Schematic {
    Key(usize, usize, usize, usize, usize),
    Lock(usize, usize, usize, usize, usize),
}

type TInput = Vec<Schematic>;

fn parse_schematic(schematic: &str) -> Schematic {
    let mut heights = (0, 0, 0, 0, 0);
    let schematic: Vec<Vec<char>> = schematic
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for row in 1..=5 {
        if schematic[row][0] == '#' {
            heights.0 += 1;
        }
        if schematic[row][1] == '#' {
            heights.1 += 1;
        }
        if schematic[row][2] == '#' {
            heights.2 += 1;
        }
        if schematic[row][3] == '#' {
            heights.3 += 1;
        }
        if schematic[row][4] == '#' {
            heights.4 += 1;
        }
    }

    if schematic[0][0] == '#' {
        Schematic::Lock(heights.0, heights.1, heights.2, heights.3, heights.4)
    } else {
        Schematic::Key(heights.0, heights.1, heights.2, heights.3, heights.4)
    }
}

fn parse(input: &str) -> TInput {
    input.split("\n\n").map(parse_schematic).collect()
}

fn part1(schematics: &TInput) -> usize {
    schematics
        .iter()
        .map(|schematic| {
            if let Schematic::Key(k1, k2, k3, k4, k5) = schematic {
                schematics
                    .iter()
                    .filter(|schematic| {
                        if let Schematic::Lock(l1, l2, l3, l4, l5) = schematic {
                            k1 + l1 <= 5
                                && k2 + l2 <= 5
                                && k3 + l3 <= 5
                                && k4 + l4 <= 5
                                && k5 + l5 <= 5
                        } else {
				false
			}
                    })
                    .count()
            } else {
                0
            }
        })
        .sum()
}

fn part2(_map: &TInput) -> u32 {
    0
}
