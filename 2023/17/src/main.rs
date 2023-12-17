use adventage::day;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

type Point = (i32, i32);
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum Direction {
    Up, Down, Left, Right
}

fn parse(input: &str) -> HashMap<Point, u32> {
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, c)| ((row as i32, col as i32), c.to_digit(10).unwrap()))
                .collect::<HashMap<Point, u32>>()
        })
        .flatten()
        .collect()
}

fn part1(map: &HashMap<Point, u32>) -> u32 {
    crucible(map, 1, 3)
}

fn part2(map: &HashMap<Point, u32>) -> u32 {
    crucible(map, 4, 10)
}

fn crucible(map: &HashMap<Point, u32>, min: i32, max: i32) -> u32 {
    let mut next = BinaryHeap::new();
    let row_max = *map.keys().map(|(row, _)| row).max().unwrap();
    let col_max = *map.keys().map(|(_, col)| col).max().unwrap();
    let mut ud_seen: HashMap<Point, u32> = HashMap::new();
    let mut lr_seen: HashMap<Point, u32> = HashMap::new();

    next.push(Reverse((0u32, (0, 0), vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right])));
    let mut necessary = 0;
    let mut unnecessary = 0;

    while let Some(Reverse((loss, pt, allowed))) = next.pop() {
        if allowed[0] == Direction::Left {
            if let Some(prev) = lr_seen.get(&pt) {
                if *prev <= loss {
                    unnecessary += 1;
                    continue;
                }
            }
            lr_seen.insert(pt, loss);
        } else if allowed[0] == Direction::Up {
            if let Some(prev) = ud_seen.get(&pt) {
                if *prev <= loss {
                    unnecessary += 1;
                    continue;
                }
            }
            ud_seen.insert(pt, loss);
        }

        necessary += 1;

        if pt == (row_max, col_max) {
            println!("{} necessary, {} unnecessary", necessary, unnecessary);
            return loss;
        }


        let mut heat_up = loss;
        let mut heat_down = loss;
        let mut heat_left = loss;
        let mut heat_right = loss;

        for distance in 1..=max {
           let up = (pt.0 - distance, pt.1);
           let down = (pt.0 + distance, pt.1);
           let left = (pt.0, pt.1 - distance);
           let right = (pt.0, pt.1 + distance);

           if allowed.contains(&Direction::Up) {
               if let Some(heat) = map.get(&up) {
                   heat_up += heat;
                   let next_heat = *lr_seen.get(&up).unwrap_or(&u32::MAX);
                   if distance >= min && next_heat > heat_up {
                       next.push(Reverse((heat_up, up, vec![Direction::Left, Direction::Right])))
                   }
                }
            }

           if allowed.contains(&Direction::Down) {
               if let Some(heat) = map.get(&down) {
                   heat_down += heat;
                   if distance >= min {
                       next.push(Reverse((heat_down, down, vec![Direction::Left, Direction::Right])))
                   }
                }
            }

           if allowed.contains(&Direction::Left) {
               if let Some(heat) = map.get(&left) {
                   heat_left += heat;
                   if distance >= min {
                       next.push(Reverse((heat_left, left, vec![Direction::Up, Direction::Down])))
                   }
                }
            }

           if allowed.contains(&Direction::Right) {
               if let Some(heat) = map.get(&right) {
                   heat_right += heat;
                   if distance >= min {
                       next.push(Reverse((heat_right, right, vec![Direction::Up, Direction::Down])))
                   }
                }
            }
        }
    }
    panic!();
}
/*
part1demo!(r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533", 102);
*/
day!(2023, 17);

    #[test] 
    fn pt1_ex1() {
        let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let parsed = parse(input);
        let answer = part1(&parsed);
        assert_eq!(102, answer);
    }

    #[test] 
    fn pt2_ex1() {
        let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let parsed = parse(input);
        let answer = part2(&parsed);
        assert_eq!(94, answer);
    }
