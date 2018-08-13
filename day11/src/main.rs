use std::collections::HashSet;

fn format_radix(mut x: u64, radix: u64) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;
        result.push(std::char::from_digit(m as u32, radix as u32).unwrap());
        if x == 0 {
            break;
        }
    }

    result.into_iter().rev().collect()
}

fn parse36(s: &String) -> u64 {
    u64::from_str_radix(s, 36).unwrap()
}

fn inc_password(pw: &String) -> String {
    format_radix(parse36(&pw) + 1, 36)
}

fn is_valid_password(pw: &String) -> bool {
    let mut contains_digit = false;
    let mut contains_three_increasing = false;
    let mut contains_illegal = false;
    let mut pairs = HashSet::new();
    let mut prev1: Option<char> = None;
    let mut prev2: Option<char> = None;

    for c in pw.chars() {
        if c.is_numeric() {
            contains_digit = true;
        }

        let c36 = parse36(&c.to_string());
        match prev1 {
            Some(p1) => {
                match prev2 {
                    Some(p2) => {
                        if c >= 'c' && parse36(&p2.to_string()) == c36 - 2 && parse36(&p1.to_string()) == c36 - 1 {
                            contains_three_increasing = true;
                        }
                    },
                    None => {}
                }

                if p1 == c {
                    pairs.insert(c);
                }
            },
            None => {}
        }

        if c == 'i' || c == 'o' || c == 'l' {
            contains_illegal = true;
        }

        prev2 = prev1;
        prev1 = Some(c);
    }

    !contains_digit && contains_three_increasing && !contains_illegal && pairs.len() >= 2
}

fn next_valid_password(pw: &String) -> String {
    let mut result = inc_password(&pw);
    while !is_valid_password(&result) {
        result = inc_password(&result);
    }
    result
}

fn main() {
    let input = String::from("hepxcrrq");
    let result_a = next_valid_password(&input);
    let result_b = next_valid_password(&result_a);
    println!("A: {}", result_a);
    println!("B: {}", result_b);
}
