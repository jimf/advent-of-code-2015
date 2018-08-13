use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

enum Register {
    A,
    B,
}

enum Instruction {
    Half { reg: Register },
    Triple { reg: Register },
    Inc { reg: Register },
    Jump { offset: i32 },
    JumpIfEven { reg: Register, offset: i32 },
    JumpIfOne { reg: Register, offset: i32 },
}

struct Vm {
    pc: usize,
    a: u32,
    b: u32,
}

impl Vm {
    fn process(&mut self, instructions: &Vec<Instruction>) {
        while self.pc < instructions.len() {
            match instructions[self.pc] {
                Instruction::Half { ref reg } => {
                    match reg {
                        Register::A => { self.a >>= 1; },
                        Register::B => { self.b >>= 1; }
                    }
                    self.pc += 1;
                },
                Instruction::Triple { ref reg } => {
                    match reg {
                        Register::A => { self.a *= 3; },
                        Register::B => { self.b *= 3; }
                    }
                    self.pc += 1;
                },
                Instruction::Inc { ref reg } => {
                    match reg {
                        Register::A => { self.a += 1; },
                        Register::B => { self.b += 1; }
                    }
                    self.pc += 1;
                },
                Instruction::Jump { offset } => {
                    if offset < 0 {
                        if offset.abs() as usize > self.pc {
                            self.pc = instructions.len();
                        } else {
                            self.pc -= offset.abs() as usize;
                        }
                    } else {
                        self.pc += offset as usize;
                    }
                },
                Instruction::JumpIfEven { ref reg, offset } => {
                    match reg {
                        Register::A => {
                            if self.a % 2 == 0 {
                                self.pc += offset as usize;
                            } else {
                                self.pc += 1;
                            }
                        },
                        Register::B => {
                            if self.b % 2 == 0 {
                                self.pc += offset as usize;
                            } else {
                                self.pc += 1;
                            }
                        }
                    }
                },
                Instruction::JumpIfOne { ref reg, offset } => {
                    match reg {
                        Register::A => {
                            if self.a == 1 {
                                self.pc += offset as usize;
                            } else {
                                self.pc += 1;
                            }
                        },
                        Register::B => {
                            if self.b == 1 {
                                self.pc += offset as usize;
                            } else {
                                self.pc += 1;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn parse_line(line: &str) -> Instruction {
    let my_line = str::replace(&line, ",", "");
    let words = my_line.split(' ').collect::<Vec<_>>();
    let reg = if words[1] == "a" { Register::A } else { Register::B };
    let offset = if words.len() == 3 { words[2].parse().unwrap() } else { 0 };
    match words[0] {
        "hlf" => Instruction::Half { reg },
        "tpl" => Instruction::Triple { reg },
        "inc" => Instruction::Inc { reg },
        "jmp" => Instruction::Jump { offset: words[1].parse().unwrap() },
        "jie" => Instruction::JumpIfEven { reg, offset },
        _     => Instruction::JumpIfOne { reg, offset }
    }
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("could not read input file");

    let mut instructions = Vec::new();
    for line in contents.lines() {
        instructions.push(parse_line(&line));
    }

    let mut vm_a = Vm { pc: 0, a: 0, b: 0 };
    let mut vm_b = Vm { pc: 0, a: 1, b: 0 };
    vm_a.process(&instructions);
    vm_b.process(&instructions);

    println!("A: {}", vm_a.b);
    println!("B: {}", vm_b.b);
}
