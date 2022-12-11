use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Multiply,
}

use Operation::*;

#[derive(Clone, Debug)]
enum OperationSide {
    Old,
    Literal(usize),
}

use OperationSide::*;

#[derive(Debug)]
enum ParseMonkeyError {
    ParseOperationError,
    ParseIntError(ParseIntError),
}

use ParseMonkeyError::*;

impl Display for ParseMonkeyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseOperationError => write!(f, "Parsing monkey failed: operation must contain '+' or '*'"),
            ParseIntError(e) => Display::fmt(&e, f),
        }
    }
}

impl Error for ParseMonkeyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseOperationError => None,
            ParseIntError(e) => Some(e),
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<usize>,
    operation_lhs: OperationSide,
    operation_rhs: OperationSide,
    operation_op: Operation,
    test_divisible_by: usize,
    test_if_true: usize,
    test_if_false: usize,
    total_inspections: usize,
}

impl FromStr for Monkey {
    type Err = ParseMonkeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        let operation_line = lines[2].replace("  Operation: new = ", "");
        let operation_parts: Vec<&str> = operation_line.split(' ').collect();

        let operation_lhs = match operation_parts[0] {
            "old" => Old,
            n => Literal(n.parse::<usize>().map_err(ParseIntError)?),
        };

        let operation_rhs = match operation_parts[2] {
            "old" => Old,
            n => Literal(n.parse::<usize>().map_err(ParseIntError)?),
        };

        let operation_op = match operation_parts[1] {
            "+" => Add,
            "*" => Multiply,
            _ => Err(ParseOperationError)?,
        };

        let test_divisible_by = lines[3].replace("  Test: divisible by ", "").parse::<usize>().map_err(ParseIntError)?;
        let test_if_true = lines[4].replace("    If true: throw to monkey ", "").parse::<usize>().map_err(ParseIntError)?;
        let test_if_false = lines[5].replace("    If false: throw to monkey ", "").parse::<usize>().map_err(ParseIntError)?;

        Ok(Self {
            items: lines[1].replace("  Starting items: ", "").split(", ").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<usize>>(),
            operation_lhs,
            operation_rhs,
            operation_op,
            test_divisible_by,
            test_if_true,
            test_if_false,
            total_inspections: 0,
        })
    }
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    tests_product: usize,
}

impl FromStr for Monkeys {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s.split("\n\n").map( | s| s.parse::<Monkey>().unwrap()).collect::<Vec<Monkey>>();
        Ok(Self {
            monkeys: monkeys.clone(),
            tests_product: monkeys.iter().map(|m| m.test_divisible_by).product::<usize>(),
        })
    }
}

impl Monkeys {
    fn do_round(&mut self, divide_by_3: bool) {
        for idx in 0..self.monkeys.len() {
            for _ in 0..self.monkeys[idx].items.len() {
                let mut item = self.monkeys[idx].items.pop().unwrap();
                let lhs = match self.monkeys[idx].operation_lhs {
                    Old => item,
                    Literal(n) => n,
                };
                let rhs = match self.monkeys[idx].operation_rhs {
                    Old => item,
                    Literal(n) => n,
                };
                item = match self.monkeys[idx].operation_op {
                    Add => lhs + rhs,
                    Multiply => lhs * rhs,
                };

                if divide_by_3 {
                    item /= 3;
                }

                self.monkeys[idx].total_inspections += 1;

                if item % self.monkeys[idx].test_divisible_by == 0 {
                    let monkey = self.monkeys[idx].test_if_true;
                    self.monkeys[monkey].items.push(item % self.tests_product);
                } else {
                    let monkey = self.monkeys[idx].test_if_false;
                    self.monkeys[monkey].items.push(item % self.tests_product);
                }
            }
        }
    }
}

pub(crate) fn part1(input: String) {
    let mut monkeys = input.parse::<Monkeys>().unwrap();
    for _ in 0..20 {
        monkeys.do_round(true);
    }
    monkeys.monkeys.sort_by(|a, b| a.total_inspections.cmp(&b.total_inspections));
    println!("{}", monkeys.monkeys.into_iter().rev().take(2).map(|m| m.total_inspections).product::<usize>());
}

pub(crate) fn part2(input: String) {
    let mut monkeys = input.parse::<Monkeys>().unwrap();
    for _ in 0..10000 {
        monkeys.do_round(false);
    }
    monkeys.monkeys.sort_by(|a, b| a.total_inspections.cmp(&b.total_inspections));
    println!("{}", monkeys.monkeys.into_iter().rev().take(2).map(|m| m.total_inspections).product::<usize>());
}