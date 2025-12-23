use adventage::day;

day!(2019, 22);

enum Operation {
    New,
    Cut(i64),
    Deal(i64),
}

type TInput = Vec<Operation>;

fn parse_line(line: &str) -> Operation {
    let (verb, rest) = line.split_once(" ").unwrap();
    if verb == "cut" {
        Operation::Cut(rest.parse().unwrap())
    } else if rest.starts_with("into") {
        Operation::New
    } else {
        let increment = rest.split(" ").last().unwrap().parse().unwrap();
        Operation::Deal(increment)
    }
}

fn parse(input: &str) -> TInput {
    input.lines()
        .map(parse_line)
        .collect()
}

fn shuffle(cards: i64, mut interesting: i64, ops: &TInput, rounds: usize) -> i64 {
    let max = interesting - 1;
    for _ in 0..rounds {
        for op in ops {
            interesting = match op {
                Operation::New => cards - max,
                Operation::Cut(top) if *top >= 0 && *top < interesting => {
                    interesting - top
                },
                Operation::Cut(top) if *top >= 0 && *top >= interesting => {
                    interesting + cards - top
                },
                Operation::Cut(top) if *top < 0 && max + *top < interesting => {
                    interesting + top
                },
                Operation::Cut(top) if *top < 0 && max + *top >= interesting => {
                    interesting - cards + top
                },
                Operation::Deal(interval) => {
                    (interesting * interval) % cards
                }
                _ => panic!(),
            }
        }
    }

    interesting
}

fn part1(ops: &TInput) -> i64 {
    shuffle(10007, 2019, ops, 1)
}

fn part2(ops: &TInput) -> i64 {
    shuffle(119315717514047, 2020, ops, 1000000)
}
