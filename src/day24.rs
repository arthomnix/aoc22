use std::collections::{HashSet, VecDeque};

pub(crate) const PART1: fn(String) = |i| both_parts(i, false);
pub(crate) const PART2: fn(String) = |i| both_parts(i, true);

// yknow maybe i should reuse my direction structs instead of having to write a new one basically every day
#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*;

type Point = (usize, usize);
type Blizzard = (Point, Direction);
type Blizzards = Vec<Blizzard>;

fn contains_point(blizzards: &Blizzards, point: Point) -> bool {
    for blizzard in blizzards.iter() {
        if blizzard.0 == point {
            return true;
        }
    }
    false
}

fn move_blizzards(blizzards: &Blizzards, width: usize, height: usize) -> Blizzards {
    blizzards.iter().map(|((x, y), dir)| {
        let (mut x, mut y) = (x - 1, y - 1);
        (x, y) = match dir {
            Up => (x, (y as isize - 1).rem_euclid(height as isize) as usize),
            Down => (x, (y + 1) % height),
            Left => ((x as isize - 1).rem_euclid(width as isize) as usize, y),
            Right => ((x + 1) % width, y),
        };
        ((x + 1, y + 1), *dir)
    }).collect()
}

// stolen from day 12 again
fn bfs_path_len(blizzards: &VecDeque<Blizzards>, start: Point, target: Point, width: usize, height: usize) -> Option<usize> {
    let mut explored: HashSet<(Point, usize)> = HashSet::from([(start, 0)]);

    let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
    queue.push_back((start, 0));

    while queue.len() > 0 {
        let (v, len) = queue.pop_front().unwrap();
        let blizzards = &blizzards[len];

        if v == target {
            return Some(len);
        }

        let mut allowed_edges = vec![];

        if (v.0 > 1 || (v.0 > 0 && (v.0 - 1, v.1) == target)) && !contains_point(blizzards, (v.0 - 1, v.1)) && v.1 <= height {
            allowed_edges.push((v.0 - 1, v.1));
        }
        if (v.0 < width || (v.0 + 1, v.1) == target) && !contains_point(blizzards, (v.0 + 1, v.1)) && v.1 > 0 {
            allowed_edges.push((v.0 + 1, v.1));
        }
        if (v.1 > 1 || (v.1 > 0 && (v.0, v.1 - 1) == target)) && !contains_point(blizzards, (v.0, v.1 - 1)) {
            allowed_edges.push((v.0, v.1 - 1));
        }
        if (v.1 < height || (v.0, v.1 + 1) == target) && !contains_point(blizzards, (v.0, v.1 + 1)) {
            allowed_edges.push((v.0, v.1 + 1));
        }

        if !contains_point(blizzards, v) {
            allowed_edges.push(v);
        }

        for w in allowed_edges {
            if explored.insert((w, len + 1)) {
                queue.push_back((w, len + 1));
            }
        }
    }

    None
}

fn both_parts(input: String, part2: bool) {
    let mut blizzards = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '>' => Some(((x, y), Right)),
                    '<' => Some(((x, y), Left)),
                    '^' => Some(((x, y), Up)),
                    'v' => Some(((x, y), Down)),
                    _ => None,
                })
        })
        .flatten()
        .collect();

    let width = input.lines().next().unwrap().len() - 2;
    let height = input.lines().collect::<Vec<&str>>().len() - 2;

    let mut precomp_blizzards: VecDeque<Blizzards> = VecDeque::with_capacity(1000);

    for _ in 0..1000 {
        blizzards = move_blizzards(&blizzards, width, height);
        precomp_blizzards.push_back(blizzards.clone());
    }

    let s1 = bfs_path_len(&precomp_blizzards, (1, 0), (width, height + 1), width, height).unwrap();
    if part2 {
        for _ in 0..s1 {
            precomp_blizzards.pop_front();
        }
        let s2 = bfs_path_len(&precomp_blizzards, (width, height + 1), (1, 0), width, height).unwrap();
        for _ in 0..s2 {
            precomp_blizzards.pop_front();
        }
        let s3 = bfs_path_len(&precomp_blizzards, (1, 0), (width, height + 1), width, height).unwrap();
        println!("{}", s1 + s2 + s3);
    } else {
        println!("{s1}");
    }
}