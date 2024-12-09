use adventage::{day, part1demo, part2demo};
use std::iter;

day!(2024, 9);
part1demo!("2333133121414131402", 1928);
part2demo!("2333133121414131402", 2858);

type TInput = Vec<u128>;

fn parse(input: &str) -> TInput {
    input
        .chars()
        .filter_map(|c| (c as u128).checked_sub('0' as u128))
        .collect()
}

fn part1(map: &TInput) -> u128 {
    let mut disk = map
        .iter()
        .enumerate()
        .map(|(idx, size)| {
            let block = if idx % 2 == 0 {
                Some(idx as u128 / 2)
            } else {
                None
            };
            iter::repeat_n(block, *size as usize)
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut start = 0;
    let mut end = disk.len() - 1;

    while end >= start {
        if disk[end].is_none() {
            end -= 1;
        } else if disk[start].is_none() {
            disk.swap(start, end)
        } else {
            start += 1;
        }
    }

    disk.iter()
        .enumerate()
        .map(|(idx, v)| match v {
            Some(v) => *v * (idx as u128),
            None => 0,
        })
        .sum()
}

fn part2(map: &TInput) -> u128 {
    let mut disk = map
        .iter()
        .enumerate()
        .map(|(idx, size)| (idx % 2 == 0, idx / 2, *size))
        .collect::<Vec<_>>();

    let mut offset = 0;

    for p in (0..disk.len()).rev() {
        let (occupied, idx, size) = disk[p + offset];
        if !occupied {
            continue;
        }

        let slot_idx = disk
            .iter()
            .enumerate()
            .filter(|(slot_idx, (slot_occupied, _, slot_size))| {
                !slot_occupied && *slot_size >= size && *slot_idx < (p + offset)
            })
            .map(|(idx, _)| idx)
            .next();

        if let Some(slot_idx) = slot_idx {
            let slot = disk[slot_idx];
            let remainder = (false, slot.1, slot.2 - size);
            disk[slot_idx] = (true, idx, size);
            disk[p + offset].0 = false;
            disk.insert(slot_idx + 1, remainder);
            offset += 1;
        }
    }

    disk.iter()
        .scan(0, |global, (occupied, idx, size)| {
            let value = if *occupied { *idx as u128 } else { 0 };
            let checksum: u128 = (0..*size).map(|offset| (*global + offset) * value).sum();
            *global += size;
            Some(checksum)
        })
        .sum()
}
