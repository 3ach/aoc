use std::io;
use std::io::BufRead;
use std::collections::HashSet;

enum Number {
    Part(u32, char, (usize, usize), (i32, i32)),
    Nonpart
}

fn part1(parts: &[Number]) -> u32 {
    parts.iter()
        .map(|part| match part {
            Number::Part(num, _, _, _) => num,
            _ => &0
        })
    .sum()
}

fn part2(parts: &[Number]) -> u32 {
    let mut sum = 0;
    let gears = parts.iter()
        .filter_map(|part|
                    if let Number::Part(num, typ, start, loc) = part {
                        if *typ == '*' {
                            Some((*num, *start, *loc))
                        } else {
                            None
                        }
                    } else {
                        None
                    })
        .collect::<HashSet<(u32, (usize, usize), (i32, i32))>>();

        let mut seen = HashSet::new();
        for (one, one_start, one_gear) in &gears {
            if seen.contains(one_gear) {
                continue;
            }

            let mut product = None;
            for (other, other_start, other_gear) in &gears {
                if one_start == other_start || one_gear != other_gear {
                    continue
                }

                if let Some(_) = product {
                    product = None;
                    seen.insert(one_gear);
                    break;
                } else {
                    product = Some(one * other);
                    seen.insert(one_gear);
                }
            }
            
            if let Some(val) = product {
                sum += val;
            }
        }


        sum
}

fn neighbors(row: usize, col: usize) -> HashSet<(i32, i32)> {
    let row = row as i32;
    let col = col as i32;

    (row-1..=row+1)
        .map(|r| (col-1..=col+1).map(move |c| (r, c)))
        .flatten()
        .collect()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let schematic = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let symbols = schematic.iter()
        .enumerate()
        .map(|(ridx, row)| row.iter()
             .enumerate()
             .filter(|(_, cell)| !cell.is_digit(10) && **cell != '.')
             .map(|(cidx, _)| (ridx as i32, cidx as i32))
             .collect::<HashSet<(i32, i32)>>())
        .flatten()
        .collect::<HashSet<(i32, i32)>>();

    let mut parts = vec![];

    let mut number = None;
    let mut symbol = None;
    let mut start = None;

    for (ridx, row) in schematic.iter().enumerate() {
        for (cidx, cell) in row.iter().enumerate() {
            if cell.is_digit(10) {
                let digit = cell.to_digit(10).unwrap();
                if let Some(inflight) = number {
                    number = Some((inflight * 10) + digit);
                } else {
                    number = Some(digit);
                    start = Some((ridx, cidx));
                }

                if let Some(neighbor) = symbols.intersection(&neighbors(ridx, cidx)).next() {
                    symbol = Some((schematic[neighbor.0 as usize][neighbor.1 as usize], (neighbor.0, neighbor.1)));
                }
            } else if let Some(parsed) = number {
                let part = match symbol {
                    Some((symbol, location)) => Number::Part(parsed, symbol, start.unwrap(), location),
                    None => Number::Nonpart
                };
                
                parts.push(part);
                symbol = None;
                number = None;
                start = None;
            }
        }
    }

    let answer1 = part1(&parts);
    let answer2 = part2(&parts);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}
