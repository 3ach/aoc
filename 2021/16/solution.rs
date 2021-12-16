use std::io::BufRead;
use std::io;
use std::collections::HashSet;
use std::collections::HashMap;
use std::convert::From;

#[derive(Debug)]
enum State {
    Version,
    TypeId,
    Literal,
    LengthType,
    Length,
    PayloadBits(usize),
    PayloadPackets(usize),
    End
}

enum LengthType {
    Bits,
    Packets
}

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

fn read_bits(hex: &String, start: usize, end: usize) -> u128 {
    let hex_start = start / 4;
    let hex_end = (end - 1) / 4;

    let hex_of_interest = &hex[hex_start..=hex_end];

    let mut result = hex_of_interest.chars().map(|c| c.to_digit(16).unwrap()).fold(0, |acc, nib| (acc << 4) + nib);

    if end % 4 != 0 {
        result >>= 4 - (end % 4);
    }

    result &= (1 << (end - start)) - 1;
    return result.into();
}


fn run(packet: &String) -> (u128, u128) {
    let mut state = vec![State::Version];
    let mut cursor = 0;
    let mut outer: Vec<Vec<u128>> = vec![];
    let mut literals: Vec<u128> = vec![];
    let mut literal = 0;
    let mut length_type: LengthType = LengthType::Bits;
    let mut operations: Vec<Operation> = vec![];
    let mut versions = 0;

    while state.len() > 0 {
        let s = state.pop().unwrap();
        match s {
            State::Version => {
                versions += read_bits(packet, cursor, cursor + 3);
                state.push(State::TypeId);
                cursor += 3;
            },
            State::TypeId => {
                let type_id = read_bits(packet, cursor, cursor + 3) as usize;

                match type_id {
                    0 => {
                        outer.push(literals);
                        literals = vec![];
                        state.push(State::LengthType);
                        operations.push(Operation::Sum);
                    },
                    1 => {
                        outer.push(literals);
                        literals = vec![];
                        state.push(State::LengthType);
                        operations.push(Operation::Product);
                    },
                    2 => {
                        outer.push(literals);
                        literals = vec![];
                        state.push(State::LengthType);
                        operations.push(Operation::Minimum);
                    },
                    3 => {
                        outer.push(literals);
                        literals = vec![];
                        state.push(State::LengthType);
                        operations.push(Operation::Maximum);
                    },
                    4 => {
                        state.push(State::Literal)
                    },
                    5 => {
                        outer.push(literals);
                        literals = vec![];
                        state.push(State::LengthType);
                        operations.push(Operation::GreaterThan);
                    },
                    6 => {
                        outer.push(literals);
                        literals = vec![];
                        state.push(State::LengthType);
                        operations.push(Operation::LessThan);
                    },
                    7 => {
                        outer.push(literals);
                        literals = vec![];
                        state.push(State::LengthType);
                        operations.push(Operation::Equal);
                    },
                    _ => panic!()
                }
                cursor += 3;
            }
            State::Literal => {
                let group = read_bits(packet, cursor, cursor + 5);
                literal <<= 4;
                literal |= 0xF & group;

                if group & 0x10 != 0 {
                    state.push(State::Literal);
                } else {
                    literals.push(literal);

                    literal = 0;
                }
                cursor += 5;
            },
            State::LengthType => {
                match read_bits(packet, cursor, cursor + 1) {
                    0 => length_type = LengthType::Bits,
                    1 => length_type = LengthType::Packets,
                    _ => panic!(),
                }
                
                state.push(State::Length);
                cursor += 1;
            },
            State::Length => {
                let length_length = match length_type {
                    LengthType::Bits => 15,
                    LengthType::Packets => 11,
                };


                let length = read_bits(packet, cursor, cursor + length_length) as usize;

                cursor += length_length;
                match length_type {
                    LengthType::Bits => state.push(State::PayloadBits(cursor + length)),
                    LengthType::Packets => state.push(State::PayloadPackets(length)),
                }
            },
            State::PayloadPackets(pkts) => {
                if pkts > 0 {
                    state.push(State::PayloadPackets(pkts - 1));
                    state.push(State::Version);
                } else {
                    state.push(State::End);
                }
            }
            State::PayloadBits(end) => {
                if cursor < end {
                    state.push(State::PayloadBits(end));
                    state.push(State::Version);
                } else {
                    state.push(State::End);
                }
            }
            State::End => {
                let operation = operations.pop().unwrap();
                let result = match operation {
                    Operation::Sum => literals.iter().cloned().sum(),
                    Operation::Product => literals.iter().cloned().product(),
                    Operation::Minimum => literals.iter().cloned().min().unwrap(),
                    Operation::Maximum => literals.iter().cloned().max().unwrap(),
                    Operation::GreaterThan if literals[0] > literals[1] => 1,
                    Operation::GreaterThan if literals[0] <= literals[1] => 0,
                    Operation::LessThan if literals[0] < literals[1] => 1,
                    Operation::LessThan if literals[0] >= literals[1] => 0,
                    Operation::Equal if literals[0] == literals[1] => 1,
                    Operation::Equal if literals[0] != literals[1] => 0,
                    _ => panic!(),
                };

                literals = outer.pop().unwrap();
                literals.push(result);
            },
        }
    }

    return (versions, literals[0]);
}

fn part1(packet: &String) -> u128 {
    let (_, versions) = run(&packet);

    return versions;
}

fn part2(packet: &String) -> u128 {
    let (answer, _) = run(&packet);

    return answer;
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut packet = String::new();

    stdin.read_line(&mut packet);
    packet = packet.trim().to_string();

	let answer1 = part1(&packet);
	let answer2 = part2(&packet);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
