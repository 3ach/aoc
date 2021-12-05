use std::io::BufRead;
use std::io;



fn part1(report: &Vec<u16>, len: usize) -> i32 {
    let mut gamma: u16 = 0;

    for bit in 0..len {
        let mut ones = 0;
        let mask = 1 << bit;

        for datum in report {
            if datum & mask != 0x0 {
                ones += 1;
            }
        }

        if ones > (report.len() / 2) {
            gamma += 1 << bit;
        }
    }

    let mask: u16 = (1 << len) - 1;
    let epsilon: u16 = mask & !gamma;

    return i32::from(epsilon) * i32::from(gamma);
}

fn part2(report: &Vec<u16>, len: usize) -> i32 {
    let mut candidates = report.clone();
    let mut o2_rating: u16 = 0;

    for mut bit in 0..len {
        bit = len - bit - 1;
        
        let mask = 1 << bit;
        let (ones, zeros): (Vec<u16>, Vec<u16>) = candidates.iter().partition(|datum| *datum & mask != 0x0);

        if zeros.len() > ones.len() {
            candidates = zeros;
        } else {
            candidates = ones;
        }

        if candidates.len() == 1 {
            o2_rating = candidates[0];
            break;
        }
    }

    candidates = report.clone();
    let mut co2_rating: u16 = 0;

    for mut bit in 0..len {
        bit = len - bit - 1;
        
        let mask = 1 << bit;
        let (ones, zeros): (Vec<u16>, Vec<u16>) = candidates.iter().partition(|datum| *datum & mask != 0x0);

        if zeros.len() > ones.len() {
            candidates = ones;
        } else {
            candidates = zeros;
        }

        if candidates.len() == 1 {
            co2_rating = candidates[0];
            break;
        }
    }

    return i32::from(co2_rating) * i32::from(o2_rating);
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut maxlen: usize = 0;

	let report: Vec<u16> = reader.lines()
        .map(|line| line.expect("couldn't read stdin"))
        .map(|line| {
            if maxlen < line.len() {
                maxlen = line.len()
            }
            u16::from_str_radix(&line, 2).unwrap()
        })
        .collect();


	let answer1 = part1(&report, maxlen);
	let answer2 = part2(&report, maxlen);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
