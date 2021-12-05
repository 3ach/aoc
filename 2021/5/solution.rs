use std::io::BufRead;
use std::io;
use std::cmp;
use std::iter;

const SIZE: usize = 1000;

#[derive(Debug,Clone,Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug,Clone,Copy)]
struct Line {
    start: Point,
    end: Point
}

fn expand_line(line: &Line, diag: bool) -> Vec<Point> {

    let min_x = cmp::min(line.start.x, line.end.x);
    let max_x = cmp::max(line.start.x, line.end.x);

    let min_y = cmp::min(line.start.y, line.end.y);
    let max_y = cmp::max(line.start.y, line.end.y);

    let x_iter: Box<dyn Iterator<Item = usize>>;
    let y_iter: Box<dyn Iterator<Item = usize>>;

    if min_x == max_x {
        x_iter = Box::new(iter::repeat(min_x));
        y_iter = Box::new(min_y..=max_y);
    } else if min_y == max_y {
        x_iter = Box::new(min_x..=max_x);
        y_iter = Box::new(iter::repeat(min_y));
    } else if diag {
        if min_x != line.start.x {
            x_iter = Box::new((min_x..=max_x).rev());
        } else {
            x_iter = Box::new(min_x..=max_x);
        }
        
        if min_y != line.start.y {
            y_iter = Box::new((min_y..=max_y).rev());
        } else {
            y_iter = Box::new(min_y..=max_y);
        }
    } else {
        return vec![];
    }

    return x_iter.zip(y_iter).map(|(x, y)| Point{x: x, y: y}).collect();

}

fn print_seafloor(seafloor: &Vec<[u32; SIZE]>) {
    for row in 0..SIZE {
        for col in 0..SIZE {
            print!("{}", seafloor[row][col]);
        }
        println!("")
    }
}

fn part1(lines: &Vec<Line>) -> i32 {
    let mut seafloor = vec![[0; SIZE]; SIZE];

    for line in lines {
        let expanded = expand_line(&line, false);

        for point in expanded {
            seafloor[point.x][point.y] += 1
        }
    }


    let mut intersections = 0;
    for row in 0..SIZE {
        for col in 0..SIZE {
            if seafloor[row][col] > 1 {
                intersections += 1;
            }
        }
    }

    return intersections;
}

fn part2(lines: &Vec<Line>) -> i32 {
    let mut seafloor = vec![[0; SIZE]; SIZE];

    for line in lines {
        let expanded = expand_line(&line, true);

        for point in expanded {
            seafloor[point.x][point.y] += 1
        }
    }


    let mut intersections = 0;
    for row in 0..SIZE {
        for col in 0..SIZE {
            if seafloor[row][col] > 1 {
                intersections += 1;
            }
        }
    }

    return intersections;
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines: Vec<Line> = vec![];

    for line in reader.lines() {
        let pts: Vec<Point> = line.unwrap()
            .split(" -> ")
            .map(|pair| {
                let mut tok = pair.split(",");
                Point{x: tok.next().unwrap().parse().unwrap(), y:tok.next().unwrap().parse().unwrap()}
            }).collect();

        lines.push(Line{start: pts[0], end: pts[1]})
    }

	let answer1 = part1(&lines);
	let answer2 = part2(&lines);

	println!("Answer 1: {}", answer1);
	println!("Answer 2: {}", answer2);

    Ok(())
}
