use std::io::BufRead;
use std::io;
use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug,Clone,Copy)]
enum Size {
    Big,
    Small,
}

impl Into<Size> for &String {
    fn into(self) -> Size {
        let lowercase = self.to_lowercase();
        if self.eq(&lowercase) {
            return Size::Small;
        } 

        return Size::Big;
    }
}

type Input = HashMap<String, HashSet<String>>;

fn part1(map: &Input) -> usize {
    let mut to_explore: Vec<Vec<String>> = vec![vec!["start".to_string()]];
    let mut paths: HashSet<Vec<String>> = HashSet::new();

    while to_explore.len() > 0 {
        let current = to_explore.pop().unwrap();
        let cave = current.last().unwrap();

        if cave.eq("end") {
            paths.insert(current);
            continue;
        }

        for next in &map[cave] {
            let size: Size = next.into();
            let visited = current.contains(next);

            let mut potential: Vec<String> = current.iter().cloned().collect();
            potential.push(next.clone());

            match size {
                Size::Big => to_explore.push(potential),
                Size::Small if !visited => to_explore.push(potential),
                _ => continue,
            }
        }
    }

    return paths.len();
}

fn part2(map: &Input) -> usize {
    let mut to_explore: Vec<(Vec<String>, bool)> = vec![(vec!["start".to_string()], false)];
    let mut paths: HashSet<Vec<String>> = HashSet::new();

    while to_explore.len() > 0 {
        let (current, duped) = to_explore.pop().unwrap();
        let cave = current.last().unwrap();

        if cave.eq("end") {
            paths.insert(current);
            continue;
        }

        for next in &map[cave] {
            if next.eq("start") {
                continue;
            }

            let size: Size = next.into();
            let visited = current.contains(next);
            let will_be_duped = duped || visited;

            let mut potential: Vec<String> = current.iter().cloned().collect();
            potential.push(next.clone());

            match size {
                Size::Big => to_explore.push((potential, duped)),
                Size::Small if !visited || !duped => to_explore.push((potential, will_be_duped)),
                _ => continue,
            }
        }
    }

    return paths.len();
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let tunnels: Vec<(String, String)> = reader
        .lines()
        .map(|line| {
            let line = line.expect("Can't read stdin");
            let mut termini = line.split("-");
            (termini.next().unwrap().to_string(), termini.next().unwrap().to_string())
        }).collect();

    let mut caves = HashMap::new();

    for tunnel in tunnels {
        caves.entry(tunnel.0.clone()).or_insert_with(HashSet::new).insert(tunnel.1.clone());
        caves.entry(tunnel.1.clone()).or_insert_with(HashSet::new).insert(tunnel.0.clone());
    }

	let answer1 = part1(&caves);
	let answer2 = part2(&caves);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
