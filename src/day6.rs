use std::collections::VecDeque;

pub(crate) const PART1: fn(String) = |i| both_parts(i, false);
pub(crate) const PART2: fn(String) = |i| both_parts(i, true);

fn both_parts(input: String, part2: bool) {
    let mut c = 0;
    let mut buf: VecDeque<char> = VecDeque::new();

    'main_loop: for char in input.chars() {
        c += 1;
        buf.push_back(char);
        if buf.len() > if part2 { 14 } else { 4 } {
            buf.pop_front();

            let mut seen: Vec<char> = vec![buf[0]];

            for char in &buf.make_contiguous()[1..] {
                if seen.contains(char) {
                    continue 'main_loop;
                }
                seen.push(*char);
            }

            println!("{c}");
            return;
        }
    }
}