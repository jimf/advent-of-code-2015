extern crate permutohedron;

use permutohedron::heap_recursive;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn parse_line(line: &str) -> (String, String, i32) {
    let parts = line.split(' ').collect::<Vec<_>>();
    let name = String::from(parts[0]);
    let mut happiness = parts[3].parse::<i32>().unwrap();
    if parts[2] == "lose" {
        happiness = -happiness;
    }
    let mut next_to = String::from(parts[10]);
    next_to.pop();
    (name, next_to, happiness)
}

fn get_total_happiness(happiness: &HashMap<(String, String), i32>, arrangement: &Vec<String>) -> i32 {
    let mut rotated = Vec::new();
    for arr in arrangement.iter().skip(1) {
        rotated.push(arr);
    }
    rotated.push(&arrangement[0]);
    // let (mut head, mut tail) = arrangement.split_at(1);
    // tail.append(head);
    arrangement
        .iter()
        .zip(rotated)
        .map(|seating| {
            let happy1 = happiness.get(&(seating.0.to_string(), seating.1.to_string())).unwrap();
            let happy2 = happiness.get(&(seating.1.to_string(), seating.0.to_string())).unwrap();
            happy1 + happy2
        })
        .sum()
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    // let file = File::open("input_b.txt").expect("file not found"); // Swap input files for parts a/b
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let mut happiness = HashMap::new();
    let mut names = HashSet::new();

    for line in contents.lines() {
        let def = parse_line(&line);
        happiness.insert((def.0.clone(), def.1.clone()), def.2);
        names.insert(def.0.clone());
        names.insert(def.1.clone());
    }

    // TODO: Gross. Better way?
    let mut names_vec = Vec::new();
    for name in names {
        names_vec.push(name);
    }

    let mut max_total_happiness = 0;

    heap_recursive(&mut names_vec, |arrangement| {
        // // TODO: Gross. Can this be done with path more directly?
        let mut this_arr = Vec::new();
        for arr in arrangement.to_vec() {
            this_arr.push(String::from(arr));
        }

        let happy = get_total_happiness(&happiness, &this_arr);

        if happy > max_total_happiness {
            max_total_happiness = happy;
        }
    });

    println!("{}", max_total_happiness);
}
