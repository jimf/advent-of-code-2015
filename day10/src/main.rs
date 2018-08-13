fn look_and_say(s: &String) -> String {
    let mut result = String::new();
    let mut prev = None;
    let mut seen = 0;

    for c in s.chars() {
        match prev {
            Some(prev_c) => {
                if c == prev_c {
                    seen = seen + 1;
                } else {
                    result.push_str(&seen.to_string());
                    result.push(prev_c);
                    prev = Some(c);
                    seen = 1;
                }
            },
            None => {
                prev = Some(c);
                seen = 1;
            }
        }
    }

    result.push_str(&seen.to_string());
    result.push(prev.unwrap());

    result
}

fn main() {
    let mut result_a = String::from("1113122113");
    let mut result_b = String::from("1113122113");

    for _ in 0..40 {
        result_a = look_and_say(&result_a);
    }

    for _ in 0..50 {
        result_b = look_and_say(&result_b);
    }

    println!("A: {}", result_a.len());
    println!("B: {}", result_b.len());
}
