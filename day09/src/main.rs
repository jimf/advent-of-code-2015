extern crate permutohedron;

use permutohedron::heap_recursive;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn get_total_distance(distances: &HashMap<String, i32>, path: &Vec<String>) -> i32 {
    path.iter()
        .take(path.len() - 1)
        .zip(path.iter().skip(1))
        .map(|locs| {
            // TODO: Slightly gross. Can this be done w/o key?
            let mut key = locs.0.clone();
            key.push_str(&locs.1);
            distances.get::<String>(&key).unwrap()
        })
        .sum()
}

// TODO: Works, but isn't type-checking at the moment. Swapped out for crate.
// fn permutations(list: &Vec<String>) -> Vec<Vec<String>> {
//     match list.len() {
//         0 => vec![],
//         1 => vec![vec![list[0]]],
//         2 => vec![
//             vec![list[0], list[1]],
//             vec![list[1], list[0]],
//         ],
//         _ => {
//             let mut perms = Vec::new();
//             for i in 0..list.len() {
//                 let mut list_without_ith = Vec::new();
//                 for (j, item) in list.iter().enumerate() {
//                     if i != j {
//                         list_without_ith.push(item.clone());
//                     }
//                 }
//                 for next_perm in permutations(&list_without_ith) {
//                     let mut perm = Vec::new();
//                     perm.push(list[i]);
//                     for x in next_perm {
//                         perm.push(x);
//                     }
//                     perms.push(perm);
//                 }
//             }
//             perms
//         }
//     }
// }

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let mut locations = HashSet::new();
    let mut distances = HashMap::new();

    for line in contents.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let from = parts[0];
        let to = parts[2];
        let distance = parts[4].parse().unwrap();
        locations.insert(from);
        locations.insert(to);
        distances.insert([from, to].join(""), distance);
        distances.insert([to, from].join(""), distance);
    }

    // TODO: Gross. Better way?
    let mut locations_vec = Vec::new();
    for location in locations {
        locations_vec.push(location);
    }

    let mut min_distance = i32::max_value();
    let mut max_distance = 0;

    heap_recursive(&mut locations_vec, |path| {
        // TODO: Gross. Can this be done with path more directly?
        let mut this_path = Vec::new();
        for p in path.to_vec() {
            this_path.push(String::from(p));
        }

        let distance = get_total_distance(&distances, &this_path);

        if distance < min_distance {
            min_distance = distance;
        }

        if distance > max_distance {
            max_distance = distance;
        }
    });

    println!("A: {}", min_distance);
    println!("B: {}", max_distance);
}
