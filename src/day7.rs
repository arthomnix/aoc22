use std::collections::HashMap;

pub(crate) const PART1: fn(String) = |i| both_parts(i, false);
pub(crate) const PART2: fn(String) = |i| both_parts(i, true);

fn both_parts(input: String, part2: bool) {
    let mut dirs: HashMap<Vec<&str>, usize> = Default::default();
    let mut dirs_in: Vec<&str> = Vec::new();

    for line in input.lines() {
        let split: Vec<&str> = line.split(' ').collect();
        if split[0] == "$" && split[1] == "cd" {
            if split[2] == ".." {
                dirs_in.pop();
            } else {
                dirs_in.push(split[2]);
                dirs.insert(dirs_in.clone(), 0);
            }
        } else {
            match split[0].parse::<usize>() {
                Ok(size) => for i in 1..dirs_in.len() + 1 {
                    dirs.insert(dirs_in[..i].to_owned(), dirs.get(&dirs_in[..i]).unwrap() + size);
                },
                Err(_) => {},
            }
        }
    }

    if part2 {
        let required_space = *dirs.get(&*vec!["/"]).unwrap() - 40000000;
        let mut values: Vec<&usize> = dirs.values().collect();
        values.sort_unstable();
        println!("{}", values.iter().filter(|v| v > &&&required_space).min().unwrap())
    } else {
        let mut total: usize = 0;

        for dir in dirs {
            if dir.1 <= 100000 {
                total += dir.1;
            }
        }

        println!("{total}");
    }
}