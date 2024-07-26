use adventage::{part1demo, part2demo, day};

part1demo!("80871224585914546619083218645595", 24176176);
part1demo!("19617804207202209144916044189917", 73745418);
part1demo!("69317163492948606335995924319873", 52432133);
part2demo!("03036732577212944063491565474664", 84462026);
part2demo!("02935109699940807407585447034323", 78725270);
part2demo!("03081770884921959731165446850517", 53553731);

day!(2019, 16);

type TInput = Vec<i32>;

fn parse(input: &str) -> TInput {
    input.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect()
}

fn digit(input: &[i32], place: i32) -> i32 {
    let mut cycle = [0, 1, 0, -1].iter().cycle();
    let mut result = 0;
    let mut base = cycle.next().unwrap();

    for (idx, digit) in input.iter().enumerate() {
        if (idx + 1) as i32 % (place + 1) == 0 {
            base = cycle.next().unwrap();
        }

        result += digit * base;
    }

    result.abs() % 10
}

fn fft(input: &[i32]) -> Vec<i32> {
    let mut out = vec![];
    for (place, dig) in input.iter().enumerate() {
        out.push(digit(input, place as i32));
    }
    out
}

fn back_half(input: &mut [i32], offset: i32) {
    for idx in ((offset as usize)..(input.len() - 1)).rev() {
        input[idx] = (input[idx + 1] + input[idx]).abs() % 10;
    }
}

fn part1(input: &TInput) -> i32 {
    let mut output = input.to_vec();
    for _ in 0..100 {
        output = fft(&output);
    }

    output
        .iter()
        .take(8)
        .fold(0, |acc, d| (acc * 10) + d)
}

fn part2(input: &TInput) -> i32 {
    let offset = input[0..7].iter().fold(0, |acc, d| (acc * 10) + d);
    let mut output: Vec<i32>  = input.iter().cycle().take(input.len() * 10000).cloned().collect();
    for _ in 0..100 {
        back_half(&mut output, offset);
    }

    output
        .iter()
        .skip(offset as usize)
        .take(8)
        .fold(0, |acc, d| (acc * 10) + d)
}
