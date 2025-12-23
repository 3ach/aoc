use adventage::day;
use intcode::{enter, Program, init, run};
use std::collections::{HashMap, HashSet, VecDeque};

day!(2019, 25);

type TInput = Program;

#[derive(Debug)]
struct Room {
    items: HashSet<String>,
    doors: HashMap<String, String>,
}

fn parse(input: &str) -> TInput {
    input
        .trim()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn interpret(output: &str) -> (String, Room) {
    let (start, name_line) = output.lines()
        .enumerate()
        .filter(|(_, line)| line.starts_with("=="))
        .last()
        .unwrap();

    let mut items = HashSet::new();
    let mut doors = HashMap::new();

    let name = String::from(&name_line[3..(name_line.len() - 3)]);
    
    for line in output.lines().skip(start) {
        if !line.starts_with("- ") {
            continue;
        }

        match &line[2..] {
            "north" | "east" | "south" | "west" => {
                doors.insert(String::from(&line[2..]), String::new());
            },
            other => {
                items.insert(String::from(other));
            }
        }
    }

    (name, Room { items, doors })
}

fn from(direction: &str) -> String {
    match direction {
        "east" => String::from("west"),
        "west" => String::from("east"),
        "north" => String::from("south"),
        "south" => String::from("north"),
        _ => panic!(),
    }
}

fn path_to(current: &str, dest: &str, map: &HashMap<String, Room>) -> Vec<String> {
    let mut to_explore = VecDeque::from(vec![(current, vec![])]);

    while let Some((current, path)) = to_explore.pop_back() {
        if current == dest {
            return path;
        }

        if current.len() == 0 {
            continue;
        }

        let room = map.get(current).unwrap();

        for (door, next) in &room.doors {
            if next != current {
                let mut path = path.clone();
                path.push(door.clone());
                to_explore.push_front((&next, path.clone()));
            }
        }
    }

    panic!();
}


fn map(program: &TInput) -> HashMap<String, Room> {
    let mut rooms: HashMap<String, Room> = HashMap::new();
    let mut to_run: Vec<(Option<String>, Vec<String>)> = vec![(None, vec![])];
    while let Some((prev, path)) = to_run.pop() {
        let mut input: Vec<i64> = path.join("\n").chars().map(|c| c as i64).collect();
        input.push(10);

        let output: String = run(program, &input).into_iter().map(|c| char::from_u32(c as u32).unwrap()).collect();
        let (name, mut room) = interpret(&output);
        if rooms.contains_key(&name) {
            continue;
        }

        if let Some(prev) = prev {
            let last_door = path.iter().last().unwrap().clone();
            room.doors.insert(from(&last_door), prev.clone());
            rooms.get_mut(&prev).unwrap().doors.insert(last_door, name.clone());
        }

        for (door, dest) in &room.doors {
            if dest.len() > 0 {
                continue;
            }

            let mut path = path.clone();
            path.push(door.clone());
            to_run.push((Some(name.clone()), path));
        }

        rooms.insert(name.clone(), room);
    }

    rooms
}

fn part1(program: &TInput) -> u32 {
    let rooms = map(program);
    let items: Vec<String> = rooms.iter()
        .map(|(_, r)| r.items.clone())
        .flatten()
        .filter(|i| i != "photons" && i != "infinite loop" && i != "molten lava" && i != "escape pod" && i != "giant electromagnet")
        .collect();

    let mut exec = init(program, &[]);
    let (_, mut out) = enter(&mut exec, &[]);
    for item in &items {
        let output: String = out.iter().map(|c| char::from_u32(*c as u32).unwrap()).collect();
        let (name, _) = interpret(&output); 
        let target_room = rooms.iter().filter(|(_, r)| r.items.contains(item)).next().unwrap().0;
        let mut steps = path_to(&name, target_room, &rooms);
        steps.push(format!("take {item}"));
        let combined: String = steps.join("\n") + "\n";

        (_, out) = enter(&mut exec, &combined.chars().map(|c| c as i64).collect::<Vec<i64>>());
    }

    let output: String = out.iter().map(|c| char::from_u32(*c as u32).unwrap()).collect();
    let (name, _) = interpret(&output); 
    let steps = path_to(&name, "Security Checkpoint", &rooms);
    let combined: String = steps.join("\n") + "\n";
    (_, _) = enter(&mut exec, &combined.chars().map(|c| c as i64).collect::<Vec<i64>>());

    for combo in 0..2_u32.pow(items.len() as u32) {
        (_, out) = enter(&mut exec, &"inv\n".chars().map(|c| c as i64).collect::<Vec<i64>>());
        let output: String = out.iter().map(|c| char::from_u32(*c as u32).unwrap()).collect();
        let drop = output.lines()
            .filter(|l| l.starts_with("-"))
            .map(|l| format!("drop {}\n", &l[2..]))
            .collect::<String>();
        let take = items.iter()
            .enumerate()
            .filter(|(idx, _)| combo & (1 << idx) != 0)
            .map(|(_, item)| format!("take {item}\n"))
            .collect::<String>();

        let instructions = drop + &take + &String::from("east\n");
        (_, out) = enter(&mut exec, &instructions.chars().map(|c| c as i64).collect::<Vec<i64>>());
        let output: String = out.iter().map(|c| char::from_u32(*c as u32).unwrap()).collect();
        let (name, _) = interpret(&output); 
        if name != "Security Checkpoint" {
            return output.lines()
                .map(|l| l.split(" ").filter_map(|w| w.parse::<u32>().ok()))
                .flatten()
                .next()
                .unwrap();
        }
    }

    panic!();
}

fn part2(_program: &TInput) -> i64 {
    0
}
