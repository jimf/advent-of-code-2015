use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Sue {
    id: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl Sue {
    fn is_gifter_a(&self) -> bool {
        if self.children.is_some() && self.children.unwrap() != 3       { return false }
        if self.cats.is_some() && self.cats.unwrap() != 7               { return false }
        if self.samoyeds.is_some() && self.samoyeds.unwrap() != 2       { return false }
        if self.pomeranians.is_some() && self.pomeranians.unwrap() != 3 { return false }
        if self.akitas.is_some() && self.akitas.unwrap() != 0           { return false }
        if self.vizslas.is_some() && self.vizslas.unwrap() != 0         { return false }
        if self.goldfish.is_some() && self.goldfish.unwrap() != 5       { return false }
        if self.trees.is_some() && self.trees.unwrap() != 3             { return false }
        if self.cars.is_some() && self.cars.unwrap() != 2               { return false }
        if self.perfumes.is_some() && self.perfumes.unwrap() != 1       { return false }
        true
    }

    fn is_gifter_b(&self) -> bool {
        if self.children.is_some() && self.children.unwrap() != 3       { return false }
        if self.cats.is_some() && self.cats.unwrap() <= 7               { return false }
        if self.samoyeds.is_some() && self.samoyeds.unwrap() != 2       { return false }
        if self.pomeranians.is_some() && self.pomeranians.unwrap() >= 3 { return false }
        if self.akitas.is_some() && self.akitas.unwrap() != 0           { return false }
        if self.vizslas.is_some() && self.vizslas.unwrap() != 0         { return false }
        if self.goldfish.is_some() && self.goldfish.unwrap() >= 5       { return false }
        if self.trees.is_some() && self.trees.unwrap() <= 3             { return false }
        if self.cars.is_some() && self.cars.unwrap() != 2               { return false }
        if self.perfumes.is_some() && self.perfumes.unwrap() != 1       { return false }
        true
    }
}

fn parse_line(id: usize, line: &str) -> Sue {
    let defs = line.to_string().split_off(6 + id.to_string().len());
    let parts = defs.split(", ").collect::<Vec<_>>();
    let mut sue = Sue {
        id: id as u32,
        children: None,
        cats: None,
        samoyeds: None,
        pomeranians: None,
        akitas: None,
        vizslas: None,
        goldfish: None,
        trees: None,
        cars: None,
        perfumes: None,
    };
    for part in parts.iter() {
        let pair = part.split(": ").collect::<Vec<_>>();
        match pair[0] {
            "children"    => { sue.children    = Some(pair[1].parse().unwrap()); },
            "cats"        => { sue.cats        = Some(pair[1].parse().unwrap()); },
            "samoyeds"    => { sue.samoyeds    = Some(pair[1].parse().unwrap()); },
            "pomeranians" => { sue.pomeranians = Some(pair[1].parse().unwrap()); },
            "akitas"      => { sue.akitas      = Some(pair[1].parse().unwrap()); },
            "vizslas"     => { sue.vizslas     = Some(pair[1].parse().unwrap()); },
            "goldfish"    => { sue.goldfish    = Some(pair[1].parse().unwrap()); },
            "trees"       => { sue.trees       = Some(pair[1].parse().unwrap()); },
            "cars"        => { sue.cars        = Some(pair[1].parse().unwrap()); },
            "perfumes"    => { sue.perfumes    = Some(pair[1].parse().unwrap()); },
            _ => {}
        }
    }
    sue
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    for (i, line) in contents.lines().enumerate() {
        let sue = parse_line(i + 1, &line);
        if sue.is_gifter_a() {
            println!("A: {}", sue.id);
        }
        if sue.is_gifter_b() {
            println!("B: {}", sue.id);
        }
    }
}
