use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Dimensions {
    l: i32,
    w: i32,
    h: i32,
}

impl Dimensions {
    fn surface_area(&self) -> i32 {
        (2 * self.l * self.w) + (2 * self.w * self.h) + (2 * self.h * self.l)
    }

    fn area_of_smallest_side(&self) -> i32 {
        let mut v = [self.l, self.w, self.h];
        v.sort();
        v[0] * v[1]
    }

    fn perimeter_of_smallest_side(&self) -> i32 {
        let mut v = [self.l, self.w, self.h];
        v.sort();
        2 * (v[0] + v[1])
    }

    fn volume(&self) -> i32 {
        self.l * self.w * self.h
    }
}

fn parse_line(line: &str) -> Dimensions {
    let parts: Vec<i32> = line.split('x').map(|part| part.parse().unwrap()).collect();
    Dimensions {
        l: parts[0],
        w: parts[1],
        h: parts[2],
    }
}

fn total_wrapping_paper(acc: i32, dims: &Dimensions) -> i32 {
    acc + dims.surface_area() + dims.area_of_smallest_side()
}

fn total_ribbon(acc: i32, dims: &Dimensions) -> i32 {
    acc + dims.perimeter_of_smallest_side() + dims.volume()
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    // Fold over the input lines, parsing the dimensions and accumulating a 2-element tuple of
    // (total-wrapping-paper, total-ribbon).
    let result = contents
        .lines()
        .fold((0, 0), |acc, line| {
            let dims = parse_line(line);
            (total_wrapping_paper(acc.0, &dims), total_ribbon(acc.1, &dims))
        });

    println!("A: {}", result.0);
    println!("B: {}", result.1);
}
