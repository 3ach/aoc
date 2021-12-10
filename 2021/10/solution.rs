use std::io::BufRead;
use std::io;
use std::convert::TryInto;

fn part1(chunks: &Vec<Vec<char>>) -> usize {
    let mut score = 0;

    'checkline: for chunk in chunks.iter() {
        let mut stack: Vec<char> = vec![];

        for c in chunk.iter() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(open_to_close(*c)),
                ')' | ']' | '}' | '>' => {
                    if stack.pop().unwrap_or(' ') != *c {
                        score += syntax_score(*c);
                        continue 'checkline;
                    }
                },
                _ => panic!()
            }
        }
    }

    return score;
}



fn part2(chunks: &Vec<Vec<char>>) -> usize {
    let mut scores: Vec<usize> = vec![];

    'checkline: for chunk in chunks.iter() {
        let mut stack: Vec<char> = vec![];
        let mut score: usize = 0;

        for c in chunk.iter() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(open_to_close(*c)),
                ')' | ']' | '}' | '>' => {
                    if stack.pop().unwrap_or(' ') != *c {
                        score += syntax_score(*c);
                        continue 'checkline;
                    }
                },
                _ => panic!()
            }
        }

        while stack.len() > 0 {
            let missing = stack.pop().unwrap();
            score *= 5;
            score += completion_score(missing);
        }

        if score > 0 {
            scores.push(score);
        }
    }

    scores.sort();
    return scores[scores.len() / 2];
}

fn open_to_close(open: char) -> char {
    match open {
        '(' => ')',
        '<' => '>',
        '{' => '}',
        '[' => ']',
        _ => panic!(),
    }
}

fn syntax_score(open: char) -> usize {
    match open {
        '(' | ')' => 3,
        '<' | '>' => 25137,
        '{' | '}' => 1197,
        '[' | ']' => 57,
        _ => panic!(),
    }
}

fn completion_score(open: char) -> usize {
    match open {
        '(' | ')' => 1,
        '<' | '>' => 4,
        '{' | '}' => 3,
        '[' | ']' => 2,
        _ => panic!(),
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let chunks = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

	let answer1 = part1(&chunks);
	let answer2 = part2(&chunks);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
