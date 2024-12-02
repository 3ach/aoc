use adventage::{day, part1demo, part2demo};

day!(2024, 2);
part1demo!(
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    2
);
part2demo!(
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    4
);

type TInput = Vec<Vec<i32>>;

fn parse(input: &str) -> TInput {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn valid(report: &[i32]) -> bool {
    let differences = report.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>();

    differences.iter().all(|diff| *diff > 0 && *diff <= 3)
        || differences.iter().all(|diff| *diff < 0 && *diff >= -3)
}

fn one_removed(report: &[i32]) -> Vec<Vec<i32>> {
    (0..report.len())
        .map(|idx| {
            let mut dampened = report.to_owned();
            dampened.remove(idx);
            dampened
        })
        .collect()
}

fn part1(reports: &TInput) -> usize {
    reports.iter().filter(|report| valid(report)).count()
}

fn part2(reports: &TInput) -> usize {
    reports
        .iter()
        .filter(|report| {
            valid(report) || one_removed(report).iter().any(|dampened| valid(dampened))
        })
        .count()
}
