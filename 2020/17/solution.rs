use std::io::BufRead;
use std::io;
use std::convert::TryInto;
use std::collections::HashSet;

fn count_neighbors(map: &HashSet<(isize, isize, isize)>, x: isize, y: isize, z: isize) -> u32 {
    let mut count = 0;
    for row in x-1..=x+1 {
        for col in y-1..=y+1 {
            for dep in z-1..=z+1 {
                if x == row && y == col && z == dep {
                    continue
                }
                if map.contains(&(row, col, dep)) {
                    count += 1;
                }
            }
        }
    }
    
    count
}

fn count_neighbors4d(map: &HashSet<(isize, isize, isize, isize)>, x: isize, y: isize, z: isize, w: isize) -> u32 {
    let mut count = 0;
    for row in x-1..=x+1 {
        for col in y-1..=y+1 {
            for dep in z-1..=z+1 {
                for hyp in w-1..=w+1 {
                    if x == row && y == col && z == dep && w == hyp {
                        continue
                    }
                    if map.contains(&(row, col, dep, hyp)) {
                        count += 1;
                    }
                }
            }
        }
    }
    
    count
}

fn part1(start: &HashSet<(isize, isize, isize)>, width: isize, height: isize) -> usize {
    let mut rmin: isize = 0;
    let mut rmax: isize = height - 1;
    let mut cmin: isize = 0;
    let mut cmax: isize = width - 1;
    let mut dmin: isize = 0;
    let mut dmax: isize = 0;
    let mut current = start.clone();

    for _  in 0..6 {
        let mut next = HashSet::new();
        // println!("{:?} >= r >= {:?}", rmin-1, rmax+1);
        // println!("{:?} >= c >= {:?}", cmin-1, cmax+1);
        // println!("{:?} >= d >= {:?}", dmin-1, dmax+1);

        let rowrange = rmin-1..=rmax+1;
        let colrange = cmin-1..=cmax+1;
        let deprange = dmin-1..=dmax+1;
        for dep in deprange.clone() {
            // println!("z = {:?}", dep);
            for row in rowrange.clone() {
                for col in colrange.clone() {
                    if current.contains(&(row, col, dep)) {
                        // print!("#");
                    } else {
                        // print!(".");
                    }
                }
                // println!();
            }
        }

        for row in rowrange.clone() {
            for col in colrange.clone() {
                for dep in deprange.clone() {
                    let neighbors = count_neighbors(&current, row, col, dep); 
                    if (current.contains(&(row, col, dep)) && (neighbors == 2 || neighbors == 3)) 
                        || (!current.contains(&(row, col, dep)) && neighbors == 3) {
                        if dep == 0 {
                            // println!("({:}, {:}, {:}) has {} neighbors", row, col, dep, neighbors);
                        }
                        if rmin > row { rmin = row; }
                        if rmax < row { rmax = row; }
                        if cmin > col { cmin = col; }
                        if cmax < col { cmax = col; }
                        if dmin > dep { dmin = dep; }
                        if dmax < dep { dmax = dep; }

                        next.insert((row, col, dep));
                    }
                }
            }
        }

        // println!("{} -> {}", current.len(), next.len());

        current = next;
    }
    
    current.len()
}

fn part2(start: &HashSet<(isize, isize, isize, isize)>, width: isize, height: isize) -> usize {
    let mut rmin: isize = 0;
    let mut rmax: isize = height - 1;
    let mut cmin: isize = 0;
    let mut cmax: isize = width - 1;
    let mut dmin: isize = 0;
    let mut dmax: isize = 0;
    let mut hmin: isize = 0;
    let mut hmax: isize = 0;
    let mut current = start.clone();

    for _  in 0..6 {
        let mut next = HashSet::new();

        let rowrange = rmin-1..=rmax+1;
        let colrange = cmin-1..=cmax+1;
        let deprange = dmin-1..=dmax+1;
        let hyprange = hmin-1..=hmax+1;

        for row in rowrange.clone() {
            for col in colrange.clone() {
                for dep in deprange.clone() {
                    for hyp in hyprange.clone() {
                        let neighbors = count_neighbors4d(&current, row, col, dep, hyp); 
                        if (current.contains(&(row, col, dep, hyp)) && (neighbors == 2 || neighbors == 3)) 
                            || (!current.contains(&(row, col, dep, hyp)) && neighbors == 3) {
                            if rmin > row { rmin = row; }
                            if rmax < row { rmax = row; }
                            if cmin > col { cmin = col; }
                            if cmax < col { cmax = col; }
                            if dmin > dep { dmin = dep; }
                            if dmax < dep { dmax = dep; }
                            if hmin > hyp { hmin = hyp; }
                            if hmax < hyp { hmax = hyp; }

                            next.insert((row, col, dep, hyp));
                        }
                    }
                }
            }
        }


        current = next;
    }
    
    current.len()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let initial: HashSet<(isize, isize, isize)> = reader.lines()
        .enumerate()
        .map(|(row, line)| line.expect("Can't read stdin")
             .chars()
             .enumerate()
             .filter(|(_, c)| *c == '#')
             .map(|(col, _)| (row.try_into().unwrap(), col.try_into().unwrap(), 0))
             .collect::<Vec<(isize, isize, isize)>>()
        ).flatten()
        .collect();

    let initial4d = initial.iter().map(|(x, y, z)| (*x, *y, *z, 0)).collect();

	let answer1 = part1(&initial, 8, 8);
	let answer2 = part2(&initial4d, 8, 8);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
