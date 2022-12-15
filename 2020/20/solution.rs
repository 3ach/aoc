use std::io::BufRead;
use std::io;
use std::collections::{HashSet, HashMap};

type TInput = Vec<Tile>;

#[derive(Debug)]
struct Tile {
    id: u64, 
    image: [[bool; 10]; 10],
    edges: HashSet<Vec<bool>>
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

fn arrange(tiles: &TInput) -> HashMap<(usize, usize), &Tile> {
    let edgemap: HashMap<Vec<bool>, &Tile> = tiles.iter()
        .map(|tile| tile.edges.iter().map(move |edge| (edge.clone(), tile)))
        .flatten()
        .collect();

    let origin = tiles.iter()
        .filter(|tile| tile.edges
                .intersection(&edgemap
                              .iter()
                              .filter_map(|(e, o)| if o.id != tile.id { Some(e.clone()) } else { None })
                              .collect())
                .count() == 2)
        .next()
        .unwrap();

    if oritin.image[0]

    let mut arranged = HashMap::new();  
    arranged.insert((0, 0), origin);
    
    let side = (tiles.len() as f64).sqrt() as usize;
    for row in 0..side {
        for col in 0..side {
            if let Some(tile) = arranged.get(&(row, col)) {
                let mut adjoining = 2;
                if col > 0 && col < side - 1 {
                    adjoining += 1;
                }
                if row  > 0 && row < side - 1 {
                    adjoining += 1;
                }
            } else {
                panic!("Can't find neighbors of unplaced tile!");
            }
        }
    }

    arranged
}

fn part1(tiles: &TInput) -> u64 {
    arrange(tiles);
    tiles.iter()
        .map(|tile| {
            (tile.id, tiles.iter()
                .filter(|other| other.id != tile.id)
                .filter(|other| other.edges.intersection(&tile.edges).count() > 0)
                .count())
        })
        .filter(|(id, overlaps)| *overlaps == 2)
        .map(|(id, _)| id)
        .product()
}

fn part2(tiles: &TInput) -> u64 {
    let mut arrangement: Vec<Vec<&Tile>> = vec![];
    let mut image: Vec<Vec<bool>> = vec![];

    let side = (tiles.len() as f64).sqrt() as usize;

    for _ in 0..(side * 10) {
        let mut row = vec![];
        for _ in 0..(side * 10) {
            row.push(false);
        }
        image.push(row);
    }

    let mut current: &Tile = tiles.iter()
        .filter(|tile| {
            tiles.iter()
                .filter(|other| other.id != tile.id)
                .filter(|other| other.edges.intersection(&tile.edges).count() > 0)
                .count() == 2
        })
        .next()
        .unwrap();

    let edges: Vec<&Tile> = tiles.iter()
        .filter(|tile| {
            tiles.iter()
                .filter(|other| other.id != tile.id)
                .filter(|other| other.edges.intersection(&tile.edges).count() > 0)
                .count() == 2
        })
        .collect();

    let mut used: HashSet<u64> = HashSet::new();
    used.insert(current.id);

    for r in 0..side {
        let mut row = vec![];

        if r >= 0 {
        }

        for c in 1..side {

        }

        arrangement.push(row);
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

	let answer1 = part1(&tiles);
	let answer2 = part2(&tiles);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
