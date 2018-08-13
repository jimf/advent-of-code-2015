extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;

struct Point {
    x: usize,
    y: usize,
}

enum Action {
    TurnOn  { from: Point, to: Point },
    TurnOff { from: Point, to: Point },
    Toggle  { from: Point, to: Point },
}

fn parse_line(s: &str) -> Action {
    let pattern = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    let matches = pattern.captures(s).unwrap();
    let from = Point { x: matches[2].parse().unwrap(), y: matches[3].parse().unwrap() };
    let to = Point { x: matches[4].parse().unwrap(), y: matches[5].parse().unwrap() };
    match &matches[1] {
        "turn on"  => Action::TurnOn  { from, to },
        "turn off" => Action::TurnOff { from, to },
        _          => Action::Toggle  { from, to },
    }
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let mut grid1: [[i32; 1000]; 1000] = [[0; 1000]; 1000];
    let mut grid2: [[i32; 1000]; 1000] = [[0; 1000]; 1000];

    for action in contents.lines().map(parse_line) {
        match action {
            Action::TurnOn { from, to } => {
                for i in from.x .. to.x + 1 {
                    for j in from.y .. to.y + 1 {
                        grid1[i][j] = 1;
                        grid2[i][j] = grid2[i][j] + 1;
                    }
                }
            },
            Action::TurnOff { from, to } => {
                for i in from.x .. to.x + 1 {
                    for j in from.y .. to.y + 1 {
                        grid1[i][j] = 0;
                        grid2[i][j] = if grid2[i][j] > 0 { grid2[i][j] - 1 } else { 0 }
                    }
                }
            },
            Action::Toggle { from, to } => {
                for i in from.x .. to.x + 1 {
                    for j in from.y .. to.y + 1 {
                        grid1[i][j] = if grid1[i][j] == 0 { 1 } else { 0 };
                        grid2[i][j] = grid2[i][j] + 2;
                    }
                }
            },
        }
    }

    let mut sum_a = 0;
    let mut sum_b = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            sum_a = sum_a + grid1[i][j];
            sum_b = sum_b + grid2[i][j];
        }
    }

    println!("A: {}", sum_a);
    println!("B: {}", sum_b);
}
