use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let mut replacers = Vec::new();
    let mut parsed_empty_line = false;
    let mut input = String::new();
    let mut seen = HashSet::new();

    for line in contents.lines() {
        if !parsed_empty_line {
            if line.len() > 0 {
                let parts = line.split(" => ").collect::<Vec<_>>();
                replacers.push((parts[0].to_string(), parts[1].to_string()));
            } else {
                parsed_empty_line = true;
            }
        } else {
            input = line.to_string();
        }
    }

    for (needle, replace) in replacers {
        let matches = input.match_indices(&needle).collect::<Vec<_>>();
        for (i, _) in matches.iter() {
            let s1 = input.split_at(*i as usize);
            let s2 = input.split_at((i + needle.len()) as usize);
            let mut replacement = String::new();
            replacement.push_str(s1.0);
            replacement.push_str(&replace);
            replacement.push_str(s2.1);
            seen.insert(replacement);
        }
    }

    // Explanation: https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4h7ji/
    let num_symbols = input.matches(|c: char| c.is_ascii_uppercase()).collect::<Vec<_>>().len();
    let num_rn = input.matches("Rn").collect::<Vec<_>>().len();
    let num_ar = input.matches("Ar").collect::<Vec<_>>().len();
    let num_y = input.matches('Y').collect::<Vec<_>>().len();

    println!("A: {}", seen.len());
    println!("B: {}", num_symbols - num_rn - num_ar - 2 * num_y - 1);
}
