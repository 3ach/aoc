<<<<<<< HEAD
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::convert::Into;
use std::convert::From;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Mask { high: u64, low: u64, floating: u64 },
    Set { address: u64, value: u64 }
}
 
fn part1(program: &[Instruction]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask: Instruction = Instruction::Mask { high: 0, low: !0, floating: 0 };

    for instruction in program {
        match instruction {
            Instruction::Mask{..} => { mask = *instruction; },
            Instruction::Set{address, value} => {
                if let Instruction::Mask { high, low , .. } = mask {
                    let real_value = (value & low) | high;
                    mem.insert(address, real_value);
                }
            }
        }
    }

    mem.values().sum()
}

fn part2(program: &[Instruction]) -> u64 {
    let mut mem = HashMap::new();
    let mut mask: Instruction = Instruction::Mask { high: 0, low: !0, floating: 0 };
    let mut permutations = 1;
    let mut floating_idxs = vec![];

    for instruction in program {
        match instruction {
            Instruction::Mask{floating, ..} => { 
                let mut floating = *floating;

                mask = *instruction; 
                permutations = 1;
                floating_idxs = vec![];
                for idx in 0..36 {
                    if floating % 2 == 1 {
                        floating_idxs.push(idx);
                        permutations *= 2;
                    }

                    floating /= 2;
                }
            },
            Instruction::Set{address, value} => {
                if let Instruction::Mask { high, low , floating } = mask {
                    let mut addresses = vec![];

                    for mut permutation in 0..permutations {
                        let mut permuted_address = address | high;
                        for idx in &floating_idxs {
                            let bit = permutation % 2;
                            if bit == 0 {
                                permuted_address &= !(1 << idx); 
                            } else {
                                permuted_address |= 1 << idx; 
                            }
                            permutation /= 2;
                        }

                        addresses.push(permuted_address);
                    }
                   
                    for address in addresses {
                        mem.insert(address, *value);
                    }
                }
            }
        }
    }

    mem.values().sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let program: Vec<Instruction> = reader.lines().map(|line| {
        let line = line.expect("Couldn't read stdin");
        let parts = line.split_once(" = ").unwrap();
        if parts.0 == "mask" {
            let mut high = 0u64;
            let mut low = 0u64;
            let mut floating = 0u64;
            let mut mask = parts.1.chars(); 

            while let Some(bit) = mask.next() {
                high <<= 1;
                low <<= 1;
                floating <<= 1;

                match bit {
                    'X' => floating += 1,
                    '0' => low += 1, 
                    '1' => high += 1,
                    _ => panic!(),
                }
            }

            return Instruction::Mask { high: high, low: !low, floating: floating };
        } else {
            let address: u64 = parts.0[4..parts.0.len()-1].parse().unwrap();
            let value: u64 = parts.1.parse().unwrap();

            return Instruction::Set { address, value };
        }
    }).collect();

	let answer1 = part1(&program);
	let answer2 = part2(&program);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
