use std::io::BufRead;
use std::io;
use std::collections::HashSet;
use std::collections::HashMap;
use std::convert::From;
use std::ops::RangeInclusive;

#[derive(Debug,Clone)]
struct Target {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
}

fn part1(target: &Target) -> isize {
    (target.y.start().abs() * (target.y.start().abs() - 1)) / 2
}

fn lands(x: isize, max_steps: Option<isize>, target: &Target) -> Vec<(isize, isize)> {
    let max_steps = match max_steps {
        None => target.y.start().abs() + 1000,
        Some(val) => val
    };

    let mut vecs: Vec<(isize, isize)> = vec![];
    for y in (*target.y.start()..target.y.start().abs()) {
        let mut ypos = 0;
        let mut xpos = 0;
        let mut yvel = y;
        let mut xvel = x;

        for _ in 0..max_steps {
            ypos += yvel;
            xpos += xvel;

            if target.y.contains(&ypos) && target.x.contains(&xpos) {
                vecs.push((x, y));
                break;
            }

            yvel -= 1;
            xvel = std::cmp::max(xvel - 1, 0);
        }
    }

    vecs
}

fn part2(target: &Target) -> usize {
    (0..=*target.x.end())
    .skip_while(|x| *target.x.start() > (x * (x + 1)) / 2)
    .filter_map(|x| {
        let mut sum = 0;

        for pt in (0..=x).rev() {
            if sum + pt > *target.x.end() {
                if sum < *target.x.start() {
                    return None
                }

                return Some((x, Some(x - pt)));
            }
            
            sum += pt;
        }

        return Some((x, None));
    })
    .flat_map(|(x, steps)| lands(x, steps, &target))
    .count()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf);
    let target_points: Vec<isize> = buf.trim()[13..]
        .split(", ")
        .flat_map(|range| {
            range[2..].split("..")
                .map(|num| num.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect();

    let target = Target{x: target_points[0]..=target_points[1], y: target_points[2]..=target_points[3]};

	let answer1 = part1(&target);
	let answer2 = part2(&target);

	println!("Answer 1: {:?}", answer1);
	println!("Answer 2: {:?}", answer2);

    Ok(())
}
