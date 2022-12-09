use std::io::BufRead;
use std::io;
use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down, 
    Left, 
    Right
}

#[derive(Debug)]
struct Move {
    dir: Direction,
    steps: usize
}

type TInput = Vec<Move>;

fn take_step(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    let mut next = tail;

    if head.0 == tail.0 && head.1.abs_diff(tail.1) <= 1 {
        return tail;
    }

    if head.1 == tail.1 && head.0.abs_diff(tail.0) <= 1 {
        return tail;
    }

    if head.0.abs_diff(tail.0) == 1 && head.1.abs_diff(tail.1) == 1 {
        return tail;
    }

    if head.0 > tail.0 {
        next.0 += 1;
    } else if head.0 < tail.0 {
        next.0 -= 1;
    }

    if head.1 > tail.1 {
        next.1 += 1;
    } else if head.1 < tail.1 {
        next.1 -= 1;
    }

    next
}

fn part1(input: &TInput) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(tail);

    for step in input {
        for _ in 0..step.steps {
            let next = match step.dir {
                Direction::Up => (head.0 - 1, head.1),
                Direction::Down => (head.0 + 1, head.1),
                Direction::Left => (head.0, head.1 - 1),
                Direction::Right => (head.0, head.1 + 1)
            };

            head = next;
            tail = take_step(head, tail);
            visited.insert(tail);
        }
    }

    visited.len()
}

fn part2(input: &TInput) -> usize {
    let mut rope = [(0, 0); 10];
    let mut visited = HashSet::new();
    visited.insert(rope[9]);

    for step in input {
        for _ in 0..step.steps {
            rope[0] = match step.dir {
                Direction::Up => (rope[0].0 - 1, rope[0].1),
                Direction::Down => (rope[0].0 + 1, rope[0].1),
                Direction::Left => (rope[0].0, rope[0].1 - 1),
                Direction::Right => (rope[0].0, rope[0].1 + 1)
            };

            for knot in 1..rope.len() {
                rope[knot] = take_step(rope[knot - 1], rope[knot]);
            }

            visited.insert(rope[9]);
        }
    }

    visited.len()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            let (dir, steps) = line.split_once(" ").unwrap();
            let steps: usize = steps.parse().unwrap();
            let dir = match dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!()
            };

            Move{dir, steps}
        })
        .collect();

	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
