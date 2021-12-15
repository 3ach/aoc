use std::io::BufRead;
use std::io;
use std::collections::HashSet;
use std::collections::HashMap;
use std::convert::TryInto;

fn part1(chiton: &Vec<Vec<u8>>) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut next: HashMap<(usize, usize), usize> = HashMap::new();

    next.insert((0, 0), 0);

    while !next.is_empty() {
        let (coord, risk) = next.iter().min_by_key(|(_, risk)| *risk).unwrap();
        let (coord, risk) = (*coord, *risk);

        if coord == (chiton.len() - 1, chiton[0].len() - 1) {
            return risk;
        }

        visited.insert(coord);
        next.remove(&coord);

        let mut neighbors: Vec<(usize, usize)> = vec![];

        if coord.0 > 0 {
            neighbors.push((coord.0 - 1, coord.1));
        }

        if coord.1 > 0 {
            neighbors.push((coord.0, coord.1 - 1));
        }

        if coord.0 < chiton.len() - 1 {
            neighbors.push((coord.0 + 1, coord.1));
        }

        if coord.1 < chiton[0].len() - 1 {
            neighbors.push((coord.0, coord.1 + 1));
        }

        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                let risk_to_neighbor = risk + chiton[neighbor.0][neighbor.1] as usize;
                let entry = next.entry(neighbor).or_insert(risk_to_neighbor + 1);
    
                if *entry > risk_to_neighbor {
                    *entry = risk_to_neighbor
                }
            }
        }
    }

    panic!();
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
