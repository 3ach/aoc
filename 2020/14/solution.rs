use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::convert::Into;
use std::convert::From;

#[derive(Clone,Copy,Debug)]
struct Mask {
    ones: u64,
    zeroes: u64,
    floating: u64
}

#[derive(Clone,Debug)]
enum Instruction {
    SetMask(Mask),
    SetMem(usize, u64),
}

type Program = Vec<Instruction>;

impl From<String> for Mask {
    fn from(input: String) -> Self {
        let mut mask = Mask{
            ones: 0, 
            zeroes: u64::MAX,
            floating: 0,
        };

        for (idx, c) in input.chars().enumerate() {
            let bit = 35 - idx;
            match c {
                '0' => mask.zeroes &= !(0x1 << bit),
                '1' => mask.ones |= (0x1 << bit), 
                'X' => mask.floating |= (0x1 << bit), 
                _ => continue
            }
        }

        return mask;
    }
}


fn part1(program: &Program) -> u64 {
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask: Mask = Mask{ones: 0, zeroes: 0, floating: 0};

    for instruction in program {
        match instruction {
            Instruction::SetMask(m) => mask = *m,
            Instruction::SetMem(address, value) => {
                memory.insert(*address, (value & mask.zeroes) | mask.ones);
            },
        }
    }

    return memory.values().sum();
}

fn part2(program: &Program) -> u64 {
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask: Mask = Mask{ones: 0, zeroes: 0, floating: 0};

    for instruction in program {
        match instruction {
            Instruction::SetMask(m) => mask = *m,
            Instruction::SetMem(address, value) => {
                println!("address expansion of address {}", address);
                let mut addresses = vec![(*address as u64 & mask.zeroes) | mask.ones];
                println!("\t seed {:?}", addresses);
                for bit in 0..36 {
                    if (0x1 << bit) & mask.floating != 0x0 {
                        addresses = addresses.iter().cloned().flat_map(|address| [address | (0x1 << bit), address & !(0x1 << bit)]).collect();
                    }
                }

                println!("Writing to {:?}", addresses);
            },
        }
    }

    return memory.values().sum();
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let program: Program = reader.lines().map(|line| {
        let line = line.unwrap();
        let parts: Vec<&str>  = line.split(" = ").collect();

        match parts[0] {
            "mask" => Instruction::SetMask(String::from(parts[1]).into()),
            _ => {
                let address = parts[0][4..parts[0].len()-1].parse().unwrap();
                let value = parts[1].parse().unwrap();

                Instruction::SetMem(address, value)
            }
        }
    }).collect();

	let answer1 = part1(&program);
	let answer2 = part2(&program);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
