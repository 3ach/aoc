use std::io::BufRead;
use std::io;

type TInput = Vec<Instruction>;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32)
}

fn part1(input: &TInput) -> i32 {
    let mut strengths = 0;
    let mut cycles = 0;
    let mut register = 1;

    for instruction in input {
        let (next_register, next_cycles) = match instruction {
            Instruction::Noop => (register, cycles + 1),
            Instruction::Addx(v) => (register + v, cycles + 2)
        };

        let current_cycle_num = (cycles + 20) / 40;
        let next_cycle_num = (next_cycles + 20) / 40;

        if next_cycle_num != current_cycle_num {
            let change = (current_cycle_num * 40) + 20;
            strengths += change * register;
        }

        cycles = next_cycles;
        register = next_register;
    }

    strengths
}

fn part2(input: &TInput) {
    let mut register = 1;
    let mut next_work = 0;
    let mut pending = 0;
    let mut ip = 0;

    for cycle in 0..240 {
        let column: i32 = cycle % 40;

        if cycle == next_work {
            let instruction = &input[ip];
            if let Instruction::Addx(v) = instruction {
                next_work += 2;
                pending = *v;
            } else {
                next_work += 1;
                pending = 0;
            }
            ip += 1;
        }

        if column.abs_diff(register) <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        if column == 39 {
            println!("");
        }

        if pending != 0 && next_work - 1 == cycle {
            register += pending;
            pending = 0;
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let input: TInput = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| if let Some((_, v)) = line.split_once(" ") { Instruction::Addx(v.parse().unwrap()) } else { Instruction::Noop })
        .collect();

	let answer1 = part1(&input);
	println!("Answer 1: {}", answer1);

	part2(&input);

    Ok(())
}
