use adventage::{part1demo, part2demo, day};

part1demo!("abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab", 12);

part2demo!("abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz", "fgij");


day!(2018, 2);

fn parse(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| String::from(l))
        .collect()
}

fn part1(input: &Vec<String>) -> usize {
    let twos = input.iter()
        .filter(|line| line.chars().any(|c| line.chars().filter(|o| *o == c).count() == 2))
        .count();

    let threes = input.iter()
        .filter(|line| line.chars().any(|c| line.chars().filter(|o| *o == c).count() == 3))
        .count();

    twos * threes
}

fn part2(input: &Vec<String>) -> String {
    for left in input {
        for right in input {
            let same = left.chars().zip(right.chars()).filter_map(|(l, r)| if l == r { Some(r) } else { None }).collect::<String>();
            if same.len() == right.len() - 1 {
                return same;
            }
        }
    }

    panic!();
}
