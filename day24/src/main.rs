extern crate rand;

use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(PartialEq, Eq, PartialOrd)]
struct Group {
    size: usize,
    quantum_entanglement: u64,
}

// 3 groups, A B C
// sum A == sum B == sum C
// A has fewest # of packages (use product of items in A as tie-breaker)

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let mut rng = thread_rng();
    let mut packages = Vec::new();

    for line in contents.lines() {
        packages.push(line.parse::<u64>().unwrap());
    }

    let group_size_a = packages.iter().sum::<u64>() / 3;
    let group_size_b = packages.iter().sum::<u64>() / 4;
    let mut best_a = Group { size: usize::max_value(), quantum_entanglement: u64::max_value() };
    let mut best_b = Group { size: usize::max_value(), quantum_entanglement: u64::max_value() };

    // Just randomly shuffle the input MANY times, checking with each iteration
    // if the vector starts with a group that sums to the desired total. If it
    // does, assume the remainder can be split evenly, and track the min total
    // and product to report the lowest. Not guaranteed to work every time,
    // but should work over time.
    for _ in 0..1e6 as usize {
        rng.shuffle(&mut packages);

        let mut i = 0;
        let mut sum: u64 = 0;
        let mut prod: u64 = 1;

        loop {
            sum += packages[i];
            prod *= packages[i];
            i += 1;

            if sum == group_size_a {
                let group = Group { size: i, quantum_entanglement: prod };
                if group < best_a {
                    best_a = group;
                }
            } else if sum == group_size_b {
                let group = Group { size: i, quantum_entanglement: prod };
                if group < best_b {
                    best_b = group;
                }
            }

            if sum >= group_size_a || i > 10 {
                break;
            }
        }
    }

    println!("A: {}", best_a.quantum_entanglement);
    println!("B: {}", best_b.quantum_entanglement);
}
