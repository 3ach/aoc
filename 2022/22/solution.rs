use std::cmp;
use std::fmt;
use std::io;
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
enum Move {
    Forward(usize),
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Void,
    Floor,
    Wall,
}

#[derive(Debug, Copy, Clone)]
struct Face {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
    x_start: usize,
    x_end: usize,
    y_start: usize,
    y_end: usize,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Void => write!(f, " "),
            Tile::Floor => write!(f, "."),
            Tile::Wall => write!(f, "#"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(usize)]
enum Direction {
    Right = 0usize,
    Down = 1usize,
    Left = 2usize,
    Up = 3usize,
}

fn build_cube(map: &Vec<Vec<Tile>>) -> [Face; 6] {
    let mut faces = [Face {
        up: 6,
        down: 6,
        left: 6,
        right: 6,
        x_start: 0,
        x_end: 0,
        y_start: 0,
        y_end: 0,
    }; 6];
    let side = map
        .iter()
        .map(|r| {
            r.iter()
                .enumerate()
                .filter_map(|(idx, t)| match t {
                    Tile::Floor | Tile::Wall => Some(idx),
                    _ => None,
                })
                .max()
                .unwrap()
                - r.iter()
                    .enumerate()
                    .filter_map(|(idx, t)| match t {
                        Tile::Floor | Tile::Wall => Some(idx),
                        _ => None,
                    })
                    .min()
                    .unwrap()
                + 1
        })
        .min()
        .unwrap();

    let vert_faces = map.len() / side;
    let mut next_face = 0;
    let mut prev_row = [None; 6];

    for face_y in 0..vert_faces {
        let y = side * face_y;
        let horiz_faces = map[y].len() / side;
        let mut left = None;
        let mut current_row = [None; 6];
        for face_x in 0..horiz_faces {
            let x = side * face_x;
            let corner = map[y][x];
            if corner == Tile::Void {
                continue;
            }

            faces[next_face].x_start = x;
            faces[next_face].x_end = x + side;
            faces[next_face].y_start = y;
            faces[next_face].y_end = y + side;

            if let Some(left) = left {
                faces[next_face].left = left;
                faces[left].right = next_face;
            }

            if let Some(up) = prev_row[face_x] {
                faces[next_face].up = up;
                faces[up].down = next_face;
            }

            left = Some(next_face);
            current_row[face_x] = Some(next_face);
            next_face += 1;
        }

        prev_row = current_row;
    }

    loop {
        let mut finished = true;
        for face_idx in 0..6 {
            let face = faces[face_idx];
            finished &= face.up != 6 && face.left != 6 && face.right != 6 && face.left != 6;

            if face.up < 6 {
                if face.right < 6 {
                    let right = &mut faces[face.right];

                    if right.left == face_idx {
                        right.up = face.up;
                    } else if right.up == face_idx {
                        right.right = face.up;
                    } else if right.right == face_idx {
                        right.down = face.up;
                    }

                    let up = &mut faces[face.up];
                    if up.down == face_idx {
                        up.right = face.right;
                    } else if up.right == face_idx {
                        up.up = face.right;
                    } else if up.up == face_idx {
                        up.left = face.right;
                    }
                }

                if face.left < 6 {
                    let left = &mut faces[face.left];

                    if left.right == face_idx {
                        left.up = face.up;
                    } else if left.up == face_idx {
                        left.left = face.up;
                    } else if left.left == face_idx {
                        left.down = face.up;
                    }

                    let up = &mut faces[face.up];
                    if up.down == face_idx {
                        up.left = face.left;
                    } else if up.left == face_idx {
                        up.up = face.left;
                    } else if up.down == face_idx {
                        up.right = face.left;
                    }
                }
            }
            if face.down < 6 {
                if face.right < 6 {
                    let right = &mut faces[face.right];

                    if right.left == face_idx {
                        right.down = face.down;
                    } else if right.down == face_idx {
                        right.right = face.down;
                    } else if right.right == face_idx {
                        right.up = face.down;
                    }

                    let down = &mut faces[face.down];
                    if down.up == face_idx {
                        down.right = face.right;
                    } else if down.right == face_idx {
                        down.down = face.right;
                    } else if down.down == face_idx {
                        down.left = face.right;
                    }
                }

                if face.left < 6 {
                    let left = &mut faces[face.left];

                    if left.right == face_idx {
                        left.down = face.down;
                    } else if left.down == face_idx {
                        left.left = face.down;
                    } else if left.left == face_idx {
                        left.up = face.down;
                    }

                    let down = &mut faces[face.down];
                    if down.up == face_idx {
                        down.left = face.left;
                    } else if down.left == face_idx {
                        down.down = face.left;
                    } else if down.down == face_idx {
                        down.right = face.left;
                    }
                }
            }
        }

        if finished {
            break;
        }
    }

    faces
}

fn right(dir: Direction) -> Direction {
    match dir {
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
    }
}

fn left(dir: Direction) -> Direction {
    match dir {
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Up => Direction::Left,
    }
}

fn forward(position: (usize, usize), direction: Direction) -> (isize, isize) {
    let x = match direction {
        Direction::Right => position.0 as isize + 1,
        Direction::Left => position.0 as isize - 1,
        _ => position.0 as isize,
    };

    let y = match direction {
        Direction::Down => position.1 as isize + 1,
        Direction::Up => position.1 as isize - 1,
        _ => position.1 as isize,
    };

    (x, y)
}

fn contains(face: &Face, point: &(isize, isize)) -> bool {
    let x = point.0 as usize;
    let y = point.1 as usize;

    face.x_start <= x && face.x_end > x && face.y_start <= y && face.y_end > y
}

fn teleport(
    current: (usize, usize, Direction),
    next: (isize, isize, Direction),
    map: &Vec<Vec<Tile>>,
    cube: &[Face; 6],
    cubic: bool,
) -> (usize, usize, Direction) {
    if cubic {
        let (current_face_idx, current_face) = cube
            .iter()
            .enumerate()
            .filter(|(_, face)| contains(face, &(current.0 as isize, current.1 as isize)))
            .next()
            .unwrap();

        let next_face = cube
            .iter()
            .filter(|face| contains(face, &(next.0, next.1)))
            .next();

        if let None = next_face {
            let (reference_face_idx, next_face_idx) = match current.2 {
                Direction::Up => (
                    cmp::min(current_face.left, current_face.right),
                    current_face.up,
                ),
                Direction::Down => (
                    cmp::min(current_face.left, current_face.right),
                    current_face.down,
                ),
                Direction::Left => (
                    cmp::min(current_face.up, current_face.down),
                    current_face.left,
                ),
                Direction::Right => (
                    cmp::min(current_face.up, current_face.down),
                    current_face.right,
                ),
            };


            let next_face = cube[next_face_idx];
            let offset = if reference_face_idx == current_face.up {
                current.1.abs_diff(current_face.y_start)
            } else if reference_face_idx == current_face.down {
                (current_face.y_end - 1).abs_diff(current.1)
            } else if reference_face_idx == current_face.left {
                current.0.abs_diff(current_face.x_start)
            } else if reference_face_idx == current_face.right {
                (current_face.x_end - 1).abs_diff(current.0)
            } else {
                panic!()
            };
            println!("Jumping from point {:?} : {} to {}, using {} as a reference face with offset {}", current, current_face_idx, next_face_idx, reference_face_idx, offset);

            let direction = if current_face_idx == cube[next_face_idx].up {
                Direction::Down
            } else if current_face_idx == cube[next_face_idx].down {
                Direction::Up
            } else if current_face_idx == cube[next_face_idx].left {
                Direction::Right
            } else if current_face_idx == cube[next_face_idx].right {
                Direction::Left
            } else {
                panic!()
            };

            println!("New direction {:?}", direction);

            let x = match direction {
                Direction::Left => next_face.x_end - 1,
                Direction::Right => next_face.x_start,
                Direction::Up | Direction::Down => {
                    if reference_face_idx == next_face.right {
                        next_face.x_end - offset - 1
                    } else if reference_face_idx == next_face.left {
                        next_face.x_start + offset
                    } else {
                        panic!();
                    }
                }
            };

            let y = match direction {
                Direction::Up => next_face.y_end - 1,
                Direction::Down => next_face.y_start,
                Direction::Right | Direction::Left => {
                    if reference_face_idx == next_face.down {
                        next_face.y_end - offset - 1
                    } else if reference_face_idx == next_face.up {
                        next_face.y_start + offset
                    } else {
                        panic!();
                    }
                },
            };

            println!("Next point: {:?}", (x, y));

            return (x, y, direction);
        } else {
            return (next.0 as usize, next.1 as usize, next.2);
        }
    } else {
        let mut result = (next.0, next.1);
        if next.1 >= map.len() as isize {
            result.1 = 0;
        } else if result.1 < 0 {
            result.1 = map.len() as isize - 1;
        }

        if result.0 >= map[result.1 as usize].len() as isize {
            result.0 = 0;
        } else if result.0 < 0 {
            result.0 = map[result.1 as usize].len() as isize - 1;
        }

        return (result.0 as usize, result.1 as usize, next.2);
    }
}

fn run(map: &Vec<Vec<Tile>>, moves: &[Move], cubic: bool) -> (usize, usize, usize) {
    let initial_col = map[0]
        .iter()
        .enumerate()
        .filter_map(|(idx, t)| if let Tile::Floor = t { Some(idx) } else { None })
        .next()
        .unwrap();

    let cube = build_cube(&map);

    let mut pos = (initial_col, 0, Direction::Right);

    for mov in moves {
        println!("Handling move {:?} from {:?}", mov, pos);
        pos = match mov {
            Move::Left => (pos.0, pos.1, left(pos.2)),
            Move::Right => (pos.0, pos.1, right(pos.2)),
            Move::Forward(steps) => {
                for _ in 0..*steps {
                    let mut current = pos;
                    loop {
                        let next = forward((current.0, current.1), current.2);
                        let next = (next.0, next.1, current.2);
                        let mut next = teleport(current, next, map, &cube, cubic);

                        if let Tile::Floor = map[next.1][next.0] {
                            current = (next.0, next.1, next.2);
                            pos = current;
                            break;
                        } else if let Tile::Void = map[next.1][next.0] {
                            current = (next.0, next.1, next.2);
                        } else {
                            break;
                        }
                    }
                }
                pos
            }
        };
    }
    (pos.0, pos.1, pos.2 as usize)
}

fn print_map(map: &Vec<Vec<Tile>>, positions: &[(usize, usize)]) {
    for (cidx, line) in map.iter().enumerate() {
        for (ridx, cell) in line.iter().enumerate() {
            if positions.contains(&(ridx, cidx)) {
                print!("X");
            } else {
                print!("{:?}", cell);
            }
        }
        println!("");
    }
    println!("");
}

fn part1(map: &Vec<Vec<Tile>>, moves: &[Move]) -> usize {
    let end = run(map, moves, false);

    (1000 * (end.1 + 1)) + (4 * (end.0 + 1)) + end.2
}

fn part2(map: &Vec<Vec<Tile>>, moves: &[Move]) -> usize {
    let end = run(map, moves, true);
    println!("End: {:?}", end);
    (1000 * (end.1 + 1)) + (4 * (end.0 + 1)) + end.2
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut map = vec![];
    let mut moves = vec![];
    let mut longest_line = 0;

    for line in reader.lines() {
        let line = line.expect("Couldn't read stdin");
        if line == "" {
            continue;
        }

        if line.starts_with(" ") || line.starts_with(".") || line.starts_with("#") {
            let row: Vec<Tile> = line
                .chars()
                .map(|c| match c {
                    ' ' => Tile::Void,
                    '.' => Tile::Floor,
                    '#' => Tile::Wall,
                    _ => panic!("Unsupported map character"),
                })
                .collect();
            longest_line = cmp::max(longest_line, row.len());
            map.push(row);
        } else {
            moves = line
                .chars()
                .map(|c| match c {
                    'L' => Move::Left,
                    'R' => Move::Right,
                    '0'..='9' => Move::Forward(c as usize - '0' as usize),
                    _ => panic!(),
                })
                .collect();
        }
    }

    for row in &mut map {
        if row.len() < longest_line {
            for _ in row.len()..longest_line {
                row.push(Tile::Void);
            }
        }
    }

    moves = moves.iter().fold(vec![], |mut folded, mov| {
        if let (Some(Move::Forward(ten_steps)), Move::Forward(steps)) = (folded.last(), mov) {
            let steps = (ten_steps * 10) + steps;
            folded.pop();
            folded.push(Move::Forward(steps));
        } else {
            folded.push(*mov);
        }

        folded
    });

    let answer1 = part1(&map, &moves);
    let answer2 = part2(&map, &moves);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}
