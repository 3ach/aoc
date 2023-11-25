use std::io::BufRead;
use std::io;

fn print(layout: &[usize]) {
    let mut cursor = layout[layout.len() - 1];
    let mut printed = 0;

    print!("{} ", cursor + 1);

    while printed < 12 {
        cursor = layout[cursor];
        print!("{} ", cursor + 1);
        printed += 1;
    }

    println!("");
}

fn round(layout: &mut [usize], crab_idx: usize) {
    let crab = crab_idx + 1;

    let next_idx = layout[crab_idx];
    let next = next_idx + 1;

    let plus1_idx = layout[next_idx];
    let plus1 = plus1_idx + 1;

    let plus2_idx = layout[plus1_idx];
    let plus2 = plus2_idx + 1;
    
    let plus3_idx = layout[plus2_idx];

    let mut destination = if crab > 1 { crab - 1 } else { layout.len() };
    while destination == next || destination == plus1 || destination == plus2 {
        destination = if destination > 1 { destination - 1 } else { layout.len() };
    }
    
    let destination_idx = destination - 1;
    let destination_neighbor_idx = layout[destination_idx];

    layout[crab_idx] = plus3_idx;
    layout[destination_idx] = next_idx;
    layout[plus2_idx] = destination_neighbor_idx;
/*
    println!("crab: {} @ {}", crab,  crab_idx);
    println!("next: {} @ {}", next,  next_idx);
    println!("plus1: {} @ {}", plus1,  plus1_idx);
    println!("plus2: {} @ {}", plus2,  plus2_idx);
    println!("destination: {} @ {}", destination, destination_idx);
    println!("destination neighbor: {} @ {}", destination_neighbor_idx + 1,  destination_neighbor_idx);
    println!("{:?}", layout);

    print(layout);
*/
}

fn part1(layout: &mut [usize], start: usize) -> i64 {
    let mut start = start;
    for r in 0..100 {
        round(layout, start);
        start = layout[start];
    }
    print(layout);
    0
}

fn part2(layout: &mut [usize], start: usize) -> u64 {
    let mut start = start;
    for r in 0..10000000 {
        round(layout, start);
        start = layout[start];
    }

    let next_idx = layout[0];
    let next = next_idx + 1;

    let plus1_idx = layout[next_idx];
    let plus1 = plus1_idx + 1;

    plus1 as u64 * next as u64
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut line = String::new();

    stdin.read_line(&mut line);
    let input = line.trim().chars().map(|c| (c.to_digit(10).unwrap() - 1) as usize).collect::<Vec<_>>();
    let mut positions = [0usize; 9];

    for idx in 1..9 {
       positions[input[idx - 1]] = input[idx];
    }

    positions[input[8]] = input[0];

	let answer1 = part1(&mut positions.clone());
	let answer2 = part2();

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
