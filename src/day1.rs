pub(crate) fn part1(input: String) {
    println!("{}", get_sums(input).iter().max().unwrap())
}

pub(crate) fn part2(input: String) {
    let mut sums = get_sums(input);
    sums.sort();
    println!("{}", sums.iter().rev().take(3).sum::<i32>());
}

fn get_sums(input: String) -> Vec<i32> {
    input
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .map(|e| e.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect()
}
