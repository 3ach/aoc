use adventage::day;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn hash(input: &String) -> u32 {
    input.chars()
        .map(|c| u8::try_from(c).unwrap())
        .fold(0u32, |acc, c| ((acc + c as u32) * 17) % 256)
}


fn part1(input: &Vec<String>) -> u32 {
    input.iter()
        .map(hash)
        .sum()
}

fn part2(input: &Vec<String>) -> u32 {
    let mut boxes: HashMap<u32, Vec<(String, u32)>> = HashMap::new();

    for step in input {
        let operator_idx = step.find(&['-', '=']).unwrap();
        let operator = step.chars().nth(operator_idx).unwrap();
        let label = step.chars().take(operator_idx).collect::<String>();
        let box_idx = hash(&label);

        match operator {
            '-' => {
                if let Some(boxe) = boxes.get_mut(&box_idx) {
                    if let Some(idx) = boxe.iter().enumerate().filter(|(_, b)| b.0 == label).map(|(idx, _)| idx).next() {
                        boxe.remove(idx);
                    }
                };
            },
            '=' => {
                let focal = step.chars().skip(operator_idx + 1).collect::<String>().parse::<u32>().unwrap();
                let boxe = boxes.entry(box_idx).or_insert(vec![]);

                if let Some(idx) = boxe.iter().enumerate().filter(|(_, b)| b.0 == label).map(|(idx, _)| idx).next() {
                    boxe[idx] = (label, focal); 
                } else {
                    boxe.push((label, focal));
                }
            },
            _ => panic!(),
        }

    }

    boxes.iter()
        .map(|(box_num, lenses)|  lenses.iter().enumerate().map(|(idx, (_, focal))| (*box_num as u32 + 1) * (idx as u32 + 1) * focal).sum::<u32>())
        .sum()
}

#[day]
fn parse() -> Vec<String> {
    input.map(|line| line.split(",").map(|s| s.to_string()).collect::<Vec<String>>()).next().unwrap()
}
