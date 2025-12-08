use adventage::{day, part1demo, part2demo};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

day!(2025, 8);
part1demo!(
    "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
    40
);
part2demo!(
    "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
    25272
);

type Point = (u64, u64, u64);
type TInput = Vec<Point>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Connection {
    from: Point,
    to: Point,
}

impl Connection {
	fn distance(&self) -> u64 {
        self.from.0.abs_diff(self.to.0).pow(2)
        + self.from.1.abs_diff(self.to.1).pow(2)
        + self.from.2.abs_diff(self.to.2).pow(2)
	}
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance().cmp(&self.distance())
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> TInput {
    input .lines()
        .map(|line| {
            let coords: Vec<u64> = line.split(",").map(|c| c.parse().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        })
        .collect()
}

fn part1(points: &TInput) -> usize {
    let mut potential = BinaryHeap::new();

    for from_idx in 0..points.len() {
        for to_idx in (from_idx + 1)..points.len() {
            potential.push(Connection { from: points[from_idx].clone(), to: points[to_idx].clone() });
        }
    }

    let to_make = if points.len() < 30 {
        10
    } else {
        1000
    };

    let mut connections: Vec<Connection> = vec![];
    for _ in 0..to_make {
        let connection = potential.pop().unwrap();
        connections.push(connection);
    }

    let mut circuits: Vec<HashSet<Point>> = vec![];
    for connection in connections {
        let mut inserted = None;
        let mut dupe = None;
        for (idx, circuit) in circuits.iter_mut().enumerate() {
            if circuit.contains(&connection.from) || circuit.contains(&connection.to) {
                circuit.insert(connection.from);
                circuit.insert(connection.to);
                if inserted.is_some() {
                    dupe = Some(idx);
                } else {
                    inserted = Some(idx);
                }
            }
        }

        if let Some(inserted) = inserted {
            if let Some(dupe) = dupe {
                let dupe = circuits.remove(dupe);
                for point in dupe {
                    circuits[inserted].insert(point);
                }
            }
        } else {
            let mut circuit = HashSet::new();
            circuit.insert(connection.from);
            circuit.insert(connection.to);
            circuits.push(circuit);
        }
    }

    let mut circuit_lengths = circuits.iter()
        .map(|c| c.len())
        .collect::<Vec<usize>>();

    circuit_lengths.sort();
    circuit_lengths.iter()
        .rev()
        .take(3)
        .product()
}

fn part2(points: &TInput) -> u64 {
    let mut potential = BinaryHeap::new();

    for from_idx in 0..points.len() {
        for to_idx in (from_idx + 1)..points.len() {
            potential.push(Connection { from: points[from_idx].clone(), to: points[to_idx].clone() });
        }
    }

    let mut circuits: Vec<HashSet<Point>> = vec![];
    while let Some(connection) = potential.pop() {
        let mut inserted = None;
        let mut dupe = None;
        for (idx, circuit) in circuits.iter_mut().enumerate() {
            if circuit.contains(&connection.from) || circuit.contains(&connection.to) {
                circuit.insert(connection.from);
                circuit.insert(connection.to);

                if circuit.len() == points.len() {
                    return connection.from.0 * connection.to.0;
                }

                if inserted.is_some() {
                    dupe = Some(idx);
                } else {
                    inserted = Some(idx);
                }
            }
        }

        if let Some(inserted) = inserted {
            if let Some(dupe) = dupe {
                let dupe = circuits.remove(dupe);
                for point in dupe {
                    circuits[inserted].insert(point);
                }

                if circuits[inserted].len() == points.len() {
                    return connection.from.0 * connection.to.0;
                }
            }
        } else {
            let mut circuit = HashSet::new();
            circuit.insert(connection.from);
            circuit.insert(connection.to);
            circuits.push(circuit);
        }
    }   

    panic!();
}
