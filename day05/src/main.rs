use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn is_nice1(s: &str) -> bool {
    let mut vowels = 0;
    let mut contains_pair = false;
    let mut contains_naughty = false;
    let mut prev = ' ';

    for ch in s.chars() {
        if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
            vowels = vowels + 1;
        }
        if ch == prev {
            contains_pair = true;
        }
        let pair = (prev, ch);
        if pair == ('a', 'b') || pair == ('c', 'd') || pair == ('p', 'q') || pair == ('x', 'y') {
            contains_naughty = true;
            break;
        }
        prev = ch;
    }

    vowels >= 3 && contains_pair && !contains_naughty
}

fn is_nice2(s: &str) -> bool {
    let mut contains_double_pair = false;
    let mut contains_separated_repeat = false;
    let mut prev = ' ';
    let mut prev_prev = ' ';
    let mut pairs = HashMap::new();

    for (i, ch) in s.chars().enumerate() {
        let pair = (prev, ch);
        match pairs.get(&pair) {
            Some(&idx) => {
                if idx <= i - 2 {
                    contains_double_pair = true;
                }
            },
            None => { pairs.insert(pair, i); }
        }
        if prev_prev == ch {
            contains_separated_repeat = true;
        }
        prev_prev = prev;
        prev = ch;
    }

    contains_double_pair && contains_separated_repeat
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let result = contents.lines().fold((0, 0), |acc, line| (
        if is_nice1(&line) { acc.0 + 1 } else { acc.0 },
        if is_nice2(&line) { acc.1 + 1 } else { acc.1 }
    ));

    println!("A: {}", result.0);
    println!("B: {}", result.1);
}
