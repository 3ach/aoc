use adventage::{day, part1demo, part2demo};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::cmp::Reverse;

day!(2019, 20);
part1demo!(
    "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ",
    23
);

part1demo!(
    "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ",
    58
);

part2demo!(
"             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ", 396);

type TInput = HashMap<(String, bool), HashMap<(String, bool), usize>>;
type Point = (isize, isize);

fn parse(input: &str) -> TInput {
    let row_count = input.lines().count() as isize;
    let col_count = input.lines().next().unwrap().chars().count() as isize;

    let map: HashMap<Point, char> = input
        .lines()
        .enumerate()
        .map(|(ridx, l)| {
            l.chars()
                .enumerate()
                .map(move |(cidx, c)| ((ridx as isize, cidx as isize), c))
        })
        .flatten()
        .collect();
    let mut skip_next = false;
    let mut tags = HashMap::new();

    // vertical tags
    for row in 0..(row_count - 1) {
        if skip_next {
            skip_next = false;
            continue;
        }

        for col in 2..(col_count - 2) {
            if !map[&(row, col)].is_alphabetic() {
                continue;
            }

            if !map[&(row + 1, col)].is_alphabetic() {
                continue;
            }

            let point = if row == 0 || map[&(row - 1, col)] == ' ' {
                (row + 2, col)
            } else {
                (row - 1, col)
            };

            tags.insert(point, format!("{}{}", map[&(row, col)], map[&(row + 1, col)]));
            skip_next = true;
        }
    }

    skip_next = false;

    // horizontal tags
    for col in 0..(col_count - 1) {
        if skip_next {
            skip_next = false;
        }

        for row in 2..(row_count - 2) {
            if !map[&(row, col)].is_alphabetic() {
                continue;
            }

            if !map[&(row, col + 1)].is_alphabetic() {
                continue;
            }

            let point = if col == 0 || map[&(row, col - 1)] == ' ' {
                (row, col + 2)
            } else {
                (row, col - 1)
            };

            tags.insert(point, format!("{}{}", map[&(row, col)], map[&(row, col + 1)]));
            skip_next = true;
        }
    }

    let mut adjacent = HashMap::new();

    for (point, tag) in &tags {
        let mut seen = HashSet::new();
        let mut to_explore = VecDeque::from([(*point, 0)]);

        let point_outer = point.1 == 2
            || point.1 == col_count - 3
            || point.0 == 2 
            || point.0 == row_count - 3;

        while let Some((current, distance)) = to_explore.pop_front() {
            if !seen.insert(current) {
                continue;
            }

            let outer = current.1 == 2 
                || current.1 == col_count - 3
                || current.0 == 2
                || current.0 == row_count - 3;

            if let Some(other) = tags.get(&current)
                && distance > 0
            {

                adjacent
                    .entry((tag.clone(), point_outer))
                    .or_insert(HashMap::new())
                    .entry((other.clone(), outer))
                    .or_insert(distance + 1);
            }

            for neighbor in neighbors(current) {
                if seen.contains(&neighbor) {
                    continue;
                }

                if let Some(tile) = map.get(&neighbor) && *tile == '.' {
                    to_explore.push_back((neighbor, distance + 1));
                }
            }
        }
    }

    adjacent
}

fn neighbors(pt: Point) -> [Point; 4] {
    [
        (pt.0 - 1, pt.1),
        (pt.0 + 1, pt.1),
        (pt.0, pt.1 - 1),
        (pt.0, pt.1 + 1),
    ]
}

fn part1(maze: &TInput) -> usize {
    let mut to_explore = BinaryHeap::from([Reverse((0, "AA", true))]);
    let mut seen: HashSet<(String, bool)> = HashSet::new();

    while let Some(Reverse((distance, portal, outside))) = to_explore.pop() {
        if !seen.insert((String::from(portal), outside)) {
            continue;
        }

        if portal == "ZZ" {
            return distance - 1;
        }

        if let Some(next) = maze.get(&(portal.to_string(), outside)) {
            for ((next_portal, next_outside), steps) in next {
                if seen.contains(&(next_portal.clone(), *next_outside)) {
                    continue;
                }

                to_explore.push(Reverse((steps + distance, next_portal, *next_outside)));
            }
        }

        if !seen.contains(&(String::from(portal), !outside)) {
            to_explore.push(Reverse((distance, portal, !outside)));
        }
    }

    panic!();
}

fn part2(maze: &TInput) -> usize {
    let mut to_explore = BinaryHeap::from([Reverse((0, "AA", true, 0, vec![]))]);
    let mut seen: HashSet<(String, bool, isize)> = HashSet::new();

    while let Some(Reverse((distance, portal, outside, level, path))) = to_explore.pop() {
        if !seen.insert((String::from(portal), outside, level)) || level < 0 {
            continue;
        }

        if portal == "ZZ" && level == 0 {
            return distance - 1;
        }

        if let Some(next) = maze.get(&(portal.to_string(), outside)) {
            for ((next_portal, next_outside), steps) in next {
                if seen.contains(&(next_portal.clone(), *next_outside, level)) {
                    continue;
                }

                let path = path.clone();

                to_explore.push(Reverse((steps + distance, next_portal, *next_outside, level, path)));
            }
        }

        let next_level = if outside {
            level - 1
        } else {
            level + 1
        };

        if !seen.contains(&(String::from(portal), !outside, next_level)) && portal != "ZZ" {
            let mut path = path.clone();
            path.push((portal.to_string(), !outside, next_level));
            to_explore.push(Reverse((distance, portal, !outside, next_level, path)));
        }
    }

    panic!();
}
