const TEST_PART1: fn(Vec<Vec<i32>>) -> i32 = |a| {
    (((a[0][0] <= a[1][0]) && (a[0][1] >= a[1][1]))
        || ((a[0][0] >= a[1][0]) && (a[0][1] <= a[1][1]))) as i32
};
const TEST_PART2: fn(Vec<Vec<i32>>) -> i32 = |a| {
    ((a[0][1] >= a[1][0] && a[0][1] <= a[1][1]) || (a[1][1] >= a[0][0] && a[1][1] <= a[0][1]))
        as i32
};

pub(crate) const PART1: fn(String) = |i| both_parts(i, TEST_PART1);
pub(crate) const PART2: fn(String) = |i| both_parts(i, TEST_PART2);

fn both_parts(input: String, test: fn(Vec<Vec<i32>>) -> i32) {
    println!(
        "{}",
        input
            .split("\n")
            .map(|e| {
                test(
                    e.split(",")
                        .map(|e| {
                            e.split("-")
                                .map(|e| e.parse::<i32>().unwrap())
                                .collect::<Vec<i32>>()
                        })
                        .collect::<Vec<Vec<i32>>>(),
                )
            })
            .sum::<i32>()
    );
}
