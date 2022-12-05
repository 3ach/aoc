use std::io::BufRead;
use std::io;
use std::ops::RangeInclusive;
use std::collections::HashMap;

#[derive(Debug,Clone)]
struct Field {
    name: String,
    ranges: (RangeInclusive<usize>, RangeInclusive<usize>)
}

trait Matches {
    fn matches(&self, value: &usize) -> bool;
}

impl Matches for Field {
    fn matches(&self, value: &usize) -> bool {
        self.ranges.0.contains(value) || self.ranges.1.contains(value)
    }
}

fn part1(fields: &[Field], nearby: &[Vec<usize>]) -> usize {
    let mut error = 0;

    for ticket in nearby {
        let invalid = ticket.iter().filter(|x| {
            for field in fields {
                if field.matches(x) {
                    return false;
                }
            }

            return true;
        });

        error += invalid.sum::<usize>();
    }

    error
}

fn part2(fields: &[Field], mine: &[usize], nearby: &[Vec<usize>]) -> usize {
    let valid: Vec<&Vec<usize>> = nearby.iter()
                      .filter(|ticket| ticket.iter()
                            .all(|value| fields.iter()
                                 .any(|field| field.matches(value))))
                    .collect();


    let mut mapping: HashMap<String, usize> = HashMap::new();
    let mut candidates = vec![];
    let mut found = vec![];

    for idx in 0..valid[0].len() {
        let idx_cands: Vec<_> = fields.iter()
            .filter(|field| {
                valid.iter().all(|ticket| field.matches(&ticket[idx]))
            }).collect();

        if idx_cands.len() == 1 {
            mapping.insert(idx_cands[0].name.clone(), idx);
            found.push(idx_cands[0]);
        }

        candidates.push(idx_cands);
    }

    while let Some(resolved) = found.pop() {
        for (idx, idx_cands) in candidates.iter_mut().enumerate() {
            idx_cands.retain(|f| f.name != resolved.name);
            if idx_cands.len() == 1 {
                mapping.insert(idx_cands[0].name.clone(), idx);
                found.push(idx_cands[0]);
            }
        }
    }

    mapping.iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, idx)| mine[*idx])
        .reduce(|a, b| a * b)
        .unwrap()
}

fn to_range(desc: &str) -> RangeInclusive<usize> {
    if let Some(parts) = desc.split_once("-") {
        return parts.0.parse().unwrap()..=parts.1.parse().unwrap();
    }

    panic!();
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut fields: Vec<Field> = vec![];
    let mut my_ticket: Vec<usize> = vec![];
    let mut nearby_tickets: Vec<Vec<usize>> = vec![];

    for line in reader.lines() {
        let line = line.expect("can't read stdin");

        if line.len() == 0 {
            continue;
        }

        if let Some((name, ranges)) = line.split_once(": ") {
            let (first, second) = ranges.split_once(" or ").unwrap();
            fields.push(Field { name: name.to_string(), ranges: (to_range(first), to_range(second)) });
        } else if !line.contains("ticket") {
            let ticket = line.trim().split(",").map(|x| x.parse().unwrap()).collect();
            if my_ticket.is_empty() {
                my_ticket = ticket;
            } else { 
                nearby_tickets.push(ticket);
            }
        }
    }

	let answer1 = part1(&fields, &nearby_tickets[..]);
	let answer2 = part2(&fields, &my_ticket, &nearby_tickets[..]);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
