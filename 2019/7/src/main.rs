extern crate intcode;
use intcode::{enter, init, run, Program};
use std::io;
use std::io::BufRead;

fn part1(program: &Program) -> i32 {
    let mut max = 0;
    let mut sequence = vec![0, 0, 0, 0, 0];

    for a in 0..5 {
        for b in 0..5 {
            if b == a {
                continue;
            }

            for c in 0..5 {
                if c == b || c == a {
                    continue;
                }

                for d in 0..5 {
                    if d == c || d == b || d == a {
                        continue;
                    }

                    for e in 0..5 {
                        if e == d || e == c || e == b || e == a {
                            continue;
                        }

                        let mut out = run(&program, &[a, 0]);
                        let first = out.pop().unwrap();
                        let mut out = run(&program, &[b, first]);
                        let second = out.pop().unwrap();
                        let mut out = run(&program, &[c, second]);
                        let third = out.pop().unwrap();
                        let mut out = run(&program, &[d, third]);
                        let fourth = out.pop().unwrap();
                        let mut out = run(&program, &[e, fourth]);
                        let fifth = out.pop().unwrap();

                        if fifth > max {
                            max = fifth;
                            sequence = vec![a, b, c, d, e];
                        }
                    }
                }
            }
        }
    }

    max
}

fn part2(program: &Program) -> i32 {
    let mut max = 0;
    let mut sequence = vec![0, 0, 0, 0, 0];

    for a in 5..10 {
        for b in 5..10 {
            if b == a {
                continue;
            }

            for c in 5..10 {
                if c == b || c == a {
                    continue;
                }

                for d in 5..10 {
                    if d == c || d == b || d == a {
                        continue;
                    }

                    for e in 5..10 {
                        if e == d || e == c || e == b || e == a {
                            continue;
                        }

                        let mut amplifier_a = &mut init(&program, &[]);
                        let mut amplifier_b = &mut init(&program, &[]);
                        let mut amplifier_c = &mut init(&program, &[]);
                        let mut amplifier_d = &mut init(&program, &[]);
                        let mut amplifier_e = &mut init(&program, &[]);

                        println!("{:?}", [a, b, c, d, e]);
                        
                        let mut signal = 0;

                        loop {
                            let mut out = vec![];
                            let mut done = false;

                            (_, out, amplifier_a) = enter(amplifier_a, &[a, signal]);
                            signal = out.pop().unwrap();

                            (_, out, amplifier_b) = enter(amplifier_b, &[b, signal]);
                            signal = out.pop().unwrap();

                            (_, out, amplifier_c) = enter(amplifier_c, &[c, signal]);
                            signal = out.pop().unwrap();

                            (_, out, amplifier_d) = enter(amplifier_d, &[d, signal]);
                            signal = out.pop().unwrap();

                            (done, out, amplifier_e) = enter(amplifier_e, &[e, signal]);
                            signal = out.pop().unwrap();

                            if done {
                                if signal > max {
                                    max = signal;
                                    sequence = vec![a, b, c, d, e];
                                }

                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    max
}

fn main() -> io::Result<()> {
    let program: Program = {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut buf = String::new();
        reader.read_line(&mut buf)?;

        buf.trim()
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    };

    let answer1 = part1(&program);
    println!("Part 1: {}", answer1);

    let answer2 = part2(&program);
    println!("Part 2: {}", answer2);

    Ok(())
}
