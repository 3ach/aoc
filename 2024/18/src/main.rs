use adventage::{day, part1demo, part2demo};
use std::collections::{HashSet, VecDeque};

day!(2024, 18);
part1demo!(
    "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
    22
);
part2demo!(
    "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
"6,1"
);

type Byte = (i32, i32);
type TInput = Vec<Byte>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn neighbors(byte: &Byte) -> Vec<Byte> {
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .iter()
        .map(|(dx, dy)| (byte.0 + *dx, byte.1 + *dy))
        .collect()
}

fn explore_at_time(bytes: &TInput, time: usize) -> Option<u32> {
    let (x, y) = if bytes.len() > 1000 {
        (71, 71)
    } else {
        (7, 7)
    };

    let mut mem: HashSet<Byte> = (0..x)
        .map(|x| (0..y).map(move |y| (x, y)))
        .flatten()
        .collect();

    for byte in bytes.iter().take(time) {
        mem.remove(byte);
    }

    let mut to_explore = VecDeque::from([((0, 0), 0)]);
    let mut explored = HashSet::new();
    while let Some((current, distance)) = to_explore.pop_back() {
        if !explored.insert(current) {
            continue;
        }

        if current == (x - 1, y - 1) {
            return Some(distance);
        }

        for neighbor in neighbors(&current) {
            if !explored.contains(&neighbor) && mem.contains(&neighbor) {
                to_explore.push_front((neighbor, distance + 1));
            }
        }
    }

    None
}

fn part1(bytes: &TInput) -> u32 {
    let to_simulate = if bytes.len() > 1000 {
        1024
    } else {
        12
    };

    explore_at_time(bytes, to_simulate).unwrap()
}

fn part2(bytes: &TInput) -> String {
    let mut possible = 0;
    let mut impossible = bytes.len() - 1;

    while impossible > (possible + 1) {
        let midpoint = (possible + impossible) / 2;
        if let Some(_) = explore_at_time(&bytes, midpoint) {
            possible = midpoint;
        } else {
            impossible = midpoint;
        }
    }

    format!("{},{}", bytes[possible].0, bytes[possible].1)
}
