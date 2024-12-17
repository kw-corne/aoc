use std::collections::HashMap;

use crate::util::get_lines;

#[derive(Debug)]
enum Instr {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instr {
    fn from_num(n: i32) -> Self {
        match n {
            0 => Instr::Adv,
            1 => Instr::Bxl,
            2 => Instr::Bst,
            3 => Instr::Jnz,
            4 => Instr::Bxc,
            5 => Instr::Out,
            6 => Instr::Bdv,
            7 => Instr::Cdv,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Program {
    ra: i32,
    rb: i32,
    rc: i32,
    program: Vec<i32>,
    ip: usize,
}

impl Program {
    fn new(lines: &[String]) -> Self {
        let ra = Program::get_reg_value(&lines[0]);
        let rb = Program::get_reg_value(&lines[1]);
        let rc = Program::get_reg_value(&lines[2]);
        let program = lines[4]
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        Self {
            ra,
            rb,
            rc,
            program,
            ip: 0,
        }
    }

    fn get_reg_value(s: &str) -> i32 {
        s.split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap()
    }

    fn run_instruction(&mut self) -> Option<i32> {
        let opcode = self.program[self.ip];
        let operand = self.program[self.ip + 1];
        let combo = match operand {
            0 | 1 | 2 | 3 => operand,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            _ => panic!(),
        };

        let instr = Instr::from_num(opcode);

        let mut output = None;
        let mut next_ip = self.ip + 2;

        match instr {
            Instr::Adv => self.ra /= 2_i32.pow(combo as u32),
            Instr::Bxl => self.rb ^= operand,
            Instr::Bst => self.rb = combo % 8,
            Instr::Jnz if self.ra != 0 => next_ip = operand as usize,
            Instr::Jnz => (),
            Instr::Bxc => self.rb ^= self.rc,
            Instr::Out => output = Some(combo % 8),
            Instr::Bdv => self.rb = self.ra / 2_i32.pow(combo as u32),
            Instr::Cdv => self.rc = self.ra / 2_i32.pow(combo as u32),
        }

        self.ip = next_ip;
        output
    }

    fn run(&mut self) -> Vec<i32> {
        let mut output = vec![];
        while self.ip < self.program.len() {
            if let Some(v) = self.run_instruction() {
                output.push(v);
            }
        }
        output
    }
}

fn p02(lines: Vec<String>) {}

fn p01(lines: Vec<String>) {
    let mut program = Program::new(&lines);
    let output = program
        .run()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    println!("{}", output.join(","));
}

pub fn d17() {
    p01(get_lines("src/d17/in.txt"));
    p02(get_lines("src/d17/dbg.txt"));
}
