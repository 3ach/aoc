use std::io;
use std::io::BufRead;

type Race = (u128, u128);

fn ways_to_win(race: &Race) -> u128 {
    let mut ways = 0;

    for way in 0..=race.0 {
        if (way * (race.0 - way)) > race.1 {
            ways += 1;
        }
    }

    ways
}

fn part1(races: &[Race]) -> u128 {
    races.iter()
        .map(ways_to_win)
        .product()
}

fn part2(races: &[Race]) -> u128 {
    let time = races.iter()
        .map(|(t, _)| format!("{}", t))
        .collect::<String>()
        .parse::<u128>()
        .unwrap();

    let distance = races.iter()
        .map(|(_, d)| format!("{}", d))
        .collect::<String>()
        .parse::<u128>()
        .unwrap();

    ways_to_win(&(time, distance))
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let races = reader.lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .map(|line| line.split_whitespace().skip(1).map(|num| num.parse::<u128>().unwrap()).collect::<Vec<u128>>())
        .collect::<Vec<Vec<u128>>>();

    let races = (0..races[0].len()).map(|idx| (races[0][idx], races[1][idx])).collect::<Vec<(u128, u128)>>();

    let answer1 = part1(&races);
    let answer2 = part2(&races);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}
