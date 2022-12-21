use std::cmp;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug)]
struct Blueprint {
    num: usize,
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
}

#[derive(Debug, Clone)]
struct Tick {
    minute: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
}

type TInput = Vec<Blueprint>;

fn cost(ore: usize, clay: usize, obsidian: usize) -> Cost {
    Cost {
        ore,
        clay,
        obsidian,
    }
}

fn can_afford(tick: &Tick, cost: &Cost) -> bool {
    tick.ore >= cost.ore && tick.clay >= cost.clay && tick.obsidian >= cost.obsidian
}

fn evaluate(blueprint: &Blueprint, time: usize) -> usize {
    let mut max = 0;
    let mut ticks = vec![];

    ticks.push(Tick {
        minute: 1,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
    });

    let max_ore = cmp::max(blueprint.clay.ore, cmp::max(blueprint.obsidian.ore, blueprint.geode.ore));

    while let Some(tick) = ticks.pop() {
        let mins_left = time - tick.minute;
        let geode_potential = ((mins_left + 1) * mins_left) / 2;
        if tick.geode + geode_potential < max {
            continue;
        }

        if tick.geode > max {
            max = tick.geode;
        }

        if tick.minute == time + 1 {
            continue;
        }

        if can_afford(&tick, &blueprint.geode) {
            ticks.push(Tick {
                minute: tick.minute + 1,
                ore: tick.ore + tick.ore_robots - blueprint.geode.ore,
                clay: tick.clay + tick.clay_robots - blueprint.geode.clay,
                obsidian: tick.obsidian + tick.obsidian_robots - blueprint.geode.obsidian,
                geode: tick.geode + (time - tick.minute),
                ore_robots: tick.ore_robots,
                clay_robots: tick.clay_robots,
                obsidian_robots: tick.obsidian_robots,
            });

            continue;
        } else {
            let geode_potential = ((mins_left - 1) * mins_left) / 2;
            if tick.geode + geode_potential < max {
                continue;
            }
        }

        ticks.push(Tick {
            minute: tick.minute + 1,
            ore: tick.ore + tick.ore_robots,
            clay: tick.clay + tick.clay_robots,
            obsidian: tick.obsidian + tick.obsidian_robots,
            geode: tick.geode,
            ore_robots: tick.ore_robots,
            clay_robots: tick.clay_robots,
            obsidian_robots: tick.obsidian_robots,
        });


        if can_afford(&tick, &blueprint.obsidian)
            && tick.obsidian_robots < blueprint.geode.obsidian
        {
            ticks.push(Tick {
                minute: tick.minute + 1,
                ore: tick.ore + tick.ore_robots - blueprint.obsidian.ore,
                clay: tick.clay + tick.clay_robots - blueprint.obsidian.clay,
                obsidian: tick.obsidian + tick.obsidian_robots - blueprint.obsidian.obsidian,
                geode: tick.geode,
                ore_robots: tick.ore_robots,
                clay_robots: tick.clay_robots,
                obsidian_robots: tick.obsidian_robots + 1,
            });
        }

        if can_afford(&tick, &blueprint.clay)
            && tick.clay_robots < blueprint.obsidian.clay
        {
            ticks.push(Tick {
                minute: tick.minute + 1,
                ore: tick.ore + tick.ore_robots - blueprint.clay.ore,
                clay: tick.clay + tick.clay_robots - blueprint.clay.clay,
                obsidian: tick.obsidian + tick.obsidian_robots - blueprint.clay.obsidian,
                geode: tick.geode,
                ore_robots: tick.ore_robots,
                clay_robots: tick.clay_robots + 1,
                obsidian_robots: tick.obsidian_robots,
            });
        }

        if can_afford(&tick, &blueprint.ore) && tick.ore_robots < max_ore
        {
            ticks.push(Tick {
                minute: tick.minute + 1,
                ore: tick.ore + tick.ore_robots - blueprint.ore.ore,
                clay: tick.clay + tick.clay_robots - blueprint.ore.clay,
                obsidian: tick.obsidian + tick.obsidian_robots - blueprint.ore.obsidian,
                geode: tick.geode,
                ore_robots: tick.ore_robots + 1,
                clay_robots: tick.clay_robots,
                obsidian_robots: tick.obsidian_robots,
            });
        }
    }

    max
}

fn part1(input: &TInput) -> usize {
    input
        .iter()
        .map(|blueprint| (blueprint.num, evaluate(blueprint, 24)))
        .map(|(n, b)| n * b)
        .sum()
}

fn part2(input: &TInput) -> usize {
    input
        .iter()
        .take(3)
        .map(|blueprint| (blueprint.num, evaluate(blueprint, 32)))
        .map(|(_, b)| b)
        .product()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            line.split(" ")
                .filter_map(|w| w.trim_end_matches(':').parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .map(|nums| Blueprint {
            num: nums[0],
            ore: cost(nums[1], 0, 0),
            clay: cost(nums[2], 0, 0),
            obsidian: cost(nums[3], nums[4], 0),
            geode: cost(nums[5], 0, nums[6]),
        })
        .collect();

    let answer1 = part1(&input);
    let answer2 = part2(&input);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}
