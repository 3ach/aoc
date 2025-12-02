use adventage::{day, part1demo, part2demo};

day!(2025, 1);
part1demo!("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82", 3);
part2demo!("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82", 6);
part2demo!("L1000", 10);
part2demo!("R1000", 10);
part2demo!("R50", 1);
part2demo!("R49", 0);
part2demo!("R51", 1);
part2demo!("R150", 2);
part2demo!("R250", 3);
part2demo!("R250
L250", 5);


#[derive(Debug)]
enum Movement {
    Left(i32), 
    Right(i32),
}

type TInput = Vec<Movement>;

fn parse(input: &str) -> TInput {
    input.lines()
        .map(|movement| {
            let ticks = movement[1..movement.len()].parse::<i32>().unwrap();

            match &movement[0..1] {
                "L" => Movement::Left(ticks),
                "R" => Movement::Right(ticks),
                _ => panic!("Unknown movement")
            }
        })
        .collect()
}

fn part1(input: &TInput) -> i32 {
    input
        .into_iter()
        .scan(50, |state, movement| {
            *state += match movement {
                Movement::Left(t) => -t,
                Movement::Right(t) => *t,
            };

            *state %= 100;

            if *state == 0 {
                Some(1)
            } else {
                Some(0)
            }

        }).sum()
}

fn part2(input: &TInput) -> i32 {
    input
        .into_iter()
        .scan(50, |state, movement| {
            let zero = *state == 0;
            let (uncorrected, mut ticks) = match movement {
                Movement::Left(t) => (-(t % 100), *t / 100),
                Movement::Right(t) => ((t % 100), t / 100),
            };

            *state += uncorrected;

            let next = if *state <= 0 {
                if !zero { ticks += 1; }
                if *state < 0 { *state + 100 } else { *state } 
            } else if *state >= 100 {
                ticks += 1;
                *state % 100
            } else {
                *state
            };

            *state = next;

            Some(ticks)
        }).sum()
}
