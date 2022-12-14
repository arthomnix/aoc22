//let's actually bother to use type aliases today
type Point = (usize, usize);
type Line = Vec<Point>;
type Grid = Vec<Vec<bool>>;

fn line_from_str(line: &str) -> Line {
    line.split(" -> ").map(|s| {
        let mut coords = s.split(',');
        (coords.next().unwrap().parse::<usize>().unwrap(), coords.next().unwrap().parse::<usize>().unwrap())
    }).collect()
}

fn grid_from_lines(lines: Vec<Line>) -> Grid {
    let max_y = lines.iter().map(|l| l.iter().map(|p| p.1).max().unwrap()).max().unwrap();
    let mut grid: Grid = vec![vec![false; 1000]; max_y + 3];
    grid[max_y + 2] = vec![true; 1000];

    for line in lines {
        let mut points_iter = line.into_iter();
        let mut prev_point = points_iter.next().unwrap();

        for point in points_iter {
            if prev_point.0 == point.0 { // vertical
                if prev_point.1 < point.1 {
                    for y in prev_point.1..=point.1 {
                        grid[y][point.0] = true;
                    }
                } else if prev_point.1 > point.1 {
                    for y in point.1..=prev_point.1 {
                        grid[y][point.0] = true;
                    }
                }
            } else if prev_point.1 == point.1 { // horizontal
                if prev_point.0 < point.0 {
                    for x in prev_point.0..=point.0 {
                        grid[point.1][x] = true;
                    }
                } else if prev_point.0 > point.0 {
                    for x in point.0..=prev_point.0 {
                        grid[point.1][x] = true;
                    }
                }
            } else {
                panic!("line segment not straight");
            }

            prev_point = point;
        }
    }

    grid
}

pub(crate) fn part1(input: String) {
    let mut grid = grid_from_lines(input.lines().map(|l| line_from_str(l)).collect());
    let mut grains = 0usize;

    println!("{}", 'sim_loop: loop {
        let mut sand_pos = (500usize, 0usize);
        while !grid[sand_pos.1 + 1][sand_pos.0] || !grid[sand_pos.1 + 1][sand_pos.0 - 1] || !grid[sand_pos.1 + 1][sand_pos.0 + 1] {
            if !grid[sand_pos.1 + 1][sand_pos.0] {
                sand_pos = (sand_pos.0, sand_pos.1 + 1);
            } else if !grid[sand_pos.1 + 1][sand_pos.0 - 1] {
                sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
            } else if !grid[sand_pos.1 + 1][sand_pos.0 + 1] {
                sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
            }

            if sand_pos.1 == grid.len() - 3 {
                break 'sim_loop grains;
            }
        }
        grid[sand_pos.1][sand_pos.0] = true;
        grains += 1;
    });
}

pub(crate) fn part2(input: String) {
    let mut grid = grid_from_lines(input.lines().map(|l| line_from_str(l)).collect());
    let mut grains = 0usize;

    println!("{}", loop {
        let mut sand_pos = (500usize, 0usize);
        while !grid[sand_pos.1 + 1][sand_pos.0] || !grid[sand_pos.1 + 1][sand_pos.0 - 1] || !grid[sand_pos.1 + 1][sand_pos.0 + 1] {
            if !grid[sand_pos.1 + 1][sand_pos.0] {
                sand_pos = (sand_pos.0, sand_pos.1 + 1);
            } else if !grid[sand_pos.1 + 1][sand_pos.0 - 1] {
                sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
            } else if !grid[sand_pos.1 + 1][sand_pos.0 + 1] {
                sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
            }
        }

        grains += 1;

        if sand_pos == (500usize, 0usize) {
            break grains;
        }
        grid[sand_pos.1][sand_pos.0] = true;
    });
}