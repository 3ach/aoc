use std::io::BufRead;
use std::io;

#[derive(Debug)]
enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}


fn part1(commands: &Vec<Command>) -> i32 {
    let mut depth: i32 = 0;
    let mut horiz: i32 = 0;

    for command in commands {
        match command {
            Command::Up(distance) => depth -= distance,
            Command::Down(distance) => depth += distance,
            Command::Forward(distance) => horiz += distance,
        }
    }
   
    return depth * horiz;
}

fn part2(commands: &Vec<Command>) -> i32 {
    let mut depth: i32 = 0;
    let mut horiz: i32 = 0;
    let mut aim: i32 = 0;

    for command in commands {
        match command {
            Command::Up(distance) => aim += -1 * distance,
            Command::Down(distance) => aim += 1 * distance,
            Command::Forward(distance) => {
                horiz += distance;
                depth += aim * distance
            },
        }
    }
   
    return depth * horiz;
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

	let commands: Vec<Command> = reader.lines()
        .map(|line| line.expect("couldn't read stdin"))
        .map(|line| {
            let mut tok = line.split_whitespace();

            let direction = tok.next().unwrap();
            let distance: i32 = tok.next().unwrap().parse().unwrap();
            match direction {
                "up" => Command::Up(distance),
                "down" => Command::Down(distance),
                "forward" => Command::Forward(distance),
                _ => panic!("Unknown command!")
            }
        })
        .collect();

	let answer1 = part1(&commands);
	let answer2 = part2(&commands);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
