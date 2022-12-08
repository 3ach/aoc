use std::io::BufRead;
use std::io;
use std::collections::{HashMap, HashSet};

type TInput = Directory;

#[derive(Debug, Eq, PartialEq)]
struct Directory {
    name: String,
    files: HashMap<String, u32>,
    children: Vec<Directory>
}

fn sum(dir: &Directory) -> u32 {
    dir.files.values().sum::<u32>() + 
        dir.children.iter().map(sum).sum::<u32>()
}

fn part1(input: &TInput) -> u32 {
    0
}

fn part2(input: &TInput) -> u32 {
    0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let root = Directory { name: "/".to_string(), files: HashMap::new(), children: vec![] };
    let mut path: Vec<&Directory> = vec![];
    let mut current = root;

    for line in reader.lines() {
        let line = line.expect("Could not read stdin");
        if line == "$ cd /" {
            continue;
        }

        if line.starts_with("$") {
            let line = line.strip_prefix("$ ").unwrap();
            if let Some((_cd, arg)) = line.split_once(" ") {
                if arg == ".." {
                    current = *path.pop().unwrap();
                } else {
                    let next = current.children.iter_mut().filter(|c| c.name == arg).next().unwrap();
                    path.push(&current);
                    current = *next;
                }
            }
        } else {
            let (type_or_size, name) = line.split_once(" ").unwrap();

            if type_or_size == "dir" {
                current.children.push(Directory { 
                    name: name.to_string(), 
                    files: HashMap::new(),
                    children: vec![] 
                });
            } else {
                let size: u32 = type_or_size.parse().unwrap();
                current.files.insert(name.to_string(), size);
            }
        }
    }

    println!("{:?}", root);
	let answer1 = part1(&root);
	let answer2 = part2(&root);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
