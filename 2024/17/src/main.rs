#![feature(iter_intersperse)]
use adventage::{day, part1demo, part2demo};

day!(2024, 17);

#[derive(Debug, Clone)]
struct Computer {
    a: u128,
    b: u128,
    c: u128,
    memory: Vec<u128>,
}
part1demo!(
    "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
    "4,6,3,5,6,3,5,2,1,0"
);
part2demo!("Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0", 117440);
type TInput = Computer;

fn parse(input: &str) -> TInput {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers: Vec<u128> = registers
        .lines()
        .map(|line| {
            let (_, value) = line.split_once(": ").unwrap();
            value.parse().unwrap()
        })
        .collect();
    let (_, program) = program.split_once(": ").unwrap();
    let program = program.trim().split(",").map(|c| c.parse().unwrap()).collect();

    Computer {
        a: registers[0],
        b: registers[1],
        c: registers[2],
        memory: program,
    }
}

fn combo(device: &TInput, operand: u128) -> u32 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => device.a as u32,
        5 => device.b as u32,
        6 => device.c as u32,
        _ => panic!(),
    }
}

fn combo_name(operand: u128) -> String {
    match operand {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "a",
        5 => "b",
        6 => "c",
        _ => panic!(),
    }.to_string()
}

fn run(device: &TInput) -> Vec<u128>{
    let mut device = device.clone();
    let mut output = vec![];
    let mut ip = 0;

    while ip < device.memory.len() - 1 {
        let operator = device.memory[ip];
        let operand = device.memory[ip + 1];

        ip = match operator {
            0 => {
                device.a = device.a / (2_u128.pow(combo(&device, operand)));
                ip + 2
            },
            1 => {
                device.b = device.b ^ (operand as u128);
                ip + 2
            },
            2 => {
                device.b = (combo(&device, operand) % 8) as u128;
                ip + 2
            },
            3 => {
                if device.a == 0 {
                    ip + 2
                } else {
                    operand as usize
                }
            },
            4 => {
                device.b = device.b ^ device.c; 
                ip + 2
            },
            5 => {
                output.push((combo(&device, operand) % 8) as u128);
                ip + 2
            },
            6 => {
                device.b = device.a / (2_u128.pow(combo(&device, operand)));
                ip + 2
            }
            7 => {
                device.c = device.a / (2_u128.pow(combo(&device, operand)));
                ip + 2
            },
            _ => panic!()
        }
    }

    output
}
    

fn part1(device: &TInput) -> String {
    let output = run(&device);
    output.iter().map(|v| v.to_string()).intersperse(",".to_string()).collect()
}

fn part2(device: &TInput) -> u128 {
    let memory = device.memory.clone();
    let mut to_explore = vec![(memory, 0)];

    while let Some((memory, a)) = to_explore.pop() {
        if memory.len() == 0 {
            return a << 3;
        }

        let target = memory[memory.len() - 1];
        let mut rest = memory.clone();
        rest.remove(rest.len() - 1);


        for trial in (0..8).rev() {
            let mut modified = device.clone();
            modified.a = a + trial;

            if run(&modified)[0] == target {
                to_explore.push((rest.clone(), (a + trial) << 3));
            }
        }
    }

    panic!();
}
