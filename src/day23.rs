use std::collections::{HashMap, HashSet};

pub(crate) const PART1: fn(String) = |i| both_parts(i, false);
pub(crate) const PART2: fn(String) = |i| both_parts(i, true);


#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

use Direction::*;

impl Direction {
    fn move_by_mut(&self, pos: &mut (isize, isize)) {
        match self {
            North => {
                pos.1 -= 1;
            },
            Northeast => {
                pos.1 -= 1;
                pos.0 += 1;
            },
            East => {
                pos.0 += 1;
            },
            Southeast => {
                pos.1 += 1;
                pos.0 += 1;
            },
            South => {
                pos.1 += 1;
            },
            Southwest => {
                pos.1 += 1;
                pos.0 -= 1;
            },
            West => {
                pos.0 -= 1;
            },
            Northwest => {
                pos.1 -= 1;
                pos.0 -= 1;
            }
        }
    }

    fn move_by(&self, pos: (isize, isize)) -> (isize, isize) {
        let mut p = pos.clone();
        self.move_by_mut(&mut p);
        p
    }
}

const ALL_DIRECTIONS: [Direction; 8] = [North, Northeast, East, Southeast, South, Southwest, West, Northwest];

fn both_parts(input: String, part2: bool) {
    let mut direction_priority: Vec<Vec<Direction>> = vec![
        vec![North, Northeast, Northwest],
        vec![South, Southeast, Southwest],
        vec![West, Northwest, Southwest],
        vec![East, Northeast, Southeast]
    ];
    let mut elves: HashSet<(isize, isize)> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((x as isize, y as isize))
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .collect();
    let mut prev_elves: HashSet<(isize, isize)>;

    let mut round = 1;

    loop {
        let proposals: HashMap<(isize, isize), (isize, isize)> = elves
            .iter()
            .map(|(x, y)| {
                let mut proposed_pos = (*x, *y);

                if ALL_DIRECTIONS.iter().map(|dir| elves.contains(&dir.move_by((*x, *y)))).max().unwrap() {
                    for dirs in direction_priority.iter() {
                        let move_dir = dirs[0];
                        if !dirs.iter().map(|dir| elves.contains(&dir.move_by((*x, *y)))).max().unwrap() {
                            move_dir.move_by_mut(&mut proposed_pos);
                            break;
                        }
                    }
                }

                ((*x, *y), proposed_pos)
            })
            .collect();

        direction_priority.rotate_left(1);

        prev_elves = elves;
        elves = proposals
            .iter()
            .map(|(old, new)| {
                if proposals.values().filter(|k| *k == new).collect::<Vec<&(isize, isize)>>().len() > 1 {
                    *old
                } else {
                    *new
                }
            })
            .collect();

        if round == 10 && !part2 || prev_elves == elves {
            break;
        }
        round += 1;
    }

    if part2 {
        println!("{round}");
    } else {
        let max_x = elves.iter().map(|(x, _)| *x).max().unwrap();
        let min_x = elves.iter().map(|(x, _)| *x).min().unwrap();

        let max_y = elves.iter().map(|(_, y)| *y).max().unwrap();
        let min_y = elves.iter().map(|(_, y)| *y).min().unwrap();

        println!("{}", (max_y - min_y + 1) * (max_x - min_x + 1) - elves.len() as isize);
    }
}