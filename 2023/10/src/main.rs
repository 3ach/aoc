use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

type Point = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
    Start,
}

#[derive(Debug)]
struct Map {
    pipes: HashMap<Point, Pipe>,
    x_max: usize,
    y_max: usize,
}

fn goes_north(tile: &Pipe) -> bool {
    match tile {
        Pipe::Vertical => true,
        Pipe::Horizontal => false,
        Pipe::Northeast => false,
        Pipe::Northwest => false,
        Pipe::Southeast => true,
        Pipe::Southwest => true,
        Pipe::Start => true,
    }
}

fn goes_south(tile: &Pipe) -> bool {
    match tile {
        Pipe::Vertical => true,
        Pipe::Horizontal => false,
        Pipe::Northeast => true,
        Pipe::Northwest => true,
        Pipe::Southeast => false,
        Pipe::Southwest => false,
        Pipe::Start => true,
    }
}

fn goes_east(tile: &Pipe) -> bool {
    match tile {
        Pipe::Vertical => false,
        Pipe::Horizontal => true,
        Pipe::Northeast => true,
        Pipe::Northwest => false,
        Pipe::Southeast => true,
        Pipe::Southwest => false,
        Pipe::Start => true,
    }
}

fn goes_west(tile: &Pipe) -> bool {
    match tile {
        Pipe::Vertical => false,
        Pipe::Horizontal => true,
        Pipe::Northeast => false,
        Pipe::Northwest => true,
        Pipe::Southeast => false,
        Pipe::Southwest => true,
        Pipe::Start => true,
    }
}

fn neighbors(point: &Point, map: &Map) -> Vec<Point> {
    let mut neighbors = vec![];

    if point.0 > 0 {
        neighbors.push((point.0 - 1, point.1));
    }

    if point.0 < map.x_max {
        neighbors.push((point.0 + 1, point.1));
    }

    if point.1 > 0 {
        neighbors.push((point.0, point.1 - 1));
    }

    if point.1 < map.y_max {
        neighbors.push((point.0, point.1 + 1));
    }

    neighbors
}

fn piped_neighbors(point: &Point, map: &Map) -> Vec<Point> {
    let mut neighbors = vec![];

    if point.0 > 0 && goes_west(map.pipes.get(point).unwrap()) {
        if let Some(tile) = map.pipes.get(&(point.0 - 1, point.1)) {
            if goes_east(&tile) {
                neighbors.push((point.0 - 1, point.1));
            }
        }
    }

    if point.0 < map.x_max && goes_east(map.pipes.get(point).unwrap()) {
        if let Some(tile) = map.pipes.get(&(point.0 + 1, point.1)) {
            if goes_west(&tile) {
                neighbors.push((point.0 + 1, point.1));
            }
        }
    }

    if point.1 > 0 && goes_north(map.pipes.get(point).unwrap()) {
        if let Some(tile) = map.pipes.get(&(point.0, point.1 - 1)) {
            if goes_south(&tile) {
                neighbors.push((point.0, point.1 - 1));
            }
        }
    }

    if point.1 < map.y_max && goes_south(map.pipes.get(point).unwrap()) {
        if let Some(tile) = map.pipes.get(&(point.0, point.1 + 1)) {
            if goes_north(&tile) {
                neighbors.push((point.0, point.1 + 1));
            }
        }
    }

    neighbors
}

fn pipe(input: &Map) -> HashMap<Point, usize> {
    let start = *input
        .pipes
        .iter()
        .filter(|(_, p)| **p == Pipe::Start)
        .next()
        .unwrap()
        .0;

    let mut distances = HashMap::new();
    let mut to_see = vec![(start, 0usize)];

    while let Some((current, steps)) = to_see.pop() {
        let curr = current.clone();
        if let Some(distance) = distances.get(&curr) {
            if *distance > steps {
                distances.insert(current, steps);
                to_see.append(
                    &mut piped_neighbors(&current, input)
                        .iter_mut()
                        .map(|neighbor| (*neighbor, steps + 1))
                        .collect(),
                )
            }
        } else {
            distances.insert(current, steps);
            to_see.append(
                &mut piped_neighbors(&current, input)
                    .iter_mut()
                    .map(|neighbor| (*neighbor, steps + 1))
                    .collect(),
                );
        }
    }

    distances
}

fn part1(input: &Map) -> usize {
    *pipe(input).values().max().unwrap()
}

fn biggen(point: Point) -> Point {
    ((3 * point.0) + 1, (3 * point.1) +1)
}

fn part2(map: &Map) -> usize {
    let bigpipes = map.pipes.iter()
        .map(|(point, tile)| {
            let bigpoint = biggen(*point);
            let mut zoom = vec![(bigpoint, *tile)];

            if point.0 > 0 && goes_west(tile) {
                zoom.push(((bigpoint.0 - 1, bigpoint.1), Pipe::Horizontal));
            }

            if point.0 < map.x_max && goes_east(tile) {
                zoom.push(((bigpoint.0 + 1, bigpoint.1), Pipe::Horizontal));
            }

            if point.1 < map.y_max && goes_south(tile) {
                zoom.push(((bigpoint.0, bigpoint.1 + 1), Pipe::Vertical));
            }

            if point.1 > 0 && goes_north(tile) {
                zoom.push(((bigpoint.0, bigpoint.1 - 1), Pipe::Vertical));
            }

            zoom
        }).flatten()
    .collect::<HashMap<Point, Pipe>>();

    let bigmap = Map { x_max: (map.x_max * 3) + 2, y_max: (map.y_max * 3) + 2, pipes: bigpipes };

    let smallpipe = pipe(&map).keys().cloned().collect::<HashSet<Point>>();
    let bigpipe = pipe(&bigmap).keys().cloned().collect::<HashSet<Point>>();

    let mut unexplored = vec![(0, 0)];
    let mut unenclosed: HashSet<Point> = HashSet::new();

    while let Some(current) = unexplored.pop() {
        if unenclosed.contains(&current) || bigpipe.contains(&current) {
            continue;
        }

        unenclosed.insert(current);
        unexplored.append(&mut neighbors(&current, &bigmap));
    }

    ((map.x_max + 1) * (map.y_max + 1)) - smallpipe.len()
       - (0..=map.y_max)
        .map(|y|
             (0..=map.x_max)
             .filter(|x| unenclosed.contains(&biggen((*x, y))))
             .count())
        .sum::<usize>()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut x_max = 0;
    let mut y_max = 0;

    let pipes = reader
        .lines()
        .map(|line| line.expect("Couldn't read stdin"))
        .enumerate()
        .map(|(row, line)| {
            if row > y_max {
                y_max = row;
            }
            if line.len() > x_max {
                x_max = line.len() - 1;
            }

            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| {
                    match c {
                        '|' => Some(((col, row), Pipe::Vertical)),
                        '-' => Some(((col, row), Pipe::Horizontal)),
                        'F' => Some(((col, row), Pipe::Northeast)),
                        'J' => Some(((col, row), Pipe::Southwest)),
                        'L' => Some(((col, row), Pipe::Southeast)),
                        '7' => Some(((col, row), Pipe::Northwest)),
                        'S' => Some(((col, row), Pipe::Start)),
                        _ => None,
                    }
                })
                .collect::<Vec<(Point, Pipe)>>()
        })
        .flatten()
        .collect::<HashMap<Point, Pipe>>();

    let map = Map {
        x_max,
        y_max,
        pipes,
    };

    let answer1 = part1(&map);
    let answer2 = part2(&map);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}
