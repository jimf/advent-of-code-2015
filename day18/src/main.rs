use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn from_index(i: usize) -> Coord {
        Coord {
            row: (i / 100),
            col:i % 100,
        }
    }

    fn to_index(&self) -> usize {
        (self.row * 100) + self.col
    }

    fn neighbors(&self) -> Vec<Coord> {
        let mut result = Vec::new();
        if self.row > 0 {
            if self.col > 0 {
                result.push(Coord { row: self.row - 1, col: self.col - 1 });
            }
            result.push(Coord { row: self.row - 1, col: self.col });
            if self.col < 99 {
                result.push(Coord { row: self.row - 1, col: self.col + 1 });
            }
        }
        if self.col > 0 {
            result.push(Coord { row: self.row, col: self.col - 1 });
        }
        if self.col < 99 {
            result.push(Coord { row: self.row, col: self.col + 1 });
        }
        if self.row < 99 {
            if self.col > 0 {
                result.push(Coord { row: self.row + 1, col: self.col - 1 });
            }
            result.push(Coord { row: self.row + 1, col: self.col });
            if self.col < 99 {
                result.push(Coord { row: self.row + 1, col: self.col + 1 });
            }
        }
        result
    }
}

fn next_state(state: &[u32; 10000], is_p2: bool) -> [u32; 10000] {
    let mut result: [u32; 10000] = [0; 10000];
    for i in 0..10000 {
        let coord = Coord::from_index(i);
        let mut neighbors_on = 0;
        for neighbor in coord.neighbors().iter() {
            if state[neighbor.to_index()] == 1 {
                neighbors_on += 1
            }
        }
        if state[i] == 1 {
            if neighbors_on == 2 || neighbors_on == 3 {
                result[i] = 1;
            } else {
                result[i] = 0;
            }
        } else {
            if neighbors_on == 3 {
                result[i] = 1;
            }
        }
    }
    if is_p2 {
        result[(Coord { row: 0, col: 0 }).to_index()] = 1;
        result[(Coord { row: 0, col: 99 }).to_index()] = 1;
        result[(Coord { row: 99, col: 0 }).to_index()] = 1;
        result[(Coord { row: 99, col: 99 }).to_index()] = 1;
    }
    result
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let mut grid_a: [u32; 10000] = [0; 10000];
    let mut grid_b: [u32; 10000] = [0; 10000];
    let mut i = 0;

    grid_b[(Coord { row: 0, col: 0 }).to_index()] = 1;
    grid_b[(Coord { row: 0, col: 99 }).to_index()] = 1;
    grid_b[(Coord { row: 99, col: 0 }).to_index()] = 1;
    grid_b[(Coord { row: 99, col: 99 }).to_index()] = 1;

    for line in contents.lines() {
        for ch in line.chars() {
            if ch == '#' {
                grid_a[i] = 1;
                grid_b[i] = 1;
            }
            i += 1;
        }
    }

    for _ in 0..100 {
        grid_a = next_state(&grid_a, false);
        grid_b = next_state(&grid_b, true);
    }

    println!("A: {}", grid_a.iter().sum::<u32>());
    println!("B: {}", grid_b.iter().sum::<u32>());
}
