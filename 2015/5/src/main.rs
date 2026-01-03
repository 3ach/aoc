use adventage::{day, part1demo, part2demo};

day!(2015, 5);
part1demo!("ugknbfddgicrmopn", 1);
part1demo!("aaa", 1);
part1demo!("jchzalrnumimnmhp", 0);
part1demo!("haegwjzuvuyypxyu", 0);
part1demo!("dvszwmarrgswjxmb", 0);
part2demo!("qjhvhtzxzqqjkmpb", 1);
part2demo!("xxyxx", 1);
part2demo!("uurcxstgmygtbstg", 0);
part2demo!("ieodomkazucvgmuy", 0);

type TInput = Vec<String>;

fn parse(input: &str) -> TInput {
    input.lines().map(String::from).collect()
}

fn vowels(s: &str) -> bool {
    s.chars()
        .filter(|c| *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u')
        .count()
        >= 3
}

fn double(s: &str) -> bool {
    for idx in 0usize..(s.len() - 1) {
        if s[idx..(idx+1)] == s[(idx + 1)..(idx + 2)] {
            return true;
        }
    }

    return false;
}

fn triple(s: &str) -> bool {
    for idx in 0usize..(s.len() - 2) {
        if s[idx..(idx+1)] == s[(idx + 2)..(idx + 3)] {
            return true;
        }
    }

    return false;
}

fn pair(s: &str) -> bool {
    for idx in 0usize..(s.len() - 1) {
        if s[(idx + 2)..s.len()].contains(&s[idx..(idx + 2)]) {
            return true;
        }
    }

    return false;
}

fn disallowed(s: &str) -> bool {
    s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
}

fn part1(input: &TInput) -> usize {
    input.into_iter()
        .filter(|s| vowels(s) && double(s) && !disallowed(s))
        .count()
}

fn part2(input: &TInput) -> usize {
    input.into_iter()
        .filter(|s| triple(s) && pair(s))
        .count()
}
