use std::io::BufRead;
use std::io;
use std::cmp;
use std::fmt;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::hash::{Hash, Hasher};

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert
}

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
enum Tile {
    Empty,
    Amphipod(Amphipod)
}

#[derive(Clone)]
struct Burrow {
    rooms: Vec<[Tile; 2]>,
    hallway: [Tile; 11], 
    cost: u32
}


impl Hash for Burrow {
    fn hash<H: Hasher>(&self, state: &mut H) {
		self.rooms.hash(state);
		self.hallway.hash(state);
    }
}

impl Ord for Burrow {
    fn cmp(&self, other: &Self) -> Ordering {
       self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Burrow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl PartialEq for Burrow {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for Burrow { }

impl fmt::Debug for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("\n#############\n#{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}#\n###{:?}#{:?}#{:?}#{:?}###\n  #{:?}#{:?}#{:?}#{:?}#\n  ######### {}\n", 
                                 self.hallway[0], self.hallway[1], self.hallway[2], self.hallway[3], self.hallway[4], self.hallway[5], 
                                 self.hallway[6], self.hallway[7], self.hallway[8], self.hallway[9], self.hallway[10],
                                 self.rooms[0][0], self.rooms[1][0], self.rooms[2][0], self.rooms[3][0],
                                 self.rooms[0][1], self.rooms[1][1], self.rooms[2][1], self.rooms[3][1], self.cost))
    }
}


impl From<char> for Amphipod {
    fn from(c: char) -> Self {
        match c {
            'A' => Amphipod::Amber,
            'B' => Amphipod::Bronze,
            'C' => Amphipod::Copper,
            'D' => Amphipod::Desert,
            _ => panic!(),
        }
    }
}

impl fmt::Debug for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Amphipod::Amber => f.write_str("A"),
            Amphipod::Bronze => f.write_str("B"),
            Amphipod::Copper  => f.write_str("C"),
            Amphipod::Desert  => f.write_str("D"),
        }
    }
}


impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            _ => Tile::Amphipod(c.into()),
        }
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty => f.write_str("."),
            Tile::Amphipod(a) => f.write_fmt(format_args!("{:?}", a)),
        }
    }
}

impl From<Vec<String>> for Burrow {
    fn from(input: Vec<String>) -> Self {
        let hallway = [Tile::Empty; 11];

        let first_row: Vec<Tile> = input[2].chars().skip(3).step_by(2).take(4).map(|c| c.into()).collect();
        let second_row: Vec<Tile> = input[3].chars().skip(3).step_by(2).take(4).map(|c| c.into()).collect();

        let rooms: Vec<[Tile; 2]> = (0..=3).map(|idx| [first_row[idx], second_row[idx]]).collect();

        Burrow{
            hallway: hallway,
            rooms: rooms,
            cost: 0,
        }
    }
}

fn is_finished(burrow: &Burrow) -> bool {
	finish_count(burrow) == 8
}

fn finish_count(burrow: &Burrow) -> usize {
	let mut count = 0;

	if burrow.rooms[0][0] == Tile::Amphipod(Amphipod::Amber) {
		if burrow.rooms[0][1] == Tile::Amphipod(Amphipod::Amber) {
			count += 1;
		}
		
		count += 1;
	}

	if burrow.rooms[1][0] == Tile::Amphipod(Amphipod::Bronze) {
		if burrow.rooms[1][1] == Tile::Amphipod(Amphipod::Bronze) {
			count += 1;
		}
		
		count += 1;
	}

	if burrow.rooms[2][0] == Tile::Amphipod(Amphipod::Copper) {
		if burrow.rooms[2][1] == Tile::Amphipod(Amphipod::Copper) {
			count += 1;
		}
		
		count += 1;
	}

	if burrow.rooms[3][0] == Tile::Amphipod(Amphipod::Desert) {
		if burrow.rooms[3][1] == Tile::Amphipod(Amphipod::Desert) {
			count += 1;
		}
		
		count += 1;
	}

	count			
}

fn cost(moves: usize, amphipod: Amphipod) -> u32 {
    moves as u32 * match amphipod {
        Amphipod::Amber => 1,
        Amphipod::Bronze => 10,
        Amphipod::Copper => 100,
        Amphipod::Desert => 1000,
    }
}

fn next(initial: Burrow) -> Vec<Burrow> {
    let mut next = vec![];
    for (idx, room) in initial.rooms.iter().enumerate() {
        let mut moves = 0;
        let starting_idx: usize;
        let amphipod: Amphipod;

        if let Tile::Amphipod(a) = room[0] {
            moves += 1;
            starting_idx = 0;
            amphipod = a;
        } else if let Tile::Amphipod(a) = room[1] {
            moves += 2;
            starting_idx = 1;
            amphipod = a;
        } else {
            continue;
        }

        let mut next_burrow_template = initial.clone();
        next_burrow_template.rooms[idx][starting_idx] = Tile::Empty;

        let hall_idx = 2 + (idx * 2);

        for dest_idx in (0..hall_idx).rev() {
            if let Tile::Amphipod(_) = initial.hallway[dest_idx] {
                break;
            }
            
            if dest_idx < 8 && dest_idx > 1 && dest_idx % 2 == 0 {
                continue; 
            }

            let total_moves = moves + (hall_idx - dest_idx);

            let mut next_burrow = next_burrow_template.clone();
            next_burrow.hallway[dest_idx] = Tile::Amphipod(amphipod);
            next_burrow.cost += cost(total_moves, amphipod);

            next.push(next_burrow);
        }

        for dest_idx in (hall_idx + 1..11).rev() {
            if let Tile::Amphipod(_) = initial.hallway[dest_idx] {
                break;
            }
            
            if dest_idx < 8 && dest_idx > 1 && dest_idx % 2 == 0 {
                continue; 
            }

            let total_moves = moves + (dest_idx - hall_idx);

            let mut next_burrow = next_burrow_template.clone();
            next_burrow.hallway[dest_idx] = Tile::Amphipod(amphipod);
            next_burrow.cost += cost(total_moves, amphipod);

            next.push(next_burrow);
        }
    }

    let occupied: Vec<usize> = initial.hallway.iter().enumerate().filter(|(_, space)| {
        if let Tile::Amphipod(_) = space { 
            return true
        } 

        return false
    }).map(|(idx, _)| idx).collect();

    for &hallway_idx in &occupied {
        if let Tile::Amphipod(amphipod) = initial.hallway[hallway_idx] {
            let mut moves = 0;
            let room_idx = match amphipod {
                Amphipod::Amber => 2,
                Amphipod::Bronze => 4,
                Amphipod::Copper => 6,
                Amphipod::Desert => 8,
            };

            let blocking_count = occupied.iter().cloned().filter(|&x| x > hallway_idx && x < room_idx).count();
            if blocking_count > 0 {
                continue;
            }

            let mut next_burrow = initial.clone();
            next_burrow.hallway[hallway_idx] = Tile::Empty;

            moves += (hallway_idx as isize - room_idx as isize).abs() as usize;

            let room = (room_idx - 2) / 2;
            if let Tile::Empty = initial.rooms[room][0] {
                if let Tile::Empty = initial.rooms[room][1] {
                    next_burrow.rooms[room][1] = Tile::Amphipod(amphipod);
                    next_burrow.cost += cost(moves + 2, amphipod);
                } else if let Tile::Amphipod(a) = initial.rooms[room][1] {
                    if amphipod == a {
                        next_burrow.rooms[room][0] = Tile::Amphipod(amphipod);
                        next_burrow.cost += cost(moves + 1, amphipod);
                    }
                }

                if initial.hallway[3] == Tile::Amphipod(Amphipod::Bronze)  && initial.hallway[5] == Tile::Amphipod(Amphipod::Copper) && initial.cost == 240 {
                    println!("NEXT! {:?}", next_burrow);
                }
                next.push(next_burrow);
            }
        }
    }
    
    next
}

fn part1(burrow: Burrow) -> u32 {
    let mut futures = BinaryHeap::new();
    let mut seen = HashSet::new();
    let mut min: u32 = u32::MAX;
    
    futures.push(Reverse(burrow.clone()));
    seen.insert(burrow);

    while let Some(Reverse(burrow)) = futures.pop() {
        if is_finished(&burrow) && burrow.cost < min {
            println!("Found a finish with cost {}", burrow.cost);
            min = burrow.cost; 
            continue;
        }

        for future in next(burrow) {
            if !seen.contains(&future) {
                futures.push(Reverse(future.clone()));
                seen.insert(future);
            }
        }

        println!("there are now {} futures, and I've seen {}", futures.len(), seen.len());
    }

    min
}

fn part2() -> usize {
    0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let burrow: Burrow = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>().into();

	let answer1 = part1(burrow);
	let answer2 = part2();

	println!("Answer 1: {:?}", answer1);
	println!("Answer 2: {:?}", answer2);

    Ok(())
}
