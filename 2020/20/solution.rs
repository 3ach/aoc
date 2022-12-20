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
		write!(f, "Tile: {}\n\n", self.id)?;
		for row in self.image {
			for px in row {
				write!(f, "{}", if px { "#" } else { "." })?;
			}
			write!(f, "\n")?;
		}
		Ok(())
	}
}

fn coalesce(arrangement: &HashMap<(usize, usize), Tile>) -> HashMap<(usize, usize), bool> {
	arrangement.iter()
		.map(|((pcol, prow), panel)| 
			panel.image.iter()
				.enumerate()
				.map(|(py, row)| 
					row.iter()
						.enumerate()
						.map(|(px, pixel)| ((((10 * pcol) + px) - pcol, 				
											 ((10 * prow) + py) - prow), *pixel))
						.collect::<Vec<((usize, usize), bool)>>()
				).flatten()
				.collect::<Vec<((usize, usize), bool)>>()
		).flatten()
		.collect()
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

fn rotations(tile: &Tile) -> Vec<Tile> {
	let mut rots = vec![];
	let mut current = tile.clone();

	for _ in 0..4 {
		rots.push(current.clone());
		rotate(&mut current);
	}

	let mut flipped = rots.iter()
		.cloned()
		.map(|mut rot| { flip(&mut rot); rot })
		.collect();

	rots.append(&mut flipped);

	rots
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
	for row in &mut tile.image {
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
	.filter(|(_, v)| v.len() > 1)
	.map(|(k, v)| (k.clone(), v.clone()))
	.collect();
	
    let origin = tiles.iter()
        .filter(|tile| tile.edges
                .intersection(&edgemap.keys().cloned().collect())
                .count() == 4)
        .next()
        .unwrap()
	.clone();

	let origin = rotations(&origin)
		.iter()
		.filter(|origin| {
			let top: Vec<bool> = origin.image[0].to_vec();
			let left: Vec<bool> = origin.image.iter().cloned().map(|r| r[0]).collect();

			!edgemap.contains_key(&top) && !edgemap.contains_key(&left)
		})
		.next()
		.unwrap()
		.clone();

    let mut arranged = HashMap::new();  
    arranged.insert((0, 0), origin);

    let side = (tiles.len() as f64).sqrt() as usize;
    for row in 0..side {
		if row > 0 {
			let above = arranged.get(&(0, row - 1)).unwrap();
			let bottom = above.image[9].to_vec();
			let next = edgemap.get(&bottom)
						.unwrap()
						.iter()
						.filter(|tile| tile.id != above.id)
						.next()
						.unwrap();

			let next = rotations(next)
				.iter()
				.filter(|tile| tile.image[0].to_vec() == bottom)
				.next()
				.unwrap()
				.clone();

			arranged.insert((0, row), next);
		}

        for col in 0..side-1 {
			let current = arranged.get(&(col, row)).unwrap();
			let right: Vec<bool> = current.image.iter().map(|r| r[9]).collect();

			let next = edgemap.get(&right)
						.unwrap()
						.iter()
						.filter(|tile| tile.id != current.id)
						.next()
						.unwrap();

			let next = rotations(next)
				.iter()
				.filter(|tile| tile.image.iter().map(|r| r[0]).collect::<Vec<bool>>() == right)
				.next()
				.unwrap()
				.clone();

			arranged.insert((col + 1, row), next);
        }
    }

    arranged
}

fn part1(tiles: &TInput) -> u64 {
	let side = (tiles.len() as f64).sqrt() as usize;
	let tiles = tiles.clone();
   	let arrangement = arrange(&tiles);

	arrangement.iter()
		.filter(|((row, _), _)| *row == 0 || *row == side - 1)
		.filter(|((_, col), _)| *col == 0 || *col == side - 1)
		.map(|(_, tile)| tile.id)
		.product()
}

fn part2(tiles: &TInput) -> u64 {
	let mut side = (tiles.len() as f64).sqrt() as usize;
	side = (side * 10) - side;

	let tiles = tiles.clone();
   	let arrangement = arrange(&tiles);
	let image = coalesce(&arrangement);

	for row in 1..side {
		for col in 1..side {
			let pixel = *image.get(&(col, row)).unwrap();
			print!("{}", if pixel { "#" } else { "." });
		}
		println!("");
	}
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
