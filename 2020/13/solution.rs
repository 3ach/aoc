use std::io::BufRead;
use std::io;
use std::cmp;
use std::convert::TryInto;

/* https://rosettacode.org/wiki/Modular_inverse#Rust */
fn mod_inv(a: i64, module: i64) -> i64 {
  let mut mn = (module, a);
  let mut xy = (0, 1);
 
  while mn.1 != 0 {
    xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
    mn = (mn.1, mn.0 % mn.1);
  }
 
  while xy.0 < 0 {
    xy.0 += module;
  }
  xy.0
}
 
fn part1(earliest: i64, busses: &Vec<i64>) -> i64 {
    let (bus, time) = busses.iter()
        .filter_map(|bus| {
            if *bus == 0 { 
                return None;
            } 

            return Some((bus, bus - (earliest % bus)));
        }).min_by(|(_, time1), (_, time2)| time1.cmp(time2))
        .unwrap();

    return bus * time;
}

fn part2(busses: &Vec<i64>) -> i64 {
    let M: i64 = busses.iter().filter(|b| **b != 0).product();

	/* Chinese Remainder Theorem (deffo didn't know on my own) */
    return busses.iter().enumerate().filter_map(|(i, m)| {
        let m = *m;
        if m == 0 {
            return None;
        }

        let p = M / m;
        let inv = mod_inv(p, m);;
        
        return Some(p * (m - i as i64) * inv);
    }).sum::<i64>() % M; 
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buf = String::new();

    stdin.read_line(&mut buf).expect("Couldn't read line");
    let earliest: i64 = buf.trim().parse().unwrap();
    buf.truncate(0);

    stdin.read_line(&mut buf).expect("Couldn't read line.");
    let busses: Vec<i64> = buf.trim().split(",").map(|bus| bus.parse().unwrap_or(0)).collect();

	let answer1 = part1(earliest, &busses);
	let answer2 = part2(&busses);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
