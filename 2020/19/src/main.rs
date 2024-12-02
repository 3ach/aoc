use std::io::BufRead;
use std::io;
use std::convert::TryInto;
use std::collections::HashMap;
use regex::Regex;

fn part1(rule: &str, words: &[String]) -> usize {
    let mut whole_rule = String::from("^");
    whole_rule.push_str(rule);
    whole_rule.push_str("$");

    let regex = Regex::new(&whole_rule).unwrap();

    words.iter()
        .inspect(|word| println!("testing {:?}", word))
        .filter(|word| regex.is_match(word))
        .inspect(|word| println!("{:?} matches", word))
        .count()
}

fn part2() -> u64 {
    0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines = reader.lines();

    let mut resolved = HashMap::new();
    let mut unresolved = HashMap::new();
    let mut to_replace = vec![];
    let mut in_words = false;
    let mut words = vec![];

    for line in lines {
        let line = line.expect("Could not read stdin");
        if line == "" {
            in_words = true;
            continue;
        }

        if in_words {
            words.push(line);
            continue;
        }
        
        
        if let Some((name, rule)) = line.split_once(": ") {
            if rule.contains("\"") {
                let rule = rule.trim_matches('"');
                to_replace.push(name.to_string());
                unresolved.insert(name.to_string(), vec![rule.to_string()]);
            } else {
                let mut parsed = vec!["(".to_string()];
                parsed.extend(rule.split(" ").map(|s| s.to_string()));
                parsed.push(")".to_string());
                unresolved.insert(name.to_string(), parsed);
            }
        } else {
            panic!();
        }
    }

    let mut unresolved2 = unresolved.clone();
    let mut resolved2 = resolved.clone();

    unresolved2.insert("8".to_string(), vec!["(".to_string(), "42".to_string(), "+".to_string(), ")".to_string()]);

    let mut eleven = vec!["(".to_string()];
    for cnt in 1..=10 {
        let cnts = format!("{{{}}}", cnt);
        eleven.append(&mut vec!["(".to_string(), "(".to_string(), "42".to_string(), ")".to_string(), cnts.to_string(), "(".to_string(), "31".to_string(), ")".to_string(), cnts.to_string(), ")".to_string()]);
        if cnt < 10 {
            eleven.push("|".to_string());
        }
    }
    eleven.push(")".to_string());

    println!("{:?}", eleven.join(""));

    unresolved2.insert("11".to_string(), eleven);

    while let Some(name) = to_replace.pop() {
        if resolved.contains_key(&name) {
            continue
        }

        let rule = unresolved.remove(&name).unwrap().join("");
        let rule2 = unresolved2.remove(&name).unwrap().join("");

        resolved.insert(name.to_string(), rule);
        resolved2.insert(name.to_string(), rule2);
        
        for (resolving, rule) in unresolved.iter_mut() {
            let mut is_resolved = true;
            for term in rule.iter_mut() {
                if *term == name {
                    *term = (*resolved.get(&name).expect("not found!").clone()).to_string();
                } else if term.as_bytes()[0].is_ascii_digit() {
                    is_resolved = false;
                }
            }

            if is_resolved {
                to_replace.push(resolving.to_string());
            }
        }

        for (resolving, rule) in unresolved2.iter_mut() {
            let mut is_resolved = true;
            for term in rule.iter_mut() {
                if *term == name {
                    *term = (*resolved2.get(&name).expect("not found!").clone()).to_string();
                }
            }
        }
    }

	let answer1 = part1(&resolved["0"], &words);
	println!("Answer 1: {}", answer1);

	let answer2 = part1(&resolved2["0"], &words);
	println!("Answer 2: {}", answer2);

    Ok(())
}
