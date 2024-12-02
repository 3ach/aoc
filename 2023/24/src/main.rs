use adventage::{part1demo, part2demo, day};
use std::collections::HashSet;

part1demo!(
    "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3", 2);

part2demo!(
    "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3", 47);

day!(2023, 24);

type Point = (i128, i128, i128);
type Velocity = (i128, i128, i128);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Particle {
    t0: Point,
    v: Velocity,
}

fn str_to_tuple(input: &str) -> (i128, i128, i128) {
    let parts = input.split(", ").map(|i| i.parse::<i128>().unwrap()).collect::<Vec<_>>();
    (parts[0], parts[1], parts[2])
}

fn float(input: (i128, i128, i128)) -> (f64, f64, f64) {
    (input.0 as f64, input.1 as f64, input.2 as f64)
}

fn parse(input: &str) -> Vec<Particle> {
    input
        .lines()
        .map(|l| {
            let (point, velocity) = l.split_once(" @ ").unwrap();
            let point: Point = str_to_tuple(point);
            let velocity: Velocity = str_to_tuple(velocity);
            Particle { t0: point, v: velocity }
        })
        .collect()
}

fn part1(particles: &Vec<Particle>) -> u32 {
    let mut crossing = 0;
    let min = if particles.len() > 15 { 200000000000000. } else { 7. };
    let max = if particles.len() > 15 { 400000000000000. } else { 27. }; 
    for a in particles {
        for b in particles {
            if a == b {
                continue;
            }

            let (x1, y1, _) = float(a.t0);
            let (mx1, my1, _) = float(a.v);
            let (x2, y2, _) = float(b.t0);
            let (mx2, my2, _) = float(b.v);
            let m1 = my1 / mx1;
            let m2 = my2 / mx2;

            let x = ((m1 * x1) - (m2 * x2) - y1 + y2) / (m1 - m2);
            let y = (m1 * (x - x1)) + y1;
            let t1 = (x - x1) / mx1;
            let t2 = (x - x2) / mx2;

            if t1.min(t2) >= 0. && x >= min && x <= max && y >= min && y <= max {
                crossing += 1;
            }
        }
    }

    crossing / 2
}

const MAX_VELOCITY: i128 = 1000;

fn part2(particles: &Vec<Particle>) -> i128 {
    let mut velocity_candidates: (Option<HashSet<i128>>, Option<HashSet<i128>>, Option<HashSet<i128>>) = (None, None, None);

    for a in particles {
        for b in particles {
            if a == b {
                continue;
            }

            if a.v.0 == b.v.0 {
                let distance = a.t0.0.abs_diff(b.t0.0) as i128;
                let possible_velocities = (-MAX_VELOCITY..=MAX_VELOCITY)
                    .filter(|v| *v != a.v.0 )
                    .filter(|v| distance % (a.v.0 - v) == 0)
                    .collect::<HashSet<i128>>();
                if let Some(candidates) = velocity_candidates.0 {
                    velocity_candidates.0 = Some(candidates.intersection(&possible_velocities).cloned().collect());
                } else {
                    velocity_candidates.0 = Some(possible_velocities);
                }
            } else if a.v.1 == b.v.1 {
                let distance = a.t0.1.abs_diff(b.t0.1) as i128;
                let possible_velocities = (-MAX_VELOCITY..=MAX_VELOCITY)
                    .filter(|v| *v != a.v.1 )
                    .filter(|v| distance % (a.v.1 - v) == 0)
                    .collect::<HashSet<i128>>();
                if let Some(candidates) = velocity_candidates.1 {
                    velocity_candidates.1 = Some(candidates.intersection(&possible_velocities).cloned().collect());
                } else {
                    velocity_candidates.1 = Some(possible_velocities);
                }
            } else if a.v.2 == b.v.2 {
                let distance = a.t0.2.abs_diff(b.t0.2) as i128;
                let possible_velocities = (-MAX_VELOCITY..=MAX_VELOCITY)
                    .filter(|v| *v != a.v.2 )
                    .filter(|v| distance % (a.v.2 - v) == 0)
                    .collect::<HashSet<i128>>();
                if let Some(candidates) = velocity_candidates.2 {
                    velocity_candidates.2 = Some(candidates.intersection(&possible_velocities).cloned().collect());
                } else {
                    velocity_candidates.2 = Some(possible_velocities);
                }
            }
        }
    }

    let velocity_candidates = (velocity_candidates.0.unwrap(), velocity_candidates.1.unwrap(), velocity_candidates.2.unwrap());

    for vx in &velocity_candidates.0 {
        for vy in &velocity_candidates.1 {
            for vz in &velocity_candidates.2 {
                let vx = *vx as f64;
                let vy = *vy as f64;
                let vz = *vz as f64;

                let a = particles[0];
                let b = particles[1];
                let c = particles[2];

                let (x1, y1, z1) = float(a.t0);
                let (mx1, my1, mz1) = float(a.v);
                let (x2, y2, z2) = float(b.t0);
                let (mx2, my2, mz2) = float(b.v);
                let (x3, y3, z3) = float(c.t0);
                let (mx3, my3, mz3) = float(c.v);

                let m1 = (my1 - vy) / (mx1 - vx);
                let m2 = (my2 - vy) / (mx2 - vx);
                let m3 = (my3 - vy) / (mx3 - vx);

                let x12 = (((m1 * x1) - (m2 * x2) - y1 + y2) / (m1 - m2)).round();
                let x23 = (((m2 * x2) - (m3 * x3) - y2 + y3) / (m2 - m3)).round();
                if x12 != x23 {
                    continue;
                }

                let y12 = (m1 * (x12 - x1)) + y1;
                let y23 = (m2 * (x23 - x2)) + y2;
                if y12 != y23 {
                    continue;
                }

                let m1 = (mz1 - vz) / (mx1 - vx);
                let m2 = (mz2 - vz) / (mx2 - vx);
                let m3 = (mz3 - vz) / (mx3 - vx);

                let x12 = (((m1 * x1) - (m2 * x2) - z1 + z2) / (m1 - m2)).round();
                let x23 = (((m2 * x2) - (m3 * x3) - z2 + z3) / (m2 - m3)).round();
                if x12 != x23 {
                    continue;
                }

                let z12 = (m1 * (x12 - x1)) + z1;
                let z23 = (m2 * (x23 - x2)) + z2;

                if z12 != z23 {
                    continue;
                }


                return x12 as i128 + y12 as i128 + z12 as i128;
            }
        }
    }

    0
}
