use adventage::day;
use intcode::{enter, Execution, Program, init};

day!(2019, 23);

type TInput = Program;

type Packet = (i64, i64);
struct Computer {
    rx: Vec<Packet>,
    computer: Execution
}

fn parse(input: &str) -> TInput {
    input
        .trim()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn part1(program: &TInput) -> i64 {
    let mut computers: Vec<Computer> = (0..50)
        .into_iter()
        .map(|c|
            Computer {
                rx: vec![],
                computer: init(program, &[c])
            }).collect();


    loop {
        for idx in 0..50 {
            let input = if computers[idx].rx.len() > 0 {
               computers[idx].rx.iter().map(|(x, y)| [*x, *y]).flatten().collect() 
            } else {
                vec![-1]
            };

            computers[idx].rx.clear();

            let (done, output) = enter(&mut computers[idx].computer, &input);
            if done {
                panic!("Some computer halted!");
            }

            if output.len() % 3 != 0 {
                panic!("malformed packet");
            }

            for base in 0..(output.len() / 3) {
                let dest = output[3 * base];
                let x = output[(3 * base) + 1];
                let y = output[(3 * base) + 2];

                if dest == 255 {
                    return y;
                }

                computers[dest as usize].rx.push((x, y));
            }
        }
    }
}

fn part2(program: &TInput) -> i64 {
    let mut computers: Vec<Computer> = (0..50)
        .into_iter()
        .map(|c|
            Computer {
                rx: vec![],
                computer: init(program, &[c])
            }).collect();

    let mut nat: Option<Packet> = None;
    let mut last_sent: Option<Packet> = None;

    loop {
        let mut sent = false;
        for idx in 0..50 {
            let input = if computers[idx].rx.len() > 0 {
               computers[idx].rx.iter().map(|(x, y)| [*x, *y]).flatten().collect() 
            } else {
                vec![-1]
            };

            computers[idx].rx.clear();

            let (done, output) = enter(&mut computers[idx].computer, &input);
            if done {
                panic!("Some computer halted!");
            }

            if output.len() % 3 != 0 {
                panic!("malformed packet");
            }

            for base in 0..(output.len() / 3) {
                sent = true;
                let dest = output[3 * base];
                let x = output[(3 * base) + 1];
                let y = output[(3 * base) + 2];

                if dest == 255 {
                    nat = Some((x, y));
                } else {
                    computers[dest as usize].rx.push((x, y));
                }
            }
        }
        
        if !sent {
            if let Some(packet) = nat {
                if let Some(last_packet) = last_sent {
                    if packet.1 == last_packet.1 {
                        return packet.1;
                    }
                }

                computers[0].rx.push(packet.clone());
                last_sent = Some(packet);
                nat = None;
            } else {
                panic!("No activity but no packet");
            }
        }
    }
}
