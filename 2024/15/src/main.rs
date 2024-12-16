use adventage::{day, part1demo, part2demo};
use std::collections::HashSet;
use std::convert::TryFrom;

day!(2024, 15);
part1demo!(
    "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
    2028
);
part1demo!(
    "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    10092
);

part2demo!(
    "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    9021
);

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '>' => Ok(Direction::Right),
            '<' => Ok(Direction::Left),
            '^' => Ok(Direction::Up),
            'v' | 'V' => Ok(Direction::Down),
            _ => Err("Bad direction."),
        }
    }
}

type Point = (isize, isize);

#[derive(Clone, Debug)]
struct Warehouse {
    walls: HashSet<Point>,
    boxes: HashSet<Point>,
    robot: Point,
}

type TInput = (Warehouse, Vec<Direction>);

fn parse(input: &str) -> TInput {
    let (warehouse, directions) = input.split_once("\n\n").unwrap();

    let directions = directions
        .chars()
        .filter_map(|d| {
            if let Ok(d) = d.try_into() {
                Some(d)
            } else {
                None
            }
        })
        .collect();

    let warehouse = warehouse
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, c)| ((col as isize, row as isize), c))
        })
        .flatten()
        .fold(
            Warehouse {
                walls: HashSet::new(),
                boxes: HashSet::new(),
                robot: (-1, -1),
            },
            |mut warehouse, (pos, c)| {
                match c {
                    '@' => {
                        warehouse.robot = pos;
                    }
                    'o' | 'O' => {
                        warehouse.boxes.insert(pos);
                    }
                    '#' => {
                        warehouse.walls.insert(pos);
                    }
                    _ => {}
                };

                warehouse
            },
        );

    (warehouse, directions)
}

fn shift_box(
    mut warehouse: &mut Warehouse,
    point: Point,
    direction: Direction,
    mutate: bool,
) -> Point {
    let is_box = warehouse.boxes.contains(&point);
    let neighbors = match direction {
        Direction::Down if is_box => vec![(point.0, point.1 + 1), (point.0 + 1, point.1 + 1)],
        Direction::Up if is_box => vec![(point.0, point.1 - 1), (point.0 + 1, point.1 - 1)],
        Direction::Left if is_box => vec![(point.0 - 1, point.1)],
        Direction::Right if is_box => vec![(point.0 + 2, point.1)],
        Direction::Down => vec![(point.0, point.1 + 1)],
        Direction::Up => vec![(point.0, point.1 - 1)],
        Direction::Left => vec![(point.0 - 1, point.1)],
        Direction::Right => vec![(point.0 + 1, point.1)],
    };

    if neighbors.iter().any(|n| warehouse.walls.contains(n)) {
        return point;
    }

    let box_neighbors: HashSet<Point> = neighbors
        .iter()
        .map(|n| vec![*n, (n.0 - 1, n.1)])
        .flatten()
        .filter(|b| warehouse.boxes.contains(b))
        .collect();

    if box_neighbors
        .iter()
        .all(|b| *b != shift_box(&mut warehouse, *b, direction, false))
    {
        let new_anchor = match direction {
            Direction::Down => (point.0, point.1 + 1),
            Direction::Up => (point.0, point.1 - 1),
            Direction::Left => (point.0 - 1, point.1),
            Direction::Right => (point.0 + 1, point.1),
        };

        if mutate {
            for neighbor in box_neighbors {
                let shifted = shift_box(&mut warehouse, neighbor, direction, mutate);
                warehouse.boxes.remove(&neighbor);
                warehouse.boxes.insert(shifted);
            }
        }

        new_anchor
    } else {
        point
    }
}

fn shift(mut warehouse: &mut Warehouse, point: Point, direction: Direction, mutate: bool) -> Point {
    let neighbor = match direction {
        Direction::Down => (point.0, point.1 + 1),
        Direction::Up => (point.0, point.1 - 1),
        Direction::Left => (point.0 - 1, point.1),
        Direction::Right => (point.0 + 1, point.1),
    };

    if warehouse.walls.contains(&neighbor) {
        point
    } else if warehouse.boxes.contains(&neighbor) {
        let shifted = shift(&mut warehouse, neighbor, direction, mutate);
        if shifted == neighbor {
            point
        } else {
            if mutate {
                warehouse.boxes.remove(&neighbor);
                warehouse.boxes.insert(shifted);
            }
            neighbor
        }
    } else {
        neighbor
    }
}

fn print(warehouse: &Warehouse, big: bool) {
    let max_row = *warehouse.walls.iter().map(|(_, row)| row).max().unwrap();
    let max_col = *warehouse.walls.iter().map(|(col, _)| col).max().unwrap();

    for row in 0..=max_row {
        for col in 0..=max_col {
            if big && warehouse.boxes.contains(&(col, row)) {
                print!("[");
            } else if big && warehouse.boxes.contains(&(col - 1, row)) {
                print!("]");
            } else if warehouse.boxes.contains(&(col, row)) {
                print!("o");
            } else if warehouse.walls.contains(&(col, row)) {
                print!("#");
            } else if warehouse.robot == (col, row) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn embiggen(warehouse: &Warehouse) -> Warehouse {
    Warehouse {
        robot: (warehouse.robot.0 * 2, warehouse.robot.1),
        walls: warehouse
            .walls
            .iter()
            .map(|(col, row)| vec![(col * 2, *row), (col * 2 + 1, *row)])
            .flatten()
            .collect(),
        boxes: warehouse
            .boxes
            .iter()
            .map(|(col, row)| vec![(col * 2, *row)])
            .flatten()
            .collect(),
    }
}

fn part1((warehouse, moves): &TInput) -> isize {
    let mut warehouse = warehouse.clone();
    for mov in moves {
        let robot = warehouse.robot;
        let new_robot = shift(&mut warehouse, robot, *mov, false);
        if new_robot != robot {
            shift(&mut warehouse, robot, *mov, true);
        }
        warehouse.robot = new_robot;
    }

    warehouse
        .boxes
        .iter()
        .map(|(col, row)| (row * 100) + col)
        .sum()
}

fn part2((warehouse, moves): &TInput) -> isize {
    let mut warehouse = embiggen(warehouse);
    for mov in moves {
        let robot = warehouse.robot;
        let new_robot = shift_box(&mut warehouse, robot, *mov, false);
        if new_robot != robot {
            shift_box(&mut warehouse, robot, *mov, true);
        }
        warehouse.robot = new_robot;
    }

    warehouse
        .boxes
        .iter()
        .map(|(col, row)| (row * 100) + col)
        .sum()
}
