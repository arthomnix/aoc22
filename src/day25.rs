fn snafu(mut n: isize) -> String {
    let digits = if n.abs() <= 1 { 1 } else { (n.abs() as f64).log(5.0).ceil() as usize };

    let mut snafu: Vec<isize> = vec![];

    for digit in (0..digits).rev() {
        let ds = 5isize.pow(digit as u32);
        let d = n / ds;
        n -= d * ds;
        let next = match d {
            4 => -1,
            3 => -2,
            2 => 2,
            1 => 1,
            0 => 0,
            _ => unreachable!(),
        };

        if next < 0 {
            let mut ps: Vec<isize> = vec![];
            loop {
                let prev = snafu.pop().unwrap_or(0);
                if prev < 2 {
                    snafu.push(prev + 1);
                    for n in ps {
                        snafu.push(n);
                    }
                    break;
                } else {
                    ps.push(-2);
                }
            }
        }

        snafu.push(next);
    }

    String::from_iter(snafu.into_iter().map(|d| match d {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => unreachable!(),
    }))
}

fn desnafu(snafu: &str) -> isize {
    let mut n = 0;
    for (d, c) in snafu.chars().enumerate() {
        n += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("invalid digit {c} in snafu number at index {d}"),
        } * 5isize.pow((snafu.len() - d - 1) as u32);
    }
    n
}

pub(crate) fn part1(input: String) {
    println!("{}", snafu(input
        .lines()
        .map(|l| desnafu(l))
        .sum::<isize>())
    );
}