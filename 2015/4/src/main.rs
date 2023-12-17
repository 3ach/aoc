use adventage::{part1demo, part2demo, day};

part1demo!("abcdef", 609043);
part1demo!("pqrstuv", 1048970);
day!(2015, 4);

fn parse(input: &str) -> &str {
    input.trim()
}

fn part1(val: &str) -> u32 {
    let mut nonce = 0;

    loop {
        let full = format!("{}{}", val, nonce);
        let mut digest = md5::compute(&full);

        let first_six = &mut digest[0..3];
        first_six[2] &= 0xf0 as u8;

        if first_six == [0 as u8; 3] {
            return nonce;
        }

        nonce += 1;
    }
}

fn part2(val: &str) -> u32 {
    let mut nonce = 0;

    loop {
        let full = format!("{}{}", val, nonce);
        let mut digest = md5::compute(&full);

        let first_six = &mut digest[0..3];

        if first_six == [0 as u8; 3] {
            return nonce;
        }

        nonce += 1;
    }
}
