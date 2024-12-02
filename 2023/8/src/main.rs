use num::Integer;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

type Direction = String;

struct Node {
    left: String,
    right: String,
}

fn run(start: &str, direction: &Direction, nodes: &HashMap<String, Node>) -> u128 {
    let mut current = start;
    let mut steps = 0;
    let mut directions = direction.chars().cycle();

    while !current.ends_with("Z") {
        steps += 1;
        let direction = directions.next().unwrap();

        current = match direction {
            'L' => &nodes.get(current).unwrap().left,
            'R' => &nodes.get(current).unwrap().right,
            _ => panic!(),
        }
    }

    steps
}

fn part1(direction: &Direction, nodes: &HashMap<String, Node>) -> u128 {
    run("AAA", direction, nodes)
}

fn part2(direction: &Direction, nodes: &HashMap<String, Node>) -> u128 {
    nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|start| run(start, direction, nodes))
        .reduce(|acc, n| acc.lcm(&n))
        .unwrap()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let direction = lines.next().unwrap().expect("Couldn't read stdin");

    let nodes = lines
        .skip(1)
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            let node = String::from(&line[0..3]);
            let left = String::from(&line[7..10]);
            let right = String::from(&line[12..15]);

            (node, Node { left, right })
        })
        .collect::<HashMap<String, Node>>();

    let answer1 = part1(&direction, &nodes);
    let answer2 = part2(&direction, &nodes);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}
