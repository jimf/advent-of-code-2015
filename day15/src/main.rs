use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn mult(&self, x: i32) -> Ingredient {
        Ingredient {
            name: self.name.clone(),
            capacity: self.capacity * x,
            durability: self.durability * x,
            flavor: self.flavor * x,
            texture: self.texture * x,
            calories: self.calories * x,
        }
    }
}

fn score(ingredients: &Vec<Ingredient>) -> i32 {
    let totals = ingredients.iter().fold((0, 0, 0, 0), |acc, ingredient| (
        acc.0 + ingredient.capacity,
        acc.1 + ingredient.durability,
        acc.2 + ingredient.flavor,
        acc.3 + ingredient.texture,
    ));
    cmp::max(0, totals.0) * cmp::max(0, totals.1) * cmp::max(0, totals.2) * cmp::max(0, totals.3)
}

fn total_calories(ingredients: &Vec<Ingredient>) -> i32 {
    ingredients.iter().map(|ingredient| ingredient.calories).sum()
}

fn parse_line(line: &str) -> Ingredient {
    let parts = line.split(' ').collect::<Vec<_>>();
    let mut nam = parts[0].to_string();
    let mut cap = parts[2].to_string();
    let mut dur = parts[4].to_string();
    let mut fla = parts[6].to_string();
    let mut tex = parts[8].to_string();
    let cal = parts[10].to_string();
    nam.pop();
    cap.pop();
    dur.pop();
    fla.pop();
    tex.pop();

    Ingredient {
        name: nam,
        capacity: cap.parse().unwrap(),
        durability: dur.parse().unwrap(),
        flavor: fla.parse().unwrap(),
        texture: tex.parse().unwrap(),
        calories: cal.parse().unwrap(),
    }
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let mut ingredients = Vec::new();
    for line in contents.lines() {
        ingredients.push(parse_line(&line));
    }

    let mut max_score_a = 0;
    let mut max_score_b = 0;

    for i in 1..98 {
        for j in 1..98 {
            for k in 1..98 {
                for l in 1..98 {
                    if i + j + k + l == 100 {
                        let mut recipe = Vec::new();
                        recipe.push(ingredients[0].mult(i));
                        recipe.push(ingredients[1].mult(j));
                        recipe.push(ingredients[2].mult(k));
                        recipe.push(ingredients[3].mult(l));
                        let s = score(&recipe);
                        if s > max_score_a {
                            max_score_a = s;
                        }
                        if s > max_score_b && total_calories(&recipe) == 500 {
                            max_score_b = s;
                        }
                    }
                }
            }
        }
    }

    println!("A: {}", max_score_a);
    println!("B: {}", max_score_b);
}
