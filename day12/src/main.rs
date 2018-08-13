use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Peekable;
use std::str::Chars;

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Number { value: i32 },
    Red,
    EndOfFileToken,
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
        Lexer { input: input.chars().peekable() }
    }

    fn read(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_ignored(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_numeric() || c == '-' || c == '"' || c == '{' || c == '}' || c == '[' || c == ']' {
                break;
            }
            self.read();
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
                _ => {
                    result.push(c);
                }
            }
        }

        result
    }

    fn read_number(&mut self) -> String {
        let mut result = String::new();
        while let Some(&c) = self.peek() {
            if c.is_numeric() || c == '-' {
                result.push(c);
                self.read();
            } else {
                break;
            }
        }
        result
    }

    fn next_token(&mut self) -> Token {
        self.skip_ignored();
        match self.peek() {
            None => Token::EndOfFileToken,
            Some('{') => {
                self.read();
                Token::LeftBrace {}
            },
            Some('}') => {
                self.read();
                Token::RightBrace {}
            },
            Some('[') => {
                self.read();
                Token::LeftBracket {}
            },
            Some(']') => {
                self.read();
                Token::RightBracket {}
            },
            Some('"') => {
                let val = self.read_string();
                if val == "red" {
                    Token::Red {}
                } else {
                    self.next_token()
                }
            },
            _   => Token::Number { value: self.read_number().parse().unwrap() },
        }
    }
}

enum Node {
    Num { value: i32 },
    Obj { sum: i32 },
    Arr { sum: i32 },
    RedObj,
}

fn combine(x: &Node, y: &Node) -> Option<Node> {
    match x {
        Node::Num { value: x_value } => {
            match y {
                Node::Num { value: y_value } => Some(Node::Num { value: x_value + y_value }),
                _ => None
            }
        },
        Node::Obj { sum: x_sum } => {
            match y {
                Node::Num { value: y_value } => Some(Node::Obj { sum: x_sum + y_value }),
                Node::Obj { sum: y_sum } => Some(Node::Obj { sum: x_sum + y_sum }),
                Node::Arr { sum: y_sum } => Some(Node::Obj { sum: x_sum + y_sum }),
                Node::RedObj => Some(Node::Obj { sum: *x_sum })
            }
        },
        Node::Arr { sum: x_sum } => {
            match y {
                Node::Num { value: y_value } => Some(Node::Arr { sum: x_sum + y_value }),
                Node::Obj { sum: y_sum } => Some(Node::Arr { sum: x_sum + y_sum }),
                Node::Arr { sum: y_sum } => Some(Node::Arr { sum: x_sum + y_sum }),
                Node::RedObj => Some(Node::Arr { sum: *x_sum })
            }
        },
        Node::RedObj => Some(Node::RedObj {})
    }
}

fn main() {
    let file = File::open("input.json").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    // Part A
    let mut sum_a = 0;
    let mut tmp = String::new();
    let mut prev_numeric = false;

    for line in contents.lines() {
        for c in line.chars() {
            if c == '-' || c.is_numeric() {
                tmp.push(c);
                prev_numeric = true;
            } else {
                if prev_numeric {
                    let val = tmp.parse::<i32>().unwrap();
                    sum_a += val;
                    tmp.clear();
                }
                prev_numeric = false;
            }
        }
    }

    println!("A: {}", sum_a);

    // Part B
    let mut lexer = Lexer::new(&contents);
    let mut stack = Vec::new();

    loop {
        let token = lexer.next_token();
        match token {
            Token::EndOfFileToken => {
                break;
            },
            Token::LeftBrace => {
                stack.push(Node::Obj { sum: 0 });
            },
            Token::LeftBracket => {
                stack.push(Node::Arr { sum: 0 });
            },
            Token::RightBrace | Token::RightBracket => {
                let y = stack.pop().unwrap();
                let x = stack.pop();
                match x {
                    Some(x_val) => {
                        stack.push(combine(&x_val, &y).unwrap());
                    },
                    None => {
                        stack.push(y);
                    }
                }
            },
            Token::Number { value } => {
                let node = Node::Num { value: value };
                let x = stack.pop().unwrap();
                stack.push(combine(&x, &node).unwrap());
            },
            Token::Red => {
                let top = stack.pop();
                match top {
                    Some(x) => {
                        match x {
                            Node::Obj { sum: _ } => {
                                stack.push(Node::RedObj {})
                            },
                            _ => {
                                stack.push(x);
                            }
                        }
                    }
                    None => {}
                }
            }
        }
    }

    match stack.pop().unwrap() {
        Node::Obj { sum } => {
            println!("B: {}", sum);
        },
        _ => {
            println!("B: unexpected top value");
        }
    }
}
