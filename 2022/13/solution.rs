#![feature(if_let_guard)]

use std::io::BufRead;
use std::io;
use std::cmp::{min, Ordering};
use std::fmt;

#[derive(Clone)]
enum Packet {
    List(Vec<Packet>),
    Num(u32)
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Num(val) => write!(f, "{}", val),
            Packet::List(list) => write!(f, "[{}]", list.iter().map(|x| format!("{:?}", x)).collect::<Vec<String>>().join(","))
        }
    }
}

type TInput = Vec<(Packet, Packet)>;

fn check_nums(left: &u32, right: &u32) -> Option<bool> {
    if left != right {
        Some(left < right)
    } else {
        None
    }
}

fn check_lists(left: &Vec<Packet>, right: &Vec<Packet>) -> Option<bool> {

    for idx in 0..min(left.len(), right.len()) {
        let left = &left[idx];
        let right = &right[idx];

        match (left, right) {
            (Packet::Num(l), Packet::Num(r)) if let Some(result) = check_nums(l, r) => return Some(result),
            (Packet::Num(_), Packet::List(r)) if let Some(result) = check_lists(&vec![left.clone()], r) => return Some(result),
            (Packet::List(l), Packet::Num(_)) if let Some(result) = check_lists(l, &vec![right.clone()]) => return Some(result),
            (Packet::List(l), Packet::List(r)) if let Some(result) = check_lists(l, r) => return Some(result),
            _ => continue,
        }
    }

    if left.len() != right.len() {
        return Some(left.len() < right.len())
    }

    None
}

fn check(left: &Packet, right: &Packet) -> bool {
    match (left, right) {
        (Packet::List(l), Packet::List(r)) => check_lists(l, r).unwrap(),
        _ => panic!()
    }
}

fn part1(input: &TInput) -> usize {
    input.iter()
        .enumerate()
        .filter_map(|(idx, pair)| if check(&pair.0, &pair.1) { Some(idx + 1) } else { None })
        .sum()
}

fn part2(input: &TInput) -> usize {
    let mut input: Vec<&Packet> = input
        .iter()
        .map(|(left, right)| vec![left, right])
        .flatten()
        .collect();

    let two = Packet::List(vec![Packet::List(vec![Packet::Num(2)])]);
    let six = Packet::List(vec![Packet::List(vec![Packet::Num(6)])]);

    input.push(&two);
    input.push(&six);

    input.sort_by(|a, b| if check(&a, &b) { Ordering::Less } else { Ordering::Greater });

    input.iter()
        .enumerate()
        .filter_map(|(idx, p)| if let Packet::List(l) = p { Some((idx, l)) } else { None })
        .filter(|(_, l)| l.len() == 1) 
        .filter_map(|(idx, l)| if let Packet::List(l) = &l[0] { Some((idx, l)) } else { None }) 
        .filter(|(_, l)| l.len() == 1) 
        .filter_map(|(idx, p)| if let Packet::Num(n) = &p[0] && (*n == 2 || *n == 6) { Some(idx) } else { None })
        .map(|(idx)| idx + 1)
        .product()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut input = vec![];
    let mut item = (None, None);

    for line in reader.lines() {
        let line = line.expect("Couldn't read stdin");
        
        let mut stack = vec![];
        let mut current = Packet::List(vec![]);
        let mut in_num = false;

        for c in line.chars() {
            if c == '[' {
                stack.push(current);
                current = Packet::List(vec![]);
            } else if c == ']' {
                in_num = false;
                let mut last = stack.pop().unwrap();

                if let Packet::List(ref mut contents) = last {
                    contents.push(current);
                    current = last;
                } else {
                    panic!("tried to pop a single value off the stack");
                }
            } else if c == ',' {
                in_num = false;
                continue;
            } else {
                let mut num = (c as u8 - '0' as u8) as u32;
                if let Packet::List(ref mut current) = current {
                    if in_num {
                        if let Packet::Num(prev) = current.pop().unwrap() {
                            num = (prev * 10) + num;
                        } else {
                            panic!();
                        }
                    } else {
                        in_num = true;
                    }

                    current.push(Packet::Num(num));
                }
            }
        }

        if let None = item.0 {
            item.0 = Some(current);
        } else if let None = item.1 {
            item.1 = Some(current);
        } else {
            input.push((item.0.unwrap(), item.1.unwrap()));
            item = (None, None);
        }
    }

    if let (Some(left), Some(right)) = item {
        input.push((left, right));
    }

	let answer1 = part1(&input);
	let answer2 = part2(&input);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
