use std::io::BufRead;
use std::io;
use std::cmp;
use std::collections::HashSet;

#[derive(Eq,Hash,PartialEq,Debug,Clone,Copy)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug,Clone)]
struct Entry {
    patterns: Vec<HashSet<Segment>>,
    outputs: Vec<HashSet<Segment>>,
}

trait Signal {
    fn from_string(&self) -> HashSet<Segment>;
}

impl Signal for String {
    fn from_string(&self) -> HashSet<Segment> {
        return self.chars().map(|segment| {
            match segment {
                'a' => Segment::A,
                'b' => Segment::B,
                'c' => Segment::C,
                'd' => Segment::D,
                'e' => Segment::E,
                'f' => Segment::F,
                'g' => Segment::G,
                _ => panic!("invalid segment {}", segment),
            }
        }).collect::<HashSet<Segment>>();
    }
}

trait Decipher {
    fn from_entry(&self) -> usize;
}

impl Decipher for Entry {
    fn from_entry(&self) -> usize {
        let one = self.patterns.iter().filter(|pattern| pattern.len() == 2).next().unwrap();
        let four = self.patterns.iter().filter(|pattern| pattern.len() == 4).next().unwrap();
        let seven = self.patterns.iter().filter(|pattern| pattern.len() == 3).next().unwrap();
        let eight = self.patterns.iter().filter(|pattern| pattern.len() == 7).next().unwrap();
        let nine = self.patterns.iter()
            .filter(|pattern| pattern.len() == 6)
            .filter(|pattern| pattern.union(&four).count() == 6)
            .next().unwrap();

        let three = self.patterns.iter()
            .filter(|pattern| pattern.len() == 5)
            .filter(|pattern| pattern.union(&one).count() == 5)
            .next().unwrap();

        let zero = self.patterns.iter()
            .filter(|pattern| pattern.len() == 6)
            .filter(|pattern| !pattern.eq(&nine))
            .filter(|pattern| pattern.union(&one).count() == 6)
            .next().unwrap();

        let six = self.patterns.iter()
            .filter(|pattern| pattern.len() == 6)
            .filter(|pattern| !pattern.eq(&nine))
            .filter(|pattern| !pattern.eq(&zero))
            .next().unwrap();

        let two = self.patterns.iter()
            .filter(|pattern| pattern.len() == 5)
            .filter(|pattern| pattern.union(&four).count() == 7)
            .next().unwrap();

        let five = self.patterns.iter()
            .filter(|pattern| pattern.len() == 5)
            .filter(|pattern| !pattern.eq(&three))
            .filter(|pattern| pattern.union(&four).count() == 6)
            .next().unwrap();

        return self.outputs.iter()
            .map(|digit| {
                if digit.eq(&zero) { return 0 }
                else if digit.eq(&one) { return 1 }
                else if digit.eq(&two) { return 2 }
                else if digit.eq(&three) { return 3 }
                else if digit.eq(&four) { return 4 }
                else if digit.eq(&five) { return 5 }
                else if digit.eq(&six) { return 6 }
                else if digit.eq(&seven) { return 7 }
                else if digit.eq(&eight) { return 8 }
                else if digit.eq(&nine) { return 9 }
                panic!("{:?} did not match any known patterns.", digit)
            }).fold(0, |acc, current| (acc * 10) + current);
    }
}

fn part1(observations: &Vec<Entry>) -> usize {
   return observations.iter().map(|observation| {
       observation.outputs.iter()
           .filter(|output| output.len() == 2 || output.len() == 3 || output.len() == 4 || output.len() == 7)
           .count()
   }).sum();
}

fn part2(observations: &Vec<Entry>) -> usize {
    return observations.iter()
        .map(|observation| observation.from_entry())
        .sum();
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let observations = reader.lines()
        .map(|line| {
            let line = line.unwrap();
            let digits: Vec<String> = line.split_whitespace().map(|str| str.to_string()).collect();

            let patterns = &digits[0..10];
            let outputs = &digits[11..15];

            return Entry{
                patterns: patterns.iter().map(|pattern| pattern.from_string()).collect(),
                outputs: outputs.iter().map(|pattern| pattern.from_string()).collect()
            }
        }).collect();

	let answer1 = part1(&observations);
	let answer2 = part2(&observations);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
