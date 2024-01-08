use adventage::{part1demo, part2demo, day};
use std::collections::HashMap;
use num::Integer;

part1demo!(
    "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>", 179);

part2demo!(
    "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>", 2772);

part1demo!("<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>", 1940);

part2demo!("<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>", 4686774924);

day!(2019, 12);

type Triple = (isize, isize, isize);
type Planet = (Triple, Triple);
type TInput = Vec<Planet>;

fn parse_line(line: &str) -> Planet {
    let stripped = line.trim_matches('<')
        .trim_matches('>');

    let coords = stripped.split(", ")
        .map(|part| part.split_once("=").unwrap().1)
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    ((coords[0], coords[1], coords[2]), (0, 0, 0))
}

fn parse(input: &str) -> TInput {
    input
        .lines()
        .map(parse_line)
        .collect()
}

fn step(planets: &mut TInput) {
    for left_idx in 0..planets.len() {
        for right_idx in (left_idx + 1)..planets.len() {
            let left = planets[left_idx];
            let right = planets[right_idx];

            let mut changes = ((0, 0, 0), (0, 0, 0));

            if left.0.0 > right.0.0 {
                changes.0.0 -= 1;
                changes.1.0 += 1;
            } else if left.0.0 < right.0.0 {
                changes.0.0 += 1;
                changes.1.0 -= 1;
            }

            if left.0.1 > right.0.1 {
                changes.0.1 -= 1;
                changes.1.1 += 1;
            } else if left.0.1 < right.0.1 {
                changes.0.1 += 1;
                changes.1.1 -= 1;
            }

            if left.0.2 > right.0.2 {
                changes.0.2 -= 1;
                changes.1.2 += 1;
            } else if left.0.2 < right.0.2 {
                changes.0.2 += 1;
                changes.1.2 -= 1;
            }

            planets[left_idx].1.0 += changes.0.0;
            planets[left_idx].1.1 += changes.0.1;
            planets[left_idx].1.2 += changes.0.2;
            planets[right_idx].1.0 += changes.1.0;
            planets[right_idx].1.1 += changes.1.1;
            planets[right_idx].1.2 += changes.1.2;
        }
    }

    for planet_idx in 0..planets.len() {
        planets[planet_idx].0.0 += planets[planet_idx].1.0;
        planets[planet_idx].0.1 += planets[planet_idx].1.1;
        planets[planet_idx].0.2 += planets[planet_idx].1.2;
    }
}

fn energy(planets: &TInput) -> isize {
    planets.iter()
        .map(|(p, v)| (p.0.abs() + p.1.abs() + p.2.abs()) * (v.0.abs() + v.1.abs() + v.2.abs()))
        .sum()
}

fn part1(initial: &TInput) -> isize {
    let mut planets = initial.clone();
    let steps = if initial[0].0.0 == -1 {
        10
    } else if initial[0].0.0 == -8 {
        100
    } else {
        1000
    };

    for _ in 0..steps {
        step(&mut planets);
    }

    energy(&planets)
}

fn part2(initial: &TInput) -> u128 {
    let mut iterations = 0;
    let mut planets = initial.clone();
    let mut cycles = HashMap::new();
    let mut xycle = None;
    let mut yycle = None;
    let mut zycle = None;
    
    
    while xycle.is_none() || yycle.is_none() || zycle.is_none() {
        step(&mut planets);
        iterations += 1;

        if planets == *initial {
            return iterations;
        }

        let xkey = format!("x{}{}{}{}{}{}{}{}", planets[0].0.0, planets[0].1.0, planets[1].0.0, planets[1].1.0, planets[2].0.0, planets[2].1.0, planets[3].0.0, planets[3].1.0);
        let ykey = format!("y{}{}{}{}{}{}{}{}", planets[0].0.1, planets[0].1.1, planets[1].0.1, planets[1].1.1, planets[2].0.1, planets[2].1.1, planets[3].0.1, planets[3].1.1);
        let zkey = format!("z{}{}{}{}{}{}{}{}", planets[0].0.2, planets[0].1.2, planets[1].0.2, planets[1].1.2, planets[2].0.2, planets[2].1.2, planets[3].0.2, planets[3].1.2);

        if xycle.is_none() {
            if cycles.contains_key(&xkey) {
                xycle = Some(iterations - 1);
            } else {
                cycles.insert(xkey, iterations);
            }
        }

        if yycle.is_none() {
            if cycles.contains_key(&ykey) {
                yycle = Some(iterations - 1);
            } else {
                cycles.insert(ykey, iterations);
            }
        }

        if zycle.is_none() {
            if cycles.contains_key(&zkey) {
                zycle = Some(iterations - 1);
            } else {
                cycles.insert(zkey, iterations);
            }
        }
    }

    println!("{:?} {:?} {:?}", xycle, yycle, zycle);

    xycle.unwrap().lcm(&yycle.unwrap()).lcm(&zycle.unwrap())
}
