use std::io::BufRead;
use std::io;
use std::convert::From;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::cmp;
use std::iter::FromIterator;
use std::convert::TryInto;

type Algorithm = [bool; 512];
type Point = (i32, i32);

#[derive(Debug,Clone)]
struct Image {
    pixels: HashSet<Point>,
    background: bool,
    edge: Point
}

impl From<Vec<Vec<char>>> for Image {
    fn from(input: Vec<Vec<char>>) -> Image {
        let x = input.len();
        let y = input[0].len();
        let mut pixels: HashSet<Point> = HashSet::new();
        
        for x in 0..x {
            for y in 0..y {
                if input[x][y] == '#' {
                    pixels.insert((x.try_into().unwrap(), y.try_into().unwrap()));
                }
            }
        }

        Image{
            background: false,
            edge: (x.try_into().unwrap(), y.try_into().unwrap()),
            pixels: pixels,
        }
    }
}

fn get_next_point(point: Point, image: &Image, algorithm: &Algorithm) -> bool {
    let mut index = 0;

    for x in point.0-1..=point.0+1 {
        for y in point.1-1..=point.1+1 {
            index <<= 1;

            if image.pixels.contains(&(x, y)) {
                index += 1;
            } else if image.background && (x < 0 || x >= image.edge.0 || y < 0 || y >= image.edge.1) {
                index += 1
            }
        }
    }

    algorithm[index]
}

fn enhance(image: &Image, algorithm: &Algorithm) -> Image {
    let new_background = match image.background {
        false => algorithm[0],
        true => algorithm[511],
    };

    let mut new_pixels: HashSet<Point> = HashSet::new();

    for x in -2..image.edge.0 + 2 {
        for y in -2..image.edge.1 + 2 {
            let px = get_next_point((x, y), image, algorithm); 
            if px {
                new_pixels.insert((x + 2, y + 2));
            }
        }
    }

    Image{ pixels: new_pixels, background: new_background, edge: (image.edge.0 + 4, image.edge.1 + 4)}
}

fn print_image(image: &Image) {
    for x in 0..image.edge.0 {
        for y in 0..image.edge.1 {
            if image.pixels.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn part1(image: &Image, algorithm: &Algorithm) -> usize {
    let mut image = image.clone();
    for _ in 0..2 {
        image = enhance(&image, algorithm);
    }

    image.pixels.len()
}

fn part2(image: &Image, algorithm: &Algorithm) -> usize {
    let mut image = image.clone();
    for _ in 0..50 {
        image = enhance(&image, algorithm);
    }

    image.pixels.len()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut algorithm: Algorithm = [false; 512];
    
    let mut input = reader.lines();

    for (idx, c) in input.next().unwrap().unwrap().chars().enumerate() {
        if c == '#' {
            algorithm[idx] = true;
        }
    }

    input.next().unwrap();

    let image: Image = input.map(|line| line.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>().into();

	let answer1 = part1(&image, &algorithm);
	let answer2 = part2(&image, &algorithm);

	println!("Answer 1: {:?}", answer1);
	println!("Answer 2: {:?}", answer2);

    Ok(())
}
