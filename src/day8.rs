pub(crate) fn part1(input: String) {
    let grid: Vec<Vec<usize>> = input.lines().map(|l| l.chars().map(|c| c as usize - 48).collect::<Vec<usize>>()).collect();

    let mut total: usize = 2 * grid.len() + 2 * (grid[0].len() - 2);

    for (row, line) in grid
        .iter()
        .enumerate()
        .filter(|(r, l)| *r != 0 && *r != l.len() - 1)
    {
        for (col, height) in line
            .iter()
            .enumerate()
            .filter(|(c, _)| *c != 0 && *c != grid.len() - 1)
        {
            if grid[row][0..col].iter().max().unwrap() < height
                || grid[row][col + 1..line.len()].iter().max().unwrap() < height
                || grid[0..row].iter().map(|e| e[col]).max().unwrap() < *height
                || grid[row + 1..grid.len()].iter().map(|e| e[col]).max().unwrap() < *height
            {
                total += 1;
            }
        }
    }

    println!("{total}");
}

pub(crate) fn part2(input: String) {
    let grid: Vec<Vec<usize>> = input.lines().map(|l| l.chars().map(|c| c as usize - 48).collect::<Vec<usize>>()).collect();

    println!("{}", grid
        .iter()
        .map(|l| l
            .iter()
            .enumerate()
            .filter(|(c, _)| *c != 0 && *c != grid.len() - 1))
        .enumerate()
        .filter(|(r, _)| *r != 0 && *r != grid[0].len() - 1)
        .map(|(row, line)| {
            line.map(|(col, height)| {
                let mut score_up: usize = 0;
                let mut score_down: usize = 0;
                let mut score_left: usize = 0;
                let mut score_right: usize = 0;

                for tree in grid[0..row].iter().map(|e| e[col]).rev() {
                    score_up += 1;
                    if tree >= *height {
                        break;
                    }
                }

                for tree in grid[row + 1..grid.len()].iter().map(|e| e[col]) {
                    score_down += 1;
                    if tree >= *height {
                        break;
                    }
                }

                for tree in grid[row][0..col].iter().rev() {
                    score_left += 1;
                    if tree >= height {
                        break;
                    }
                }

                for tree in grid[row][col + 1..grid[0].len()].iter() {
                    score_right += 1;
                    if tree >= height {
                        break;
                    }
                }

                score_up * score_down * score_left * score_right
            })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
    );
}