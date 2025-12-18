use adventage::day;
use intcode::{Program, run};

day!(2019, 21);

type TInput = Program;

fn parse(input: &str) -> TInput {
    input
        .trim()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn part1(program: &TInput) -> i64 {
    let script: String = [
        "NOT A J\n",
        "NOT B T\n",
        "OR T J\n",
        "NOT C T\n",
        "OR T J\n",
        "AND D J\n",
        "WALK\n",
    ]
    .into_iter()
    .collect();

    let script: Vec<i64> = script.chars().map(|c| c as i64).collect();
    let result = run(program, &script);

    *result.iter().last().unwrap()
}

fn part2(program: &TInput) -> i64 {
    let script: String = [
        "NOT A J\n",
        "NOT B T\n",
        "OR T J\n",
        "NOT C T\n",
        "OR T J\n",
        "AND D J\n",
        "AND H J\n",
        "NOT A T\n",
        "OR T J\n",
        "RUN\n",
    ]
    .into_iter()
    .collect();

    let script: Vec<i64> = script.chars().map(|c| c as i64).collect();
    let result = run(program, &script);

    *result.iter().last().unwrap()
}
