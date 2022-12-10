use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

pub(crate) const PART1: fn(String) = |i| both_parts(i, false);
pub(crate) const PART2: fn(String) = |i| both_parts(i, true);

#[derive(Debug)]
enum ParseInstructionError {
    ParseInstructionFailed,
    ParseOperandFailed(ParseIntError),
}

use ParseInstructionError::*;

impl Display for ParseInstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseInstructionFailed => write!(f, "Invalid instruction"),
            ParseOperandFailed(e) => Display::fmt(&e, f),
        }
    }
}

impl Error for ParseInstructionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseInstructionFailed => None,
            ParseOperandFailed(e) => Some(e),
        }
    }
}

enum Instruction {
    Addx(isize),
    Noop,
}

use Instruction::*;

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        match split[0] {
            "noop" => Ok(Noop),
            "addx" => Ok(Addx(split[1].parse::<isize>().map_err(ParseOperandFailed)?)),
            _ => Err(ParseInstructionFailed),
        }
    }
}

struct CpuWithCrt {
    cycles: usize,
    pc: usize,
    x: isize,
    mem: Vec<Instruction>,
    total_strength: isize,
    crt: [[bool; 40]; 6],
}

impl CpuWithCrt {
    fn inc_strength(&mut self) {
        if (self.cycles as isize - 20) % 40 == 0 {
            self.total_strength += self.x * self.cycles as isize;
        }
    }

    fn draw(&mut self) {
        let row = self.cycles / 40;
        let col = self.cycles % 40;

        if self.x.abs_diff(col as isize) <= 1 {
            self.crt[row][col] = true;
        }
    }

    fn noop(&mut self) {
        self.draw();
        self.cycles += 1;
        self.inc_strength();
    }

    fn addx(&mut self, amount: isize) {
        self.noop();
        self.noop();
        self.x += amount;
    }

    fn tick(&mut self) {
        match self.mem[self.pc] {
            Addx(amount) => self.addx(amount),
            Noop => self.noop(),
        }
        self.pc += 1;
    }

    fn run(&mut self) {
        while self.pc < self.mem.len() {
            self.tick();
        }
    }

    fn render(self) -> String {
        String::from_iter(self.crt.into_iter().map(|r| String::from_iter(r.into_iter().map(|c| if c { '#' } else { '.' })) + "\n"))
    }
}

fn both_parts(input: String, part2: bool) {
    let mut cpu = CpuWithCrt {
        cycles: 0,
        pc: 0,
        x: 1,
        mem: input.lines().map(|l| l.parse::<Instruction>().unwrap()).collect::<Vec<Instruction>>(),
        total_strength: 0,
        crt: [[false; 40]; 6],
    };
    cpu.run();

    if part2 {
        println!("{}", cpu.render());
    } else {
        println!("{}", cpu.total_strength);
    }
}