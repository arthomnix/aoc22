use std::num::ParseIntError;
use std::str::FromStr;

pub(crate) const PART1: fn(String) = |i| both_parts(i, false);
pub(crate) const PART2: fn(String) = |i| both_parts(i, true);

struct MoveInstruction {
    number: usize,
    from: usize,
    to: usize,
}

impl MoveInstruction {
    fn execute(self, stacks: &mut Vec<Vec<char>>, is_cratemover_9001: bool) {
        let mut new_values: Vec<char> = Vec::new();
        for _ in 0..self.number {
            let val = stacks[self.from].pop().unwrap();
            new_values.push(val);
        }
        if is_cratemover_9001 {
            new_values.reverse();
        }
        stacks[self.to].append(&mut new_values);
    }
}

impl FromStr for MoveInstruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<&str> = s.split(" ").collect();
        Ok(Self {
            number: vec[1].parse::<usize>()?,
            from: vec[3].parse::<usize>()? - 1,
            to: vec[5].parse::<usize>()? - 1,
        })
    }
}

fn both_parts(input: String, is_cratemover_9001: bool) {
    let vec: Vec<&str> = input.split("\n\n").collect();
    let moves: Vec<MoveInstruction> = vec[1]
        .split("\n")
        .map(|e| e.parse::<MoveInstruction>().unwrap())
        .collect();

    let mut stack_lines: Vec<&str> = vec[0].split("\n").collect();
    let num_stacks: usize = stack_lines
        .pop()
        .unwrap()
        .split(" ")
        .map(|e| e.parse::<usize>().unwrap_or_default())
        .max()
        .unwrap();
    stack_lines.reverse();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_stacks];

    for line in stack_lines {
        let mut chars = line.chars();
        for i in 0..num_stacks {
            chars.next();
            if let Some(char) = chars.next() {
                if char.is_uppercase() {
                    stacks[i].push(char);
                }
            }
            chars.next();
            chars.next();
        }
    }

    for move_inst in moves {
        move_inst.execute(&mut stacks, is_cratemover_9001);
    }

    let mut tops: String = String::new();
    for mut stack in stacks {
        tops.push(stack.pop().unwrap());
    }

    println!("{}", tops);
}
