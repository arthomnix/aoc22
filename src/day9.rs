use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

pub(crate) const PART1: fn(String) = |i| both_parts(i, 2);
pub(crate) const PART2: fn(String) = |i| both_parts(i, 10);

#[derive(Debug)]
struct ParseDirectionError {
    str: String,
}

impl Display for ParseDirectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "parsing direction failed: string must be 'U', 'D', 'L' or 'R', provided: {}", self.str)
    }
}

impl Error for ParseDirectionError {}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Up),
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            _ => Err(ParseDirectionError {
                str: s.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
enum ParseMoveInstructionError {
    ParseDirectionFailed(ParseDirectionError),
    ParseAmountFailed(ParseIntError),
}

use ParseMoveInstructionError::*;

impl Display for ParseMoveInstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseDirectionFailed(e) => Display::fmt(&e, f),
            ParseAmountFailed(e) => Display::fmt(&e, f),
        }
    }
}

impl Error for ParseMoveInstructionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseDirectionFailed(e) => Some(e),
            ParseAmountFailed(e) => Some(e),
        }
    }
}

#[derive(Debug)]
struct MoveInstruction {
    direction: Direction,
    amount: isize,
}

impl FromStr for MoveInstruction {
    type Err = ParseMoveInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        Ok(Self {
            direction: split[0].parse::<Direction>().map_err(ParseDirectionFailed)?,
            amount: split[1].parse::<isize>().map_err(ParseAmountFailed)?,
        })
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    fn is_touching(&self, other: &Self) -> bool {
        isize::abs_diff(self.x, other.x) <= 1 && isize::abs_diff(self.y, other.y) <= 1
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Point>,
    tail_visited: HashSet<Point>,
}

impl Rope {
    fn new(num_knots: usize) -> Self {
        Self {
            knots: vec![Point::new(); num_knots],
            tail_visited: HashSet::from([Point::new()]),
        }
    }

    fn move_head(&mut self, movement: MoveInstruction) {
        for _ in 0..movement.amount {
            match movement.direction {
                Up => self.knots[0].y += 1,
                Down => self.knots[0].y -= 1,
                Left => self.knots[0].x -= 1,
                Right => self.knots[0].x += 1,
            }

            for knot_index in 1..self.knots.len() {
                if !self.knots[knot_index].is_touching(&self.knots[knot_index - 1]) {
                    if self.knots[knot_index].x < self.knots[knot_index - 1].x {
                        self.knots[knot_index].x += 1;
                    } else if self.knots[knot_index].x > self.knots[knot_index - 1].x {
                        self.knots[knot_index].x -= 1;
                    }

                    if self.knots[knot_index].y < self.knots[knot_index - 1].y {
                        self.knots[knot_index].y += 1;
                    } else if self.knots[knot_index].y > self.knots[knot_index - 1].y {
                        self.knots[knot_index].y -= 1;
                    }

                    if knot_index == self.knots.len() - 1 {
                        self.tail_visited.insert(self.knots[knot_index]);
                    }
                }
            }
        }
    }
}

pub(crate) fn both_parts(input: String, num_knots: usize) {
    let instructions: Vec<MoveInstruction> = input.lines().map(|l| l.parse::<MoveInstruction>().unwrap()).collect();
    let mut rope = Rope::new(num_knots);

    for instruction in instructions {
        rope.move_head(instruction);
    }

    println!("{}", rope.tail_visited.len());
}