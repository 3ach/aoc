use std::io::BufRead;
use std::io;
use std::convert::TryInto;

fn part1(heights: &Vec<Vec<u32>>) -> u32 {
    let mut risk = 0;

    for row in 0..heights.len() {
        for col in 0..heights[0].len() {
            if row > 0 && heights[row - 1][col] <= heights[row][col] {
                continue;
            }

            if row < heights.len() - 1 && heights[row + 1][col] <= heights[row][col] {
                continue;
            }
            
            if col > 0 && heights[row][col - 1] <= heights[row][col] {
                continue;
            }
            
            if col < heights[0].len() - 1 && heights[row][col + 1] <= heights[row][col] {
                continue;
            }

            risk += heights[row][col] + 1;
        }
    }

    return risk;
}

fn remap(map: &mut Vec<Vec<Option<usize>>>, left: usize, upper: usize, new: usize) {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            match map[row][col] {
                Some(basin) if basin == left || basin == upper => map[row][col] = Some(new),
                _ => {}
            }
        }
    }
}



fn part2(heights: &Vec<Vec<u32>>) -> usize {
    let mut basins: Vec<usize> = vec![];
    let mut map: Vec<Vec<Option<usize>>> = heights.iter().map(|row| row.iter().map(|_| None).collect()).collect();

    for row in 0..heights.len() {
        for col in 0..heights[0].len() {
            if heights[row][col] == 9 {
                continue;
            }

            let mut left_basin: Option<usize> = None;
            let mut upper_basin: Option<usize> = None;

            if row > 0 {
                upper_basin = map[row - 1][col];
            }

            if col > 0 {
                left_basin = map[row][col - 1];
            }

            match (left_basin, upper_basin) {
                (Some(left), Some(upper)) if left != upper => {
                    let combined = basins[left] + basins[upper as usize] + 1;
                    let new = basins.len();

                    remap(&mut map, left, upper, new);

                    basins[left] = 0;
                    basins[upper] = 0;
                    basins.push(combined);
                    map[row][col] = Some(new);
                },
                (Some(left), Some(upper)) if left == upper => {
                    map[row][col] = Some(left);
                    basins[left as usize] += 1;
                }
                (Some(basin), None) | (None, Some(basin)) => {
                    map[row][col] = Some(basin);
                    basins[basin] += 1;
                },
                (None, None) => {
                    let new = basins.len();
                    basins.push(1);
                    map[row][col] = Some(new);
                },
                _ => panic!()
            }
        }
    }

    basins.sort();

    return basins.iter().rev().take(3).product();
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let heights = reader
        .lines()
        .map(|line| line.unwrap().chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

	let answer1 = part1(&heights);
	let answer2 = part2(&heights);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
