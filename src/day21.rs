use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
enum Yells {
    Number(usize),
    Maths(String, Operation, String),
}

use Operation::*;
use Yells::*;

fn parse_monkeys(input: String) -> HashMap<String, Yells> {
    let regex = regex::Regex::new(r"(?P<id>[a-z]{4}): (?:(?:(?P<lhs_id>[a-z]{4}) (?P<op>\+|-|\*|\x2F) (?P<rhs_id>[a-z]{4}))|(?P<n>\d+))").unwrap();
    regex.captures_iter(&input).map(|caps| {
        let id = &caps["id"];
        if let Some(n) = caps.name("n") {
            (id.to_string(), Number(n.as_str().parse().unwrap()))
        } else {
            (id.to_string(), Maths(
                (&caps["lhs_id"]).to_string(),
                match &caps["op"] {
                    "+" => Add,
                    "-" => Subtract,
                    "*" => Multiply,
                    "/" => Divide,
                    _ => panic!("Invalid operation")
                },
                (&caps["rhs_id"]).to_string()
            ))
        }
    }).collect()
}

fn evaluate_monkey(id: String, monkeys: &HashMap<String, Yells>, allow_humn: bool) -> Option<usize> {
    if id == "humn".to_string() && !allow_humn {
        None
    } else {
        let monkey = &monkeys[&id];
        match monkey {
            Number(n) => Some(*n),
            Maths(lhs, op, rhs) => {
                let lhs = evaluate_monkey(lhs.clone(), monkeys, allow_humn);
                let rhs = evaluate_monkey(rhs.clone(), monkeys, allow_humn);
                if lhs == None || rhs == None {
                    None
                } else {
                    let lhs = lhs.unwrap();
                    let rhs = rhs.unwrap();
                    Some(match op {
                        Add => lhs + rhs,
                        Subtract => lhs - rhs,
                        Multiply => lhs * rhs,
                        Divide => lhs / rhs,
                    })
                }
            }
        }
    }
}

fn evaluate_humn(id: String, n: usize, monkeys: &HashMap<String, Yells>) -> usize {
    let monkey = &monkeys[&id];
    if let Maths(lhs, op, rhs) = monkey {
        let lhs_value = evaluate_monkey(lhs.clone(), monkeys, false);
        let rhs_value = evaluate_monkey(rhs.clone(), monkeys, false);

        if let Some(v) = lhs_value {
            let eq = match op {
                Add => n - v,
                Subtract => v - n,
                Multiply => n / v,
                Divide => v / n,
            };
            if rhs == "humn" {
                eq
            } else {
                evaluate_humn(rhs.clone(), eq, monkeys)
            }
        } else if let Some(v) = rhs_value {
            let eq = match op {
                Add => n - v,
                Subtract => n + v,
                Multiply => n / v,
                Divide => n * v,
            };
            if lhs == "humn" {
                eq
            } else {
                evaluate_humn(lhs.clone(), eq, monkeys)
            }
        } else {
            panic!();
        }
    } else {
        panic!();
    }
}

pub(crate) fn part1(input: String) {
    let monkeys = parse_monkeys(input);
    println!("{}", evaluate_monkey("root".to_string(), &monkeys, true).unwrap());
}

pub(crate) fn part2(input: String) {
    let monkeys = parse_monkeys(input);
    if let Maths(lhs, _, rhs) = &monkeys["root"] {
        let lhs_value = evaluate_monkey(lhs.clone(), &monkeys, false);
        let rhs_value = evaluate_monkey(rhs.clone(), &monkeys, false);
        if let Some(v) = lhs_value {
            println!("{}", evaluate_humn(rhs.clone(), v, &monkeys));
        } else if let Some(v) = rhs_value {
            println!("{}", evaluate_humn(lhs.clone(), v, &monkeys))
        }
    }
}