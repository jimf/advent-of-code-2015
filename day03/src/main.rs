use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
}

fn mv(p: &Point, arrow: char) -> Point {
    match arrow {
        '<' => Point { x: p.x - 1, y: p.y     }, // Is there a way to get .. to work here?
        '>' => Point { x: p.x + 1, y: p.y     },
        '^' => Point { x: p.x,     y: p.y + 1 },
        'v' => Point { x: p.x,     y: p.y - 1 },
        _   => Point { x: p.x,     y: p.y     },
    }
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let mut santa_only_pos = Point::origin();
    let mut santa_pos = Point::origin();
    let mut robot_pos = Point::origin();
    let mut santa_only_houses = HashSet::new();
    let mut santa_and_robot_houses = HashSet::new();
    santa_only_houses.insert(santa_only_pos.clone());
    santa_and_robot_houses.insert(santa_pos.clone());

    for (i, arrow) in contents.chars().enumerate() {
        // Part a:
        santa_only_pos = mv(&santa_only_pos, arrow);
        santa_only_houses.insert(santa_only_pos.clone());

        // Part b:
        if i % 2 == 0 {
            santa_pos = mv(&santa_pos, arrow);
            santa_and_robot_houses.insert(santa_pos.clone());
        } else {
            robot_pos = mv(&robot_pos, arrow);
            santa_and_robot_houses.insert(robot_pos.clone());
        }
    }

    println!("A: {}", santa_only_houses.len());
    println!("B: {}", santa_and_robot_houses.len());
}
