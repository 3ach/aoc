use std::io::BufRead;
use std::io;

#[derive(Debug,Clone)]
struct Move {
    from: usize,
    to: usize, 
    count: usize
}


fn part1(stacks: &Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    let mut stacks = stacks.clone();
    
    for mov in moves {
        for _ in 0..mov.count {
            let c = stacks[mov.from].pop().unwrap();
            stacks[mov.to].push(c);
        }
    }

    let mut ret = String::new();

    for mut stack in stacks {
        ret.push(stack.pop().unwrap());
    }

    ret
}

fn part2(stacks: &Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    let mut stacks = stacks.clone();
    
    for mov in moves {
        let mut claw = vec![];
        for _ in 0..mov.count {
            let c = stacks[mov.from].pop().unwrap();
            claw.insert(0, c);
        }
        
        stacks[mov.to].append(&mut claw);
    }

    let mut ret = String::new();

    for mut stack in stacks {
        ret.push(stack.pop().unwrap());
    }

    ret
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();
    let mut stacks = vec![];

    loop {
        buf = String::new();
        stdin.read_line(&mut buf);
        let mut buf = buf.trim_end();
        if buf.len() == 0 {
            break;
        }

        let level = (buf.len() + 1) / 4;

        if stacks.len() < level {
            for _ in 0..(level - stacks.len()) {
                stacks.push(vec![]);
            }
        }

        for stack in 0..level {
            let crat_idx = (stack * 4) + 1;
            let crat = buf.as_bytes()[crat_idx] as char;

            if !crat.is_alphabetic() {
                continue;
            }
            stacks[stack].insert(0, crat);
        }
    }

    let mut moves = vec![];

    while let Ok(b) = stdin.read_line(&mut buf) {
        if b == 0 {
            break;
        }

        let mut parts = buf.trim_end()
            .split(" ")
            .filter(|word| word.chars().all(|c| c.is_numeric()))
            .map(|word| word.parse().unwrap());


        moves.push(Move { count: parts.next().unwrap(), from: parts.next().unwrap() - 1, to: parts.next().unwrap() - 1});
        buf = String::new();
    }

	let answer1 = part1(&stacks, &moves);
	let answer2 = part2(&stacks, &moves);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
