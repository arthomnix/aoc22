mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 4 {
        let day: i32 = match args[1].parse() {
            Ok(n) => n,
            Err(_) => {
                help();
                return;
            }
        };
        let part: i32 = match args[2].parse() {
            Ok(n) => n,
            Err(_) => {
                help();
                return;
            }
        };

        let input = std::fs::read_to_string(&args[3]).unwrap();

        let function: fn(String) = match part {
            1 => match day {
                1 => day1::part1,
                2 => day2::part1,
                3 => day3::part1,
                4 => day4::PART1,
                5 => day5::PART1,
                6 => day6::PART1,
                7 => day7::PART1,
                8 => day8::part1,
                9 => day9::PART1,
                10 => day10::PART1,
                11 => day11::part1,
                12 => day12::part1,
                13 => day13::part1,
                14 => day14::part1,
                _ => unimplemented!(),
            },
            2 => match day {
                1 => day1::part2,
                2 => day2::part2,
                3 => day3::part2,
                4 => day4::PART2,
                5 => day5::PART2,
                6 => day6::PART2,
                7 => day7::PART2,
                8 => day8::part2,
                9 => day9::PART2,
                10 => day10::PART2,
                11 => day11::part2,
                12 => day12::part2,
                13 => day13::part2,
                14 => day14::part2,
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        };
        function(input);
    } else {
        help();
    }
}

fn help() {
    eprintln!("Usage: aoc22 [day] [part] [input data file]");
}
