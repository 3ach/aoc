use std::io::BufRead;
use std::io;
use std::convert::From;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::cmp;
use std::iter::FromIterator;


type Beacon = (i32, i32, i32);

#[derive(Debug,Clone)]
struct Scanner {
    beacons: Vec<Beacon>,
    distances: HashMap<(usize, usize), i64>
}

const ROTATIONS: [[[i32; 3]; 3]; 24] = [[[1, 0, 0], [0, 1, 0], [0, 0, 1]],
                                       [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
                                       [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
                                       [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
                                       [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
                                       [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
                                       [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
                                       [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
                                       [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
                                       [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
                                       [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
                                       [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
                                       [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
                                       [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
                                       [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
                                       [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
                                       [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
                                       [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
                                       [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
                                       [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
                                       [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
                                       [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
                                       [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
                                       [[0, 1, 0], [0, 0, -1], [-1, 0, 0]]];

fn rotate(points: &HashSet<Beacon>, rotation: [[i32; 3]; 3]) -> HashSet<Beacon> {
    points.iter().map(|(x, y, z)| {
        (((rotation[0][0] * x) + (rotation[0][1] * y) + (rotation[0][2] * z)),
         ((rotation[1][0] * x) + (rotation[1][1] * y) + (rotation[1][2] * z)),
         ((rotation[2][0] * x) + (rotation[2][1] * y) + (rotation[2][2] * z)))
    }).collect()
}

fn find_transform(reference: &HashSet<Beacon>, other: &HashSet<Beacon>) -> Option<(Beacon, [[i32; 3]; 3])> {
    let threshold = (0.5 * other.len() as f32) as usize;

    for rotation in ROTATIONS {
        let rotated = rotate(other, rotation);
        let reference_beacon = reference.iter().cloned().next().unwrap();

        let possible_shifts: Vec<Beacon> = rotated.iter().copied().map(|(x, y, z)| (reference_beacon.0 - x, reference_beacon.1 - y, reference_beacon.2 - z)).collect();

        for translate in possible_shifts {
            let transformed: HashSet<Beacon> = rotated.iter().cloned().map(|(x, y, z)| (x + translate.0, y + translate.1, z + translate.2)).collect();
            let matches = transformed.intersection(&reference).count();
            if matches > threshold  {
                return Some((translate, rotation));
            }
        }
    }

    None
}


fn coalesce(scanners: &Vec<Scanner>) -> (Scanner, Vec<Beacon>) {
    let mut coalesced = scanners.clone();
    let mut scanners_pos:  Vec<Beacon> = vec![];
    let mut scanner_idx = 0;
    let mut coalesced_len = coalesced.len();

    while coalesced.len() > 1 {
        for other_idx in scanner_idx+1..coalesced_len {
            let other = coalesced[other_idx].clone();
            let scanner = &mut coalesced[scanner_idx];

            let other_distances: HashSet<i64> = other.distances.values().cloned().collect();
            let scanner_distances: HashSet<i64> = scanner.distances.values().cloned().collect();

            let distances_in_common: HashSet<&i64> = scanner_distances.intersection(&other_distances).collect();
            if distances_in_common.len() >= 66 {
                let scanner_point_indexes: HashSet<usize> = scanner.distances.iter()
                    .filter(|(_, distance)| distances_in_common.contains(distance))
                    .flat_map(|((a, b), _)| [a, b])
                    .copied()
                    .collect();

                let other_point_indexes: HashSet<usize> = other.distances.iter()
                    .filter(|(_, distance)| distances_in_common.contains(distance))
                    .flat_map(|((a, b), _)| [a, b])
                    .copied()
                    .collect();

                let mut other_points: HashSet<Beacon> = other.beacons.iter()
                    .enumerate()
                    .filter(|(idx, _)| other_point_indexes.contains(&idx))
                    .map(|(_, b)| b)
                    .cloned()
                    .collect();

                let mut scanner_points: HashSet<Beacon> = scanner.beacons.iter()
                    .enumerate()
                    .filter(|(idx, _)| scanner_point_indexes.contains(&idx))
                    .map(|(_, b)| b)
                    .cloned()
                    .collect();

                if let Some((translation, rotation)) = find_transform(&scanner_points, &other_points) {
                    other_points = other.beacons.iter().cloned().collect();
                    scanner_points = scanner.beacons.iter().cloned().collect();

                    let other_rotated = rotate(&other_points, rotation);
                    let fully_transformed: Vec<Beacon> = other_rotated.iter()
                        .map(|(x, y, z)| (x + translation.0, y + translation.1, z + translation.2)).collect();


                    for new in fully_transformed {
                        if scanner_points.contains(&new) {
                            continue
                        }

                        let current_scanners = scanner.beacons.len();
                        for idx in 0..current_scanners {
                            let d = distance(scanner.beacons[idx], new); 
                            scanner.distances.insert((current_scanners, idx), d);
                        }

                        scanner.beacons.push(new);
                    }
                    
                    coalesced.remove(other_idx);
                    scanners_pos.push(translation);
                    break;
                }
            }
        }

        if coalesced_len == coalesced.len() {
            scanner_idx = (scanner_idx + 1) % coalesced_len;
        } else {
            coalesced_len = coalesced.len();
        }
    }

    (coalesced[0].clone(), scanners_pos)
}

fn part1(map: &Scanner) -> usize {
    map.beacons.len()
}

fn part2(scanners: &Vec<Beacon>) -> usize {
    let mut max = 0;

    for a in 0..scanners.len() {
        for b in a+1..scanners.len() {
            let a = scanners[a];
            let b = scanners[b];
            
            let manhattan = (((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs())) as usize;
            if manhattan > max {
                max = manhattan
            }
        }
    }

    max
}

fn distance(b1: Beacon, b2: Beacon) -> i64 {
    ((((b1.0 - b2.0).pow(2) + (b1.1 - b2.1).pow(2) + (b1.2 - b2.2).pow(2)) as f32).sqrt() * 1024. * 1024.) as i64
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut scanners: Vec<Scanner> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with("---") {
            scanners.push(Scanner {
                beacons: vec![],
                distances: HashMap::new(),
            });

            continue;
        } else if line.len() == 0 {
            continue;
        }

        let point: Vec<i32> = line.split(",").map(|coord| coord.parse().unwrap()).collect();
        if let Some(scanner) = scanners.last_mut() {
            let beacon = (point[0], point[1], point[2]);
            let index = scanner.beacons.len();

            for a in 0..index {
                let d = distance(scanner.beacons[a], beacon);
                scanner.distances.insert((a, index), d);
            }

            scanner.beacons.push(beacon);
        }
    }

    let (map, scanner_pos) = coalesce(&scanners);

	let answer1 = part1(&map);
	let answer2 = part2(&scanner_pos);

	println!("Answer 1: {:?}", answer1);
	println!("Answer 2: {:?}", answer2);

    Ok(())
}
