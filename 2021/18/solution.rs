use std::io::BufRead;
use std::io;
use std::convert::From;
use std::ops::Add;
use std::fmt;

#[derive(Clone)]
enum Number {
    Single(u32),
    Pair(Box<Number>, Box<Number>),
}

#[derive(Debug,Clone)]
enum Explosion {
    Left(u32),
    Right(u32),
    Single(u32),
    Pair(u32, u32),
    Done,
}

impl fmt::Debug for Number {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Single(val) => fmt.write_fmt(format_args!("{}", val)),
            Number::Pair(left, right) => fmt.write_fmt(format_args!("[{:?}, {:?}]", left, right))
        }
    }
}

trait Reduce {
    fn reduce(&mut self);
}

trait Magnitude {
    fn magnitude(self) -> u32;
}

trait Explode {
    fn explode(&mut self) -> bool;
    fn explode_impl(&mut self, depth: usize) -> Option<Explosion>;
}

trait Absorb {
    fn absorb_left(&mut self, val: u32);
    fn absorb_right(&mut self, val: u32);
}

trait Split {
    fn split(&mut self) -> bool;
}

impl From<String> for Number {
    fn from(s: String) -> Number {
        let mut stack: Vec<Number> = vec![];

        for c in s.chars() {
            match c {
                '[' | ',' => continue,
                ']' => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    stack.push(Number::Pair(Box::new(left), Box::new(right)));
                },
                '0'..='9' => {
                    stack.push(Number::Single(c.to_digit(10).unwrap()));
                }
                _ => panic!()
            };
        }

        stack.pop().unwrap()
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut num = Number::Pair(Box::new(self), Box::new(other));
        num.reduce();
        num
    }
}

impl Split for Number {
    fn split(&mut self) -> bool {
        match self {
            Number::Single(val) if *val >= 10 => {
                let left = *val / 2;
                let right = *val - left;
                *self = Number::Pair(Box::new(Number::Single(left)), Box::new(Number::Single(right)));

                return true;
            }
            Number::Single(_) => { return false; },
            Number::Pair(left, right) => left.split() || right.split()
        }
    }
}

impl Reduce for Number {
    fn reduce(&mut self) {
        if self.explode() || self.split() {
            self.reduce()
        }
    }
}

impl Explode for Number {
    fn explode(&mut self) -> bool {
        match self.explode_impl(0) {
            None => false,
            _ => true
        }
    }

    fn explode_impl(&mut self, depth: usize) -> Option<Explosion> {
        match self {
            Number::Single(val) if depth > 4 => Some(Explosion::Single(*val)),
            Number::Single(_) => None,
            Number::Pair(ref mut left, ref mut right) => {
                let left_explosion = left.explode_impl(depth + 1);

                let mut left_exploded = false;
                match left_explosion {
                    None => {},
                    Some(Explosion::Single(_)) => left_exploded = true,
                    Some(Explosion::Pair(from_left, from_right)) => {
                        right.absorb_left(from_right);
                        *left = Box::new(Number::Single(0));
                        return Some(Explosion::Left(from_left));
                    }, 
                    Some(Explosion::Left(_)) => {
                        return left_explosion;
                    },
                    Some(Explosion::Right(from_right)) => {
                        right.absorb_left(from_right);
                        return Some(Explosion::Done);
                    },
                    Some(Explosion::Done) => {
                        return Some(Explosion::Done)
                    }
                };

                let right_explosion = right.explode_impl(depth + 1);

                let mut right_exploded = false;
                match right_explosion {
                    None => {},
                    Some(Explosion::Single(_)) => right_exploded = true,
                    Some(Explosion::Pair(from_left, from_right)) => {
                        left.absorb_right(from_left);
                        *right = Box::new(Number::Single(0));
                        return Some(Explosion::Right(from_right));
                    }, 
                    Some(Explosion::Left(from_left)) => {
                        left.absorb_right(from_left);
                        return Some(Explosion::Done);
                    },
                    Some(Explosion::Right(_)) => {
                        return right_explosion;
                    },
                    Some(Explosion::Done) => return Some(Explosion::Done)
                };

                if left_exploded && right_exploded {
                    return match (left_explosion.unwrap(), right_explosion.unwrap()) {
                        (Explosion::Single(from_left), Explosion::Single(from_right)) => Some(Explosion::Pair(from_left, from_right)),
                        _ => panic!()
                    }
                }

                if !(left_exploded || right_exploded) {
                    return None;
                }

                panic!();
            }
        }
    }
}

impl Absorb for Number {
    fn absorb_left(&mut self, val: u32) {
        match self {
            Number::Single(ref mut current) => *current += val,
            Number::Pair(left, _) => left.absorb_left(val),
        }
    }

    fn absorb_right(&mut self, val: u32) {
        match self {
            Number::Single(ref mut current) => *current += val,
            Number::Pair(_, right) => right.absorb_right(val),
        }
    }
}

impl Magnitude for Number {
    fn magnitude(self) -> u32 {
        match self {
            Number::Single(val) => val,
            Number::Pair(left, right) => (3 * left.magnitude()) + (2 * right.magnitude())
        }
    }
}

fn part1(numbers: &Vec<Number>) -> u32 {
    let sum = numbers.iter().cloned().reduce(|accum, num| accum + num).unwrap();
    sum.magnitude()
}

fn part2(numbers: &Vec<Number>) -> u32 {
    let mut max = 0;

    for a in 0..numbers.len() {
        for b in 0..numbers.len() {
            if a == b {
                continue
            }

            let magnitude = (numbers[a].clone() + numbers[b].clone()).magnitude();

            if magnitude > max {
                max = magnitude;
            }
        }
    }

    max
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let numbers: Vec<Number> = reader.lines().map(|line| line.unwrap().into()).collect();

	let answer1 = part1(&numbers);
	let answer2 = part2(&numbers);

	println!("Answer 1: {:?}", answer1);
	println!("Answer 2: {:?}", answer2);

    Ok(())
}
