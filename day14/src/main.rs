use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Reindeer {
    name: String,
    speed: u32,
    duration: u32,
    rest: u32,
}

fn parse_line(line: &str) -> Reindeer {
    let parts = line.split(' ').collect::<Vec<_>>();
    Reindeer {
        name: parts[0].to_string(),
        speed: parts[3].parse().unwrap(),
        duration: parts[6].parse().unwrap(),
        rest: parts[13].parse().unwrap(),
    }
}

enum State {
    Flying { distance: u32, remaining: u32, deer: Reindeer },
    Resting { distance: u32, remaining: u32, deer: Reindeer },
}

fn tick(state: &State) -> State {
    match state {
        State::Resting { distance, remaining, deer } => {
            if *remaining == 0 {
                State::Flying {
                    distance: *distance + deer.speed,
                    remaining: deer.duration - 1,
                    deer: Reindeer { name: deer.name.clone(), ..*deer },
                }
            } else {
                State::Resting {
                    distance: *distance,
                    remaining: remaining - 1,
                    deer: Reindeer { name: deer.name.clone(), ..*deer },
                }
            }
        },
        State::Flying { distance, remaining, deer } => {
            if *remaining == 0 {
                State::Resting {
                    distance: *distance,
                    remaining: deer.rest - 1,
                    deer: Reindeer { name: deer.name.clone(), ..*deer },
                }
            } else {
                State::Flying {
                    distance: *distance + deer.speed,
                    remaining: remaining - 1,
                    deer: Reindeer { name: deer.name.clone(), ..*deer },
                }
            }
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let mut states = Vec::new();

    for line in contents.lines() {
        let deer = parse_line(&line);
        states.push(State::Flying {
            distance: 0,
            remaining: deer.duration,
            deer: deer,
        });
    }

    let mut max_distance = 0;
    let mut max_points = 0;
    let mut points = HashMap::new();

    for _ in 0..2503 {
        for state in &mut states {
            let new_state = tick(&state);

            match new_state {
                State::Flying { distance, remaining: _, deer: _ } => {
                    if distance > max_distance {
                        max_distance = distance;
                    }
                },
                _ => {}
            }

            *state = new_state;
        }

        for updated_state in &states {
            match updated_state {
                State::Flying { deer, distance, remaining: _ } => {
                    if max_distance == *distance {
                        let pts = points.entry(deer.name.clone()).or_insert(0);
                        *pts += 1;
                        if *pts > max_points {
                            max_points = *pts
                        }
                    }
                },
                State::Resting { deer, distance, remaining: _ } => {
                    if max_distance == *distance {
                        let pts = points.entry(deer.name.clone()).or_insert(0);
                        *pts += 1;
                        if *pts > max_points {
                            max_points = *pts
                        }
                    }
                }
            }
        }
    }

    println!("A: {}", max_distance);
    println!("B: {}", max_points);
}
