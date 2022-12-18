use std::collections::HashSet;

type Point3d = (isize, isize, isize);
type SparseGrid = HashSet<Point3d>;

fn parse_grid(input: String) -> SparseGrid {
    input.lines().map(|l| {
        let mut split = l.split(',');
        (split.next().unwrap().parse::<isize>().unwrap(), split.next().unwrap().parse::<isize>().unwrap(), split.next().unwrap().parse::<isize>().unwrap())
    }).collect()
}

pub(crate) fn part1(input: String) {
    let grid = parse_grid(input);
    let mut sa: isize = 0;
    for (x, y, z) in grid.iter() {
        if *x == 0 || !grid.contains(&(*x - 1, *y, *z)) {
            sa += 1;
        }
        if !grid.contains(&(*x + 1, *y, *z)) {
            sa += 1;
        }
        if *y == 0 || !grid.contains(&(*x, *y - 1, *z)) {
            sa += 1;
        }
        if !grid.contains(&(*x, *y + 1, *z)) {
            sa += 1;
        }
        if *z == 0 || !grid.contains(&(*x, *y, *z - 1)) {
            sa += 1;
        }
        if !grid.contains(&(*x, *y, *z + 1)) {
            sa += 1;
        }
    }
    println!("{sa}");
}

pub(crate) fn part2(input: String) {
    let grid = parse_grid(input);
    let x_max = grid.iter().map(|(x, _, _)| *x).max().unwrap();
    let y_max = grid.iter().map(|(_, y, _)| *y).max().unwrap();
    let z_max = grid.iter().map(|(_, _, z)| *z).max().unwrap();

    let mut ext_flood: SparseGrid = Default::default();
    let mut prev: SparseGrid = HashSet::from([(0, 0, 0)]);
    let mut new: SparseGrid = Default::default();

    loop {
        for (x, y, z) in prev.iter() {
            if *x >= 0 && !grid.contains(&(*x - 1, *y, *z)) {
                new.insert((*x - 1, *y, *z));
            }
            if *x <= x_max && !grid.contains(&(*x + 1, *y, *z)) {
                new.insert((*x + 1, *y, *z));
            }
            if *y >= 0 && !grid.contains(&(*x, *y - 1, *z)) {
                new.insert((*x, *y - 1, *z));
            }
            if *y <= y_max && !grid.contains(&(*x, *y + 1, *z)) {
                new.insert((*x, *y + 1, *z));
            }
            if *z >= 0 && !grid.contains(&(*x, *y, *z - 1)) {
                new.insert((*x, *y, *z - 1));
            }
            if *z <= z_max && !grid.contains(&(*x, *y, *z + 1)) {
                new.insert((*x, *y, *z + 1));
            }
        }
        let prev_f = ext_flood.clone();
        ext_flood = ext_flood.union(&prev).map(|i| *i).collect();
        if ext_flood == prev_f {
            break;
        }
        prev = new.clone();
        new = Default::default();
    }
    ext_flood = ext_flood.union(&prev).map(|i| *i).collect();

    let mut sa: isize = 0;
    for (x, y, z) in ext_flood.iter() {
        if *x >= 0 && grid.contains(&(*x - 1, *y, *z)) {
            sa += 1;
        }
        if grid.contains(&(*x + 1, *y, *z)) {
            sa += 1;
        }
        if *y >= 0 && grid.contains(&(*x, *y - 1, *z)) {
            sa += 1;
        }
        if grid.contains(&(*x, *y + 1, *z)) {
            sa += 1;
        }
        if *z >= 0 && grid.contains(&(*x, *y, *z - 1)) {
            sa += 1;
        }
        if grid.contains(&(*x, *y, *z + 1)) {
            sa += 1;
        }
    }
    println!("{sa}");
}