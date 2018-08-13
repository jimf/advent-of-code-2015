use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn next_floor(floor: i32, inst: char) -> i32 {
    match inst {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => floor,
    }
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    // Fold the input characters, accumulating a 3-element tuple of
    // (floor, position, position-of-first-basement).
    let result = contents.chars().fold((0, 0, 0), |acc, ch| (
        next_floor(acc.0, ch),
        acc.1 + 1,
        if acc.2 == 0 && next_floor(acc.0, ch) == -1 { acc.1 + 1 } else { acc.2 }
    ));

    println!("A: {}", result.0);
    println!("B: {}", result.2);
}
