use adventage::day;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

enum Splitter {
    Vertical,
    Horizontal,
}

enum Mirror {
    West,
    East,
}

enum Tile {
    Mirror(Mirror),
    Splitter(Splitter),
    Empty,
}

type Point = (usize, usize);

fn part2(map: &HashMap<Point, Tile>) -> usize {
    let mut max = 0;
    let row_max = *map.keys().map(|(row, _)| row).max().unwrap();
    let col_max = *map.keys().map(|(_, col)| col).max().unwrap();

    for row in 0..=row_max {
        for col in 0..=col_max {
            if row == 0 {
                let energized = energize(((row, col), Direction::South), map);
                if energized > max {
                    max = energized;
                }
            }

            if row == row_max {
                let energized = energize(((row, col), Direction::North), map);
                if energized > max {
                    max = energized;
                }
            }

            if col == 0 {
                let energized = energize(((row, col), Direction::East), map);
                if energized > max {
                    max = energized;
                }
            }
            
            if col == col_max {
                let energized = energize(((row, col), Direction::West), map);
                if energized > max {
                    max = energized;
                }
            }
        }
    }

    max
}

fn part1(map: &HashMap<Point, Tile>) -> usize {
    energize(((0, 0), Direction::East), map)
}

fn energize(start: (Point, Direction), map: &HashMap<Point, Tile>) -> usize {
    let mut energized = HashSet::new();
    let mut to_energize = vec![start];

    let row_max = *map.keys().map(|(row, _)| row).max().unwrap();
    let col_max = *map.keys().map(|(_, col)| col).max().unwrap();

    while let Some((pt, dir)) = to_energize.pop() {
        if !energized.insert((pt, dir)) {
            continue;
        }

        let tile = map.get(&pt).unwrap();

        match (dir, tile) {
            (Direction::North, Tile::Empty) if pt.0 > 0 => { to_energize.push(((pt.0 - 1, pt.1), Direction::North)); },
            (Direction::North, Tile::Splitter(Splitter::Vertical)) if pt.0 > 0 => { to_energize.push(((pt.0 - 1, pt.1), Direction::North)); },
            (Direction::North, Tile::Splitter(Splitter::Horizontal)) => { 
                if pt.1 > 0 {
                    to_energize.push(((pt.0, pt.1 - 1), Direction::West));
                }

                if pt.1 < row_max {
                    to_energize.push(((pt.0, pt.1 + 1), Direction::East));
                }
            },
            (Direction::North, Tile::Mirror(Mirror::West)) => { 
                if pt.1 < row_max {
                    to_energize.push(((pt.0, pt.1 + 1), Direction::East));
                }
            },
            (Direction::North, Tile::Mirror(Mirror::East)) => { 
                if pt.1 > 0 {
                    to_energize.push(((pt.0, pt.1 - 1), Direction::West));
                }
            },

            (Direction::South, Tile::Empty) if pt.0 < row_max => { to_energize.push(((pt.0 + 1, pt.1), Direction::South)); },
            (Direction::South, Tile::Splitter(Splitter::Vertical)) if pt.0 < row_max => { to_energize.push(((pt.0 + 1, pt.1), Direction::South)); },
            (Direction::South, Tile::Splitter(Splitter::Horizontal)) => { 
                if pt.1 > 0 {
                    to_energize.push(((pt.0, pt.1 - 1), Direction::West));
                }

                if pt.1 < row_max {
                    to_energize.push(((pt.0, pt.1 + 1), Direction::East));
                }
            },
            (Direction::South, Tile::Mirror(Mirror::West)) => { 
                if pt.1 > 0 {
                    to_energize.push(((pt.0, pt.1 - 1), Direction::West));
                }
            },
            (Direction::South, Tile::Mirror(Mirror::East)) => { 
                if pt.1 < col_max {
                    to_energize.push(((pt.0, pt.1 + 1), Direction::East));
                }
            },

            (Direction::East, Tile::Empty) if pt.1 < col_max => { to_energize.push(((pt.0, pt.1 + 1), Direction::East)); },
            (Direction::East, Tile::Splitter(Splitter::Horizontal)) if pt.1 < col_max => { to_energize.push(((pt.0, pt.1 + 1), Direction::East)); },
            (Direction::East, Tile::Splitter(Splitter::Vertical)) => { 
                if pt.0 > 0 {
                    to_energize.push(((pt.0 - 1, pt.1), Direction::North));
                }

                if pt.0 < col_max {
                    to_energize.push(((pt.0 + 1, pt.1), Direction::South));
                }
            },
            (Direction::East, Tile::Mirror(Mirror::West)) => { 
                if pt.0 > 0 {
                    to_energize.push(((pt.0 - 1, pt.1), Direction::North));
                }
            },
            (Direction::East, Tile::Mirror(Mirror::East)) => { 
                if pt.0 < row_max {
                    to_energize.push(((pt.0 + 1, pt.1), Direction::South));
                }
            },

            (Direction::West, Tile::Empty) if pt.1 > 0  => { to_energize.push(((pt.0, pt.1 - 1), Direction::West)); },
            (Direction::West, Tile::Splitter(Splitter::Horizontal)) if pt.1 > 0 => { to_energize.push(((pt.0, pt.1 - 1), Direction::West)); },
            (Direction::West, Tile::Splitter(Splitter::Vertical)) => { 
                if pt.0 > 0 {
                    to_energize.push(((pt.0 - 1, pt.1), Direction::North));
                }

                if pt.0 < col_max {
                    to_energize.push(((pt.0 + 1, pt.1), Direction::South));
                }
            },
            (Direction::West, Tile::Mirror(Mirror::East)) => {
                if pt.0 > 0 {
                    to_energize.push(((pt.0 - 1, pt.1), Direction::North));
                }
            },
            (Direction::West, Tile::Mirror(Mirror::West)) => {
                if pt.0 < col_max {
                    to_energize.push(((pt.0 + 1, pt.1), Direction::South));
                }
            }
            _ => {},
        }
    }

    energized.iter()
        .map(|(pt, _)| pt)
        .collect::<HashSet<&Point>>()
        .len()
}

#[day]
fn parse() -> HashMap<Point, Tile> {
    input
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| match c {
                    '-' => Some(((row, col), Tile::Splitter(Splitter::Horizontal))),
                    '|' => Some(((row, col), Tile::Splitter(Splitter::Vertical))),
                    '/' => Some(((row, col), Tile::Mirror(Mirror::West))),
                    '\\' => Some(((row, col), Tile::Mirror(Mirror::East))),
                    '.' => Some(((row, col), Tile::Empty)),
                    _ => None,
                })
                .collect::<HashMap<Point, Tile>>()
        })
        .flatten()
        .collect::<HashMap<Point, Tile>>()
}
