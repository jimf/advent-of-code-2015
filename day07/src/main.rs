use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn is_number(s: &str) -> bool {
    let first_char = s.chars().next().unwrap();
    first_char >= '0' && first_char <= '9'
}

fn lookup(values: &HashMap<&str, i32>, key: &str) -> Option<i32> {
    if is_number(&key) {
        Some(key.parse().unwrap())
    } else {
        match values.get(&key) {
            Some(val) => Some(*val),
            None => None
        }
    }
}

fn eval_connection(values: &HashMap<&str, i32>, conn: &str) -> Option<i32> {
    let tokens: Vec<&str> = conn.split(' ').collect();

    match tokens.len() {
        1 => {
            // Wire
            lookup(&values, &tokens[0])
        },
        2 => {
            // Unary op (NOT)
            lookup(&values, &tokens[1]).map(|val| val ^ 65535)
        }
        _ => {
            // Binary op
            match lookup(&values, &tokens[0]) {
                Some(op1) => {
                    match lookup(&values, &tokens[2]) {
                        Some(op2) => {
                            match tokens[1] {
                                "OR"     => Some(op1 | op2),
                                "AND"    => Some(op1 & op2),
                                "LSHIFT" => Some(op1 << op2),
                                _        => Some(op1 >> op2),
                            }
                        },
                        None => None
                    }
                },
                None => None
            }
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let mut values: HashMap<&str, i32> = HashMap::new();
    let mut remaining: VecDeque<&str> = VecDeque::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let tokens: Vec<&str> = parts[0].split(' ').collect();
        if is_number(line) && tokens.len() == 1 {
            values.insert(&parts[1], tokens[0].parse().unwrap());
        } else {
            remaining.push_back(&line);
        }
    }

    // Uncomment for part B.
    // values.insert("b", 46065);

    while !values.contains_key("a") {
        let next = remaining.pop_front().unwrap();
        let parts: Vec<&str> = next.split(" -> ").collect();
        match eval_connection(&values, &parts[0]) {
            Some(value) => {
                values.insert(&parts[1], value);
            },
            None => {
                remaining.push_back(&next);
            }
        }
    }

    println!("wire a: {}", values.get("a").unwrap());
}
