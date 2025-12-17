use adventage::{day, part1demo, part2demo};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{BTreeSet, BinaryHeap, HashMap, VecDeque};

day!(2019, 18);

part1demo!(
    "#########
#b.A.@.a#
#########",
    8
);

part1demo!(
    "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################",
    86
);

part1demo!(
    "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################",
    132
);

part1demo!(
    "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################",
    136
);

part1demo!(
    "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################",
    81
);

part2demo!(
    "#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######",
    8
);

part2demo!(
    "###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############",
    24
);

part2demo!(
    "#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############",
    32
);

part2demo!(
    "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############",
    72
);

type Point = (isize, isize);
type Adjacency = HashMap<char, HashMap<char, (usize, u32)>>;
type TInput = HashMap<Point, char>;

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    cost: usize,
    current: Vec<char>,
    keys: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then(key_count(self.keys).cmp(&key_count(other.keys)))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors(pt: Point) -> [Point; 4] {
    [
        (pt.0 - 1, pt.1),
        (pt.0 + 1, pt.1),
        (pt.0, pt.1 - 1),
        (pt.0, pt.1 + 1),
    ]
}

fn parse(input: &str) -> TInput {
    input
        .lines()
        .enumerate()
        .map(|(ridx, line)| {
            line.chars()
                .enumerate()
                .map(move |(cidx, c)| ((cidx as isize, ridx as isize), char::from(c)))
        })
        .flatten()
        .collect()
}

fn flood(map: &TInput, point: &Point) -> HashMap<char, (usize, u32)> {
    let mut to_explore = VecDeque::from([(*point, 0, 0)]);
    let mut seen = BTreeSet::new();
    let mut distances = HashMap::new();

    while let Some((current, distance, mut needed)) = to_explore.pop_back() {
        if !seen.insert(current) {
            continue;
        }

        let tile = map.get(&current).unwrap();
        if *tile == '#' {
            continue;
        }

        if is_door(*tile) {
            needed = add_key(needed, tile.to_ascii_lowercase());
        } else if is_key(*tile) && distance > 0 {
            let best = distances.entry(tile.clone()).or_insert((usize::MAX, 0));
            if distance < best.0 {
                best.0 = distance;
                best.1 = needed.clone();
            }
        }

        for neighbor in neighbors(current) {
            if let Some(n_tile) = map.get(&neighbor) {
                if *n_tile != '#' {
                    to_explore.push_front((neighbor, distance + 1, needed.clone()));
                }
            }
        }
    }

    distances
}

fn find_adjacent(map: &TInput) -> Adjacency {
    map.iter()
        .filter(|(_, tile)| tile.is_ascii_digit() || is_key(**tile) || **tile == '@')
        .map(|(point, tile)| (tile.clone(), flood(map, point)))
        .collect()
}

fn is_key(potential: char) -> bool {
    potential != '#'
        && potential != '.'
        && potential != '@'
        && potential.to_ascii_lowercase() == potential
        && !potential.is_ascii_digit()
}

fn is_door(potential: char) -> bool {
    potential != '#'
        && potential != '.'
        && potential != '@'
        && potential.to_ascii_uppercase() == potential
        && !potential.is_ascii_digit()
}

fn add_key(existing: u32, key: char) -> u32 {
    existing | (1 << key as u8 - 'a' as u8)
}

fn has_key(keys: u32, key: char) -> bool {
    keys & (1 << key as u8 - 'a' as u8) != 0
}

fn has_all(keys: u32, needed: u32) -> bool {
    (needed & keys) == needed
}

fn key_count(keys: u32) -> usize {
    (0..32).filter(|b| keys & (1 << b) != 0).count()
}

fn solve(map: &TInput, start: Vec<char>) -> usize {
    let adjacent = find_adjacent(map);
    let total_keys = adjacent.iter().map(|k| k.0).filter(|k| is_key(**k)).count();
    let mut to_explore = BinaryHeap::from([State {
        cost: 0,
        current: start,
        keys: 0,
    }]);

    let mut best: HashMap<(Vec<char>, u32), usize> = HashMap::new();

    while let Some(state) = to_explore.pop() {
        if total_keys == key_count(state.keys) {
            return state.cost;
        }

        for bot in 0..state.current.len() {
            let current = state.current[bot];

            for (tile, (distance, needed)) in &adjacent[&current] {
                if has_key(state.keys, *tile) || !has_all(state.keys, *needed) {
                    continue;
                }

                let next_keys = add_key(state.keys, *tile);
                let next_steps = state.cost + distance;
                let mut next_bots = state.current.clone();
                next_bots[bot] = *tile;

                let best = best
                    .entry((next_bots.clone(), next_keys.clone()))
                    .or_insert(usize::MAX);

                if *best > next_steps {
                    *best = next_steps;
                    let next = State {
                        cost: next_steps,
                        current: next_bots,
                        keys: next_keys,
                    };

                    to_explore.push(next);
                }
            }
        }
    }

    panic!()
}

fn part1(input: &TInput) -> usize {
    solve(input, vec!['@'])
}

fn part2(input: &TInput) -> usize {
    let center = input
        .iter()
        .filter_map(|(pos, tile)| if *tile == '@' { Some(pos) } else { None })
        .next()
        .unwrap();

    let mut adjusted = input.clone();
    adjusted.insert(*center, '#');
    adjusted.insert((center.0, center.1 + 1), '#');
    adjusted.insert((center.0, center.1 - 1), '#');
    adjusted.insert((center.0 + 1, center.1), '#');
    adjusted.insert((center.0 - 1, center.1), '#');
    adjusted.insert((center.0 - 1, center.1 - 1), '1');
    adjusted.insert((center.0 - 1, center.1 + 1), '2');
    adjusted.insert((center.0 + 1, center.1 - 1), '3');
    adjusted.insert((center.0 + 1, center.1 + 1), '4');

    solve(&adjusted, vec!['1', '2', '3', '4'])
}
