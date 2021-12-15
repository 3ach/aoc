use std::io::BufRead;
use std::io;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
struct RiskLevel {
    coord: (usize, usize),
    risk_to_here: usize,
}

impl Ord for RiskLevel {
    fn cmp(&self, other: &RiskLevel) -> Ordering {
        return other.risk_to_here.cmp(&self.risk_to_here);
    }
}

impl PartialOrd for RiskLevel {
    fn partial_cmp(&self, other: &RiskLevel) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}


fn part1(chiton: &Vec<Vec<u8>>) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut next: Vec<RiskLevel> = vec![];

    next.push(RiskLevel{coord: (0, 0), risk_to_here: 0});

    while !next.is_empty() {
        let risk = *next.iter().min_by_key(|risk| risk.risk_to_here).unwrap();

        if risk.coord == (chiton.len() - 1, chiton[0].len() - 1) {
            return risk.risk_to_here;
        }

        visited.insert(risk.coord);
        next = next.iter().copied().filter(|n| n.coord != risk.coord).collect();
        let mut neighbors: Vec<(usize, usize)> = vec![];

        if risk.coord.0 > 0 {
            neighbors.push((risk.coord.0 - 1, risk.coord.1));
        }

        if risk.coord.1 > 0 {
            neighbors.push((risk.coord.0, risk.coord.1 - 1));
        }

        if risk.coord.0 < chiton.len() - 1 {
            neighbors.push((risk.coord.0 + 1, risk.coord.1));
        }

        if risk.coord.1 < chiton[0].len() - 1 {
            neighbors.push((risk.coord.0, risk.coord.1 + 1));
        }

        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                let risk_to_neighbor = risk.risk_to_here + chiton[neighbor.0][neighbor.1] as usize;
                next.push(RiskLevel{coord: neighbor, risk_to_here: risk_to_neighbor})
            }
        }
    }

    panic!();
}

fn print_reading(reading: &Vec<Vec<u8>>) {
    for row in 0..reading.len() {
        for col in 0..reading[row].len() {
            print!("{}", reading[row][col]);
        }
        println!("");
    }
}

fn part2(chiton: &Vec<Vec<u8>>) -> usize {
    let mut expanded: Vec<Vec<u8>> = chiton.iter().map(|row| {
        row.iter().copied().cycle().take(row.len() * 5).collect()
    }).cycle().take(chiton.len() * 5).collect();

    for row_bump in 0..5 {
        for col_bump in 0..5 {
            let bump: u8 = (row_bump + col_bump).try_into().unwrap();

            for chiton_row in 0..chiton.len() {
                for chiton_col in 0..chiton[0].len() {
                    let row = chiton_row + (chiton.len() * row_bump);
                    let col = chiton_col + (chiton[0].len() * col_bump);
                    let mut bumped = expanded[row][col] + bump;
                    if bumped > 9 {
                        bumped -= 9;
                    }

                    expanded[row][col] = bumped;
                }
            }
        }
    }

    return part1(&expanded);
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let chiton: Vec<Vec<u8>> = reader.lines()
        .map(|line| line.unwrap().chars().map(|c| c.to_digit(10).unwrap().try_into().unwrap()).collect())
        .collect();

	let answer1 = part1(&chiton);
	let answer2 = part2(&chiton);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
