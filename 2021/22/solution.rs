use std::io::BufRead;
use std::io;
use std::cmp;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug,Clone)]
struct Step {
	on: bool,
	cuboid: Cuboid
}

#[derive(Debug,Clone)]
struct Cuboid {
	x: Range<isize>,
	y: Range<isize>,
	z: Range<isize>,
}

impl From<String> for Step {
	fn from(input: String) -> Self {
		let words: Vec<&str> = input.split(" ").collect();

		let value = match words[0] {
			"on" => true,
			"off" => false,
			_ => panic!(),
		};

		Step{
			on: value,
			cuboid: words[1].into(),
		}
	}
}

impl From<&str> for Cuboid {
	fn from(input: &str) -> Self {
		let mut ranges = input.split(",")
			.map(|range| range[2..].split(".."))
			.map(|mut ends| (ends.next().unwrap().parse::<isize>().unwrap(), ends.next().unwrap().parse::<isize>().unwrap()))
			.map(|(start, end)| start..end);

		Cuboid{
			x: ranges.next().unwrap(),
			y: ranges.next().unwrap(),
			z: ranges.next().unwrap(),
		}
	}
}

fn intersects(a: &Cuboid, b: &Cuboid) -> bool {
	cmp::min(a.x.end, b.x.end) >= cmp::max(a.x.start, b.x.start) &&
	cmp::min(a.y.end, b.y.end) >= cmp::max(a.y.start, b.y.start) &&
	cmp::min(a.z.end, b.z.end) >= cmp::max(a.z.start, b.z.start)
}

fn split_x(cuboid: &Cuboid, split: isize) -> Option<Vec<Cuboid>> {
	if split > cuboid.x.end || split <= cuboid.x.start {
		None
	} else {
		Some(vec![
			Cuboid{
				x: cuboid.x.start..split-1,
				y: cuboid.y.clone(),
				z: cuboid.z.clone(),
			},
			Cuboid{
				x: split..cuboid.x.end,
				y: cuboid.y.clone(),
				z: cuboid.z.clone(),
			},
		])
	}
}

fn split_y(cuboid: &Cuboid, split: isize) -> Option<Vec<Cuboid>> {
	if split > cuboid.y.end || split <= cuboid.y.start {
		None
	} else {
		Some(vec![
			Cuboid{
				x: cuboid.x.clone(),
				y: cuboid.y.start..split-1,
				z: cuboid.z.clone(),
			},
			Cuboid{
				x: cuboid.x.clone(),
				y: split..cuboid.y.end,
				z: cuboid.z.clone(),
			},
		])
	}
}

fn split_z(cuboid: &Cuboid, split: isize) -> Option<Vec<Cuboid>> {
	if split > cuboid.z.end || split <= cuboid.z.start {
		None
	} else {
		Some(vec![
			Cuboid{
				x: cuboid.x.clone(),
				y: cuboid.y.clone(),
				z: cuboid.z.start..split-1,
			},
			Cuboid{
				x: cuboid.x.clone(),
				y: cuboid.y.clone(),
				z: split..cuboid.z.end,
			},
		])
	}
}

fn intersection(a: &Cuboid, b: &Cuboid) -> Cuboid {
	if !intersects(&a, &b) {
		panic!();
	}

	Cuboid{
		x: cmp::max(a.x.start, b.x.start)..cmp::min(a.x.end, b.x.end),
		y: cmp::max(a.y.start, b.y.start)..cmp::min(a.y.end, b.y.end),
		z: cmp::max(a.z.start, b.z.start)..cmp::min(a.z.end, b.z.end),
	}
}

fn remove(cuboid: &Cuboid, to_remove: &Cuboid) -> Vec<Cuboid> {
	let mut remainder: Vec<Cuboid> = vec![];
	let mut to_split: Vec<Cuboid> = vec![cuboid.clone()];
	
	while to_split.len() > 0 {
		let cuboid = to_split.pop().unwrap();
		if !intersects(&cuboid, to_remove) {
			remainder.push(cuboid);
			continue
		}	

		let mut splits: Option<Vec<Cuboid>> = None;
	
		if cuboid.x.start < to_remove.x.start && cuboid.x.end >= to_remove.x.start {
			splits = split_x(&cuboid, to_remove.x.start);
		} else if cuboid.x.start <= to_remove.x.end && cuboid.x.end > to_remove.x.end {
			splits = split_x(&cuboid, to_remove.x.end + 1);
		} else if cuboid.y.start < to_remove.y.start && cuboid.y.end >= to_remove.y.start {
			splits = split_y(&cuboid, to_remove.y.start);
		} else if cuboid.y.start <= to_remove.y.end && cuboid.y.end > to_remove.y.end {
			splits = split_y(&cuboid, to_remove.y.end + 1);
		} else if cuboid.z.start < to_remove.z.start && cuboid.z.end >= to_remove.z.start {
			splits = split_z(&cuboid, to_remove.z.start);
		} else if cuboid.z.start <= to_remove.z.end && cuboid.z.end > to_remove.z.end {
			splits = split_z(&cuboid, to_remove.z.end + 1);
		}

		if let Some(mut splits) = splits {
			to_split.append(&mut splits);
		}
	}

	remainder
} 

fn part1(steps: &Vec<Step>) -> isize {
	let bounds = Cuboid{x: -50..50, y: -50..50, z: -50..50};
	let mut on: Vec<Cuboid> = vec![];
	let mut next_on: Vec<Cuboid> = vec![];

	for step in steps {
		if !intersects(&step.cuboid, &bounds) {
			continue;
		}

		let cuboid = intersection(&step.cuboid, &bounds);
		for other_cuboid in on.iter().cloned() {
			if intersects(&other_cuboid, &cuboid) {
				let mut remainders = remove(&other_cuboid, &cuboid);
				next_on.append(&mut remainders);
			} else {
				next_on.push(other_cuboid);
			}
		}

		if step.on {
			next_on.push(cuboid);
		}

		on = next_on.clone();
		next_on.truncate(0);

	}

	on.iter().map(|cuboid| {
		(cuboid.x.end - cuboid.x.start + 1) 
    	* (cuboid.y.end - cuboid.y.start + 1)
	 	* (cuboid.z.end - cuboid.z.start + 1)
	}).sum()
}

fn part2(steps: &Vec<Step>) -> isize {
	let mut on: Vec<Cuboid> = vec![];
	let mut next_on: Vec<Cuboid> = vec![];

	for step in steps {
		let cuboid = step.cuboid.clone();
		for other_cuboid in on.iter().cloned() {
			if intersects(&other_cuboid, &cuboid) {
				let mut remainders = remove(&other_cuboid, &cuboid);
				next_on.append(&mut remainders);
			} else {
				next_on.push(other_cuboid);
			}
		}

		if step.on {
			next_on.push(cuboid);
		}

		on = next_on.clone();
		next_on.truncate(0);
	}

	on.iter().map(|cuboid| {
		(cuboid.x.end - cuboid.x.start + 1) 
    	* (cuboid.y.end - cuboid.y.start + 1)
	 	* (cuboid.z.end - cuboid.z.start + 1)
	}).sum()
}


fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let reader = stdin.lock();

	let steps: Vec<Step> = reader.lines().map(|line| line.unwrap().into()).collect();

	let answer1 = part1(&steps);
	let answer2 = part2(&steps);

	println!("Answer 1: {:?}", answer1);
	println!("Answer 2: {:?}", answer2);

	Ok(())
}
