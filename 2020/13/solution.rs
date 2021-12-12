use std::io::BufRead;
use std::io;
use std::cmp;
use std::convert::TryInto;

fn part1(earliest: u32, busses: &Vec<u32>) -> u32 {
    let (bus, time) = busses.iter()
        .filter_map(|bus| {
            if *bus == 0 { 
                return None;
            } 

            return Some((bus, bus - (earliest % bus)));
        }).min_by(|(_, time1), (_, time2)| time1.cmp(time2))
        .unwrap();

    return bus * time;
}

fn part2() -> u32 {
    return 0;
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf);
    let earliest: u32 = buf.trim().parse().unwrap();

    stdin.read_line(&mut buf);
    let busses: Vec<u32> = buf.trim().split(",").map(|bus| bus.parse().unwrap_or(0)).collect();

	let answer1 = part1(earliest, &busses);
	let answer2 = part2();

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
