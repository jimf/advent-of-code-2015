use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Peekable;
use std::str::Chars;

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

struct StringToken {
    value: String,
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
        Lexer { input: input.chars().peekable() }
    }

    fn read(&mut self) -> Option<char> {
        self.input.next()
    }

    fn read_escape_seq(&mut self) -> String {
        let c = self.read().unwrap();
        match c {
            '"' => String::from("\""),
            '\\' => String::from("\\"),
            _ => {
                let mut result = String::new();
                result.push(self.read().unwrap());
                result.push(self.read().unwrap());
                u8::from_str_radix("27", 16).map(|n| n as char).unwrap().to_string()
            }
        }
    }

    fn read_string(&mut self) -> String {
        let mut result = String::new();

        self.read(); // Skip initial quote.

        while let Some(c) = self.read() {
            match c {
                '"' => {
                    break;
                },
                '\\' => {
                    result.push_str(&self.read_escape_seq());
                },
                _ => {
                    result.push(c);
                }
            }
        }

        result
    }

    fn next_token(&mut self) -> StringToken {
        StringToken {
            value: self.read_string(),
        }
    }
}

fn encode(line: &str) -> String {
    let mut result = String::new();
    result.push('"');
    for c in line.chars() {
        match c {
            '"' | '\\' => {
                result.push('\\');
                result.push(c);
            },
            _ => {
                result.push(c);
            }
        }
    }
    result.push('"');
    result
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let result = contents.lines().fold((0, 0, 0), |acc, line| {
        let mut lexer = Lexer::new(&line);
        let parsed = lexer.next_token();
        let encoded = encode(&line);
        (acc.0 + line.len(), acc.1 + parsed.value.len(), acc.2 + encoded.len())
    });

    println!("A: {}", result.0 - result.1);
    println!("B: {}", result.2 - result.0);
}
