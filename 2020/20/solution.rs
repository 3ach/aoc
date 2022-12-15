use std::io::BufRead;
use std::io;
use std::collections::{HashSet, HashMap};
use std::fmt;

type TInput = Vec<Tile>;

#[derive(Clone)]
struct Tile {
    id: u64, 
    image: [[bool; 10]; 10],
    edges: HashSet<Vec<bool>>
}

impl fmt::Debug for Tile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Tile: {}\n\n", self.id);
		for row in self.image {
			for px in row {
				write!(f, "{}", if px { "#" } else { "." });
			}
			write!(f, "\n");
		}
		Ok(())
	}
}


fn edges(tile: &Tile) -> HashSet<Vec<bool>> {
    let mut edges = HashSet::new();

    edges.insert(tile.image[0].to_vec());
    edges.insert(tile.image[0].iter().cloned().rev().collect());

    edges.insert(tile.image[9].to_vec());
    edges.insert(tile.image[9].iter().cloned().rev().collect());

    edges.insert(tile.image.iter().cloned().map(|r| r[0]).collect());
    edges.insert(tile.image.iter().cloned().map(|r| r[0]).rev().collect());

    edges.insert(tile.image.iter().cloned().map(|r| r[9]).collect());
    edges.insert(tile.image.iter().cloned().map(|r| r[9]).rev().collect());

    edges
}

fn rotate(tile: &mut Tile) {
	let mut image = [[false; 10]; 10];

	for row in 0..tile.image.len() {
		for col in 0..tile.image[row].len() {
			image[9 - col][row] = tile.image[row][col];
		}
	}

	tile.image = image;
} 

fn flip(tile: &mut Tile) {
	for mut row in &mut tile.image {
		row.reverse();
	}
} 

fn arrange(tiles: &TInput) -> HashMap<(usize, usize), Tile> {
    let edgemap: HashMap<Vec<bool>, Vec<Tile>> = tiles.iter()
        .map(|tile| tile.edges.iter().map(move |edge| (edge.clone(), tile.clone())))
        .flatten()
	.fold(HashMap::new(), |mut set, edge| {
		let entry = set.entry(edge.0).or_insert(vec![]);
		entry.push(edge.1);
		set
	}).iter()
	.filter(|(k, v)| v.len() > 1)
	.map(|(k, v)| (k.clone(), v.clone()))
	.collect();
	
    let mut origin = tiles.iter()
        .filter(|tile| tile.edges
                .intersection(&edgemap.keys().cloned().collect())
                .count() == 4)
        .next()
        .unwrap()
	.clone();
	
	let mut top: Vec<bool> = origin.image[0].to_vec();
	let mut left: Vec<bool> = origin.image.iter().cloned().map(|r| r[0]).collect();

	while edgemap.contains_key(&top) || edgemap.contains_key(&left) {
	    rotate(&mut origin);
	    top = origin.image[0].to_vec();
	    left = origin.image.iter().cloned().map(|r| r[0]).collect();
	}

    let mut arranged = HashMap::new();  
    arranged.insert((0, 0), origin);

    let side = (tiles.len() as f64).sqrt() as usize;
    for row in 0..side {
	if row > 0 {
		let above = arranged.get(&(row - 1, 0)).unwrap();
		let bottom = above.image[9].to_vec();
		let mut below = edgemap.get(&bottom).unwrap().iter().filter(|t| t.id != above.id).next().unwrap().clone();
		let mut tries = 0;
		loop {
			if tries == 4 {
				flip(&mut below);
			}

			let left = below.image.iter().map(|r| r[0]).collect::<Vec<bool>>();
			let top = below.image[0].to_vec();

			if !edgemap.contains_key(&left) && top == bottom {
				break;
			} else {
				rotate(&mut below);
			}

			tries += 1;
		}
		
		arranged.insert((row, 0), below);
	}

        for col in 0..side-1 {
            if let Some(tile) = arranged.get(&(row, col)) {
		let right = tile.image.iter().map(|r| r[r.len() - 1]).collect::<Vec<bool>>();
		let mut next = edgemap.get(&right.to_vec()).unwrap().iter().filter(|t| t.id != tile.id).next().unwrap().clone();
		let above = if row > 0 { Some(arranged.get(&(row - 1, col + 1)).unwrap().image[9]) } else { None };
		let mut tries = 0;
		loop {
			if tries == 4 {
				flip(&mut next);
			} if tries > 10 {
				panic!();
			}

			let top = next.image[0].to_vec();
			let left = next.image.iter().map(|r| r[0]).collect::<Vec<bool>>();

			match above {
				None if left == right && !edgemap.contains_key(&top) => break,
				None => rotate(&mut next),
				Some(above) if left == right && top == above => break,
				Some(_) => rotate(&mut next),
			}
			tries += 1;
		}

		arranged.insert((row, col + 1), next);
            } else {
                panic!("Can't find neighbors of unplaced tile! ({}, {})", row, col);
            }
        }
    }

    arranged
}

fn part1(tiles: &TInput) -> u64 {
	let side = (tiles.len() as f64).sqrt() as usize;
	let mut tiles = tiles.clone();
   	let arrangement = arrange(&tiles);

	arrangement.iter()
		.filter(|((row, _), tile)| *row == 0 || *row == side - 1)
		.filter(|((_, col), tile)| *col == 0 || *col == side - 1)
		.map(|(_, tile)| tile.id)
		.product()
}

fn part2(tiles: &TInput) -> u64 {
    0
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut tiles = vec![];
    let mut tile = Tile { id: 0, image: [[false; 10]; 10], edges: HashSet::new() };
    let mut row = 0;

    for line in reader.lines() {
        let line = line.expect("Couldn't read stdin");
        if line == "" {
            tile.edges = edges(&tile);
            tiles.push(tile);
            tile = Tile { id: 0, image: [[false; 10]; 10], edges: HashSet::new() };
            row = 0;
            continue;
        }

        if line.starts_with("Tile") {
            let line = line.strip_prefix("Tile ").unwrap().strip_suffix(":").unwrap();
            tile.id = line.parse().unwrap();
            continue;
        }

        for (idx, c) in line.chars().enumerate() {
            tile.image[row][idx] = c == '#';
        }

        row += 1;
    }

    tile.edges = edges(&tile);
    tiles.push(tile);

	let answer1 = part1(&tiles);
	let answer2 = part2(&tiles);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
