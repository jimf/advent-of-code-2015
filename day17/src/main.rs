use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let mut containers = Vec::new();

    for line in contents.lines() {
        containers.push(line.parse::<u32>().unwrap());
    }

    let mut findings = HashMap::new();
    let mut result_a = 0;

    // Brute-force 2^n combinations.
    for i in 0..(1 << containers.len()) {
        let mut t = i;
        let mut parts = 0;
        let mut sum = 0;

        // Loop over containers:
        for container_size in containers.iter() {
            if t % 2 == 1 {
                parts += 1;
                sum += container_size;
            }
            t /= 2;
        }

        if sum == 150 {
            result_a += 1;
            let count = findings.entry(parts).or_insert(0);
            *count += 1;
        }
    }

    let mut min_containers = containers.len();
    let mut result_b = 0;
    for (combination_size, num_combinations) in findings {
        if combination_size < min_containers {
            min_containers = combination_size;
            result_b = num_combinations;
        }
    }

    println!("A: {}", result_a);
    println!("B: {}", result_b);
}
