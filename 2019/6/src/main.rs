use std::io::BufRead;
use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

type Map = HashMap<String, (HashSet<String>, HashSet<String>)>;

fn part1(map: &Map) -> usize {
    let mut orbits = 0;
    let mut unprocessed = vec![(0, "COM")];

    while let Some((count, current)) = unprocessed.pop() {
        orbits += count;
        if let Some((orbiting, _)) = map.get(current) {
            for orbiter in orbiting {
                unprocessed.push((count + 1, orbiter));
            }
        }
    }

    orbits
}

fn path<'a>(start: &'a str, map: &'a Map) -> Vec<&'a str> {
    let mut current = start;
    let mut path = vec![];

    while current != "COM" {
        let (_, center) = map.get(current).unwrap();
        path.push(current);
        current = center.iter().next().unwrap();
    }

    path
}

fn part2(map: &Map) -> usize {
    let my_path = path("YOU", map);
    let santa_path = path("SAN", map);

    let common: HashSet<_> = my_path
        .iter()
        .rev()
        .zip(santa_path.iter().rev())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect();

    (my_path.iter().filter(|x| !common.contains(*x)).count() - 1) +
        (santa_path.iter().filter(|x| !common.contains(*x)).count() - 1)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Couldn't read stdin.");
        let parts: Vec<&str> = line.split(")").collect();
        let object = String::from(parts[0]);
        let orbiter = String::from(parts[1]);

        let (orbiting, _) = map.entry(object.clone()).or_insert((HashSet::new(), HashSet::new()));
        orbiting.insert(orbiter.clone());

        let (_, reverse) = map.entry(orbiter.clone()).or_insert((HashSet::new(), HashSet::new()));
        reverse.insert(object.clone());
    }

	let answer1 = part1(&map);
	let answer2 = part2(&map);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
