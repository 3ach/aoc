use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
enum Spring {
    Operational,
    Unknown,
    Damaged,
}

fn print_row(row: &[Spring]) {
    for spring in row {
        match spring {
            Spring::Operational => print!("."),
            Spring::Unknown => print!("?"),
            Spring::Damaged => print!("#"),
        }
    }
}

fn can_slot(row: &[Spring], contiguous: usize, starting: usize) -> bool {
    if starting > 0 && row[starting - 1] == Spring::Damaged {
        false
    } else if starting + contiguous > row.len()
        || (starting + contiguous < row.len() && row[starting + contiguous] == Spring::Damaged)
    {
        false
    } else {
        row[starting..starting + contiguous]
            .iter()
            .all(|c| *c == Spring::Damaged || *c == Spring::Unknown)
    }
}

fn possibilities<'a>(row: &'a [Spring], groups: &'a[usize], mut memo: &mut HashMap<(&'a[Spring], &'a[usize]), usize>) -> usize {
    if let Some(result) = memo.get(&(row, groups)) {
        return *result;
    }

    if groups.len() == 0 {
        if row.iter().all(|spring| *spring != Spring::Damaged) {
            return 1;
        } else {
            return 0;
        }
    }

    let reserved = groups.iter().skip(1).sum::<usize>() + groups.len() - 1;

    let group = groups[0];
    let mut ways = 0;

    for start in 0..=(row.len() - group - reserved) {
        if row[start] == Spring::Operational {
            continue;
        }

        if can_slot(row, group, start) {
            if groups.len() == 1 {
                ways += possibilities(&row[start + group..], &groups[1..], &mut memo)
            } else {
                ways += possibilities(&row[start + group + 1..], &groups[1..], &mut memo)
            }
        }

        if row[start] == Spring::Damaged {
            break;
        }
    }

    memo.insert((row, groups), ways);

    return ways;
}

fn part2(input: &[(Vec<Spring>, Vec<usize>)]) -> usize {
    let total = input.len();
    input
        .iter()
        .map(|(row, groups)| {
            (
                row.iter()
                    .cloned()
                    .chain(vec![Spring::Unknown])
                    .cycle()
                    .take(((row.len() + 1) * 5) - 1)
                    .collect::<Vec<Spring>>(),
                groups
                    .iter()
                    .cycle()
                    .take(groups.len() * 5)
                    .cloned()
                    .collect::<Vec<usize>>(),
            )
        })
        .map(|(row, groups)| possibilities(&row, &groups, &mut HashMap::new()))
        .sum()
}

fn part1(input: &[(Vec<Spring>, Vec<usize>)]) -> usize {
    input
        .iter()
        .map(|(row, groups)| possibilities(&row, &groups, &mut HashMap::new()))
        .sum()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let rows = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            let (springs, groups) = line.split_once(" ").unwrap();
            (
                springs
                    .chars()
                    .map(|c| match c {
                        '.' => Spring::Operational,
                        '?' => Spring::Unknown,
                        '#' => Spring::Damaged,
                        _ => panic!(),
                    })
                    .collect::<Vec<Spring>>(),
                groups
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect::<Vec<(Vec<Spring>, Vec<usize>)>>();

    let answer1 = part1(&rows);
    println!("Answer 1: {}", answer1);

    let answer2 = part2(&rows);
    println!("Answer 2: {}", answer2);

    Ok(())
}
