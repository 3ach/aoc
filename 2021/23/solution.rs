use std::fmt;
use std::io::BufRead;
use std::io;
use std::cmp::{min, max};
use std::convert::TryInto;
use std::cmp::Ordering;
use std::collections::{BinaryHeap,HashMap}; 


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    Wall,
    Empty,
    Occupied(Amphipod)
}

type Map = Vec<Vec<Cell>> ;

#[derive(PartialEq, Eq, Clone)]
struct State {
	map: Map,
	cost: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Cell::Wall => write!(f, "#"),
			Cell::Empty => write!(f, "."),
			Cell::Occupied(Amphipod::Amber) => write!(f, "A"),
			Cell::Occupied(Amphipod::Bronze) => write!(f, "B"),
			Cell::Occupied(Amphipod::Copper) => write!(f, "C"),
			Cell::Occupied(Amphipod::Desert) => write!(f, "D"),
		}
    }
}

fn step_cost(amphipod: &Amphipod) -> usize {
    match amphipod {
        Amphipod::Amber => 1,
        Amphipod::Bronze => 10,
        Amphipod::Copper => 100,
        Amphipod::Desert => 1000,
    }
}

fn room_idx(amphipod: &Amphipod) -> usize {
    match amphipod {
        Amphipod::Amber => 3,
        Amphipod::Bronze => 5,
        Amphipod::Copper => 7,
        Amphipod::Desert => 9,
    }
}

fn print_map(map: &Vec<Vec<Cell>>) {
	for row in map {
		for cell in row {
			print!("{:?}", cell);
		}
		println!();
	}
}

fn find_amphipod_options(row: usize, col: usize, amphipod: &Amphipod, map: &Map) -> Vec<(Map, usize)> {
	let mut states = vec![];
	let room_idxs = vec![3, 5, 7, 9];

	if room_idxs.contains(&col) {
		let mut valid_cols = vec![];

		let mut clear = true;

		for row_idx in 2..row {
			if map[row_idx][col] != Cell::Empty {
				clear = false;
			}
		}

		if clear {
			for hall_idx in col+1..12 {
				if map[1][hall_idx] != Cell::Empty {
					break;
				} 

				valid_cols.push(hall_idx);
			}

			for hall_idx in (1..col+1).rev() {
				if map[1][hall_idx] != Cell::Empty {
					break;
				} 

				valid_cols.push(hall_idx);
			}

			for hall_idx in valid_cols {
				if room_idxs.contains(&hall_idx) {
					continue;
				}

				let mut new_map = map.to_vec();

				new_map[row][col] = Cell::Empty;
				new_map[1][hall_idx] = Cell::Occupied(*amphipod); 
		
				let steps = row.abs_diff(1) + col.abs_diff(hall_idx);
				let cost = steps * step_cost(amphipod);
				states.push((new_map, cost));
			}
		}
	} else {
		let desired_idx = room_idx(&amphipod);
		let mut steps = 1usize.abs_diff(row) + desired_idx.abs_diff(col);
		// println!("max steps: {:?}", steps);
		let mut dest_row = 2;

		let mut room_correct = true;
		for room_row in (2..map.len()-1).rev() {
			if let Cell::Occupied(occupant) = map[room_row][desired_idx] {
				room_correct &= occupant == *amphipod;
			} else {
				dest_row = room_row;
				break;
			}
		}

		steps += dest_row.abs_diff(1);

		if col != desired_idx && room_correct {
			let range = if desired_idx > col {
				col+1..desired_idx+1	
			} else {
				desired_idx..col
			};
			
			let mut blocked = false;
			for hall_idx in range {
				blocked |= map[1][hall_idx] != Cell::Empty;
			}

			if !blocked {
				let mut new_map = map.to_vec();

				new_map[row][col] = Cell::Empty;
				new_map[dest_row][desired_idx] = Cell::Occupied(*amphipod); 
		
				let cost = steps * step_cost(amphipod);
				states.push((new_map, cost));
			}
		}
	}

	states
}

fn finished(map: &Map) -> bool {
	for row in 1..map.len() {
		for col in 1..map[row].len() {
			if let Cell::Occupied(amphipod) = map[row][col] {
				if col != room_idx(&amphipod) {
					return false;
				}
			}
		}
	}
	
	return true;
}

fn next_states(map: &Map) -> Vec<(Map, usize)> {
	let mut states = vec![];
	
	for row in 1..map.len() {
		for col in 1..map[row].len() {
			if let Cell::Occupied(amphipod) = map[row][col] {
				states.append(&mut find_amphipod_options(row, col, &amphipod, map));
			}
		}
	}

	states
}


fn part1(map: &Map) -> usize {
	let mut cheapests: HashMap<Map, usize> = HashMap::new();
	let mut heap = BinaryHeap::new();

	heap.push(State { map: map.to_vec(), cost: 0 });
	while let Some(State { map: new_map, cost: cost_so_far }) = heap.pop() {
		if finished(&new_map) {
			return cost_so_far;
		}

		if let Some(cost) = cheapests.get(&new_map) {
			if *cost > cost_so_far {
				cheapests.insert(new_map.clone(), cost_so_far);
			}
			continue;
		} else {
			cheapests.insert(new_map.clone(), cost_so_far);
		}


		for (next_map, cost) in next_states(&new_map) {
			heap.push( State { map: next_map, cost: cost_so_far + cost });
		}	
	}

	panic!();
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

	let map: Map = reader.lines().map(|line| {
		line.expect("couldn't read stdin")
            .chars()
            .map(|c| {
                match c {
                    ' ' | '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'A' => Cell::Occupied(Amphipod::Amber),
                    'B' => Cell::Occupied(Amphipod::Bronze),
                    'C' => Cell::Occupied(Amphipod::Copper),
                    'D' => Cell::Occupied(Amphipod::Desert),
                    _ => panic!(),
                }
            }).collect()
	}).collect();

	// for option in find_amphipod_options(2, 9, &Amphipod::Desert, &map) {
	// 	print_map(&option.0);
	// 	println!("Cost: {:?}\n\n", option.1);
	// }


	let answer1 = part1(&map);
	println!("{:?}", answer1);
	
    Ok(())
}
