use std::io;
use std::io::BufRead;

fn part1(pixels: &[u32]) -> u32 {
    let mut layers = vec![];
    let mut layer = vec![];
    let mut row = vec![];

    for (idx, pixel) in pixels.iter().enumerate() {
        row.push(*pixel);
        if row.len() == 25 {
            layer.push(row);
            row = vec![];
        }

        if layer.len() == 6 {
            layers.push(layer);
            layer = vec![];
        }
    }

    let mut least_zeros = 300;
    let mut answer = 0;

    for layer in &layers {
        let zeros: u32 = layer
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pixel| if *pixel == 0 { 1 } else { 0 })
                    .sum::<u32>()
            })
            .sum();

        let ones: u32 = layer
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pixel| if *pixel == 1 { 1 } else { 0 })
                    .sum::<u32>()
            })
            .sum();

        let twos: u32 = layer
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pixel| if *pixel == 2 { 1 } else { 0 })
                    .sum::<u32>()
            })
            .sum();

        if zeros < least_zeros {
            least_zeros = zeros;
            answer = ones * twos;
        }
    }

    answer
}

fn part2(pixels: &[u32]) -> u32 {
    let mut layers = vec![];
    let mut layer = vec![];
    let mut row = vec![];

    for (idx, pixel) in pixels.iter().enumerate() {
        row.push(*pixel);
        if row.len() == 25 {
            layer.push(row);
            row = vec![];
        }

        if layer.len() == 6 {
            layers.push(layer);
            layer = vec![];
        }
    }

    for row in 0..6 {
        for col in 0..25 {
            for layer in &layers {
                match layer[row][col] {
                    0 => { print!(" "); break; }
                    1 => { print!("#"); break; }
                    2 => continue,
                    _ => panic!(),
                }
            }
        }
        println!();
    }
    0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let pixels: Vec<u32> = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .next()
        .unwrap();

    let answer1 = part1(&pixels);
    let answer2 = part2(&pixels);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}
