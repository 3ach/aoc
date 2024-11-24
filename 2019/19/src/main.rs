use adventage::day;
use intcode::{Program, run};

day!(2019, 19);

type TInput = Program;

fn parse(input: &str) -> TInput {
    input
        .trim()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn part1(program: &TInput) -> i64 {
    let mut affected = 0;

    for y in 0..50 {
        for x in 0..50 {
            let cell = run(program, &[x, y])[0];
            if cell == 1 {
                print!("#");
            } else {
                print!(".");
            }

            affected += cell;
        }
        println!();
    }

    affected
}

fn get_x_range(program: &TInput, start: i64, y: i64) -> (i64, i64) {
    let mut x_min = None;

    let mut x = start;
    loop {
        if run(program, &[x, y])[0] == 0 {
            if let Some(x_min) = x_min {
                return (x_min, x)
            }
        } else {
            if x_min.is_none() {
                x_min = Some(x);
            }
        }

        x += 1;
    }
}

fn part2(program: &TInput) -> i64 {
    let mut y_min = 5;
    let mut y_prev = 5;
    let mut x_min = 0;

    loop {
        let x_range = get_x_range(program, x_min, y_min);
        let width = x_range.1 - x_range.0;

        x_min = x_range.0 - 1;
        y_prev = y_min;
        y_min += 1;

        if width < 100 {
            continue;
        }

        for corner_x in x_range.0..(x_range.1 - 99) {
            if run(program, &[corner_x, y_prev + 99])[0] == 1 && run(program, &[corner_x + 99, y_prev])[0] == 1 {
                return (corner_x * 10000) + y_prev;
            }
        }
    }
}
