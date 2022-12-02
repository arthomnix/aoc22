use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn get_score(self, other: &Rps) -> i32 {
        match self {
            Self::Rock => 1 + match other {
                Self::Rock => 3,
                Self::Paper => 0,
                Self::Scissors => 6,
            },
            Self::Paper => 2 + match other {
                Self::Rock => 6,
                Self::Paper => 3,
                Self::Scissors => 0,
            },
            Self::Scissors => 3 + match other {
                Self::Rock => 0,
                Self::Paper => 6,
                Self::Scissors => 3,
            },
        }
    }

    fn get_score_wld(self, wld: &str) -> i32 {
        match self {
            Self::Rock => match wld {
                "X" => Self::Scissors,
                "Y" => Self::Rock,
                "Z" => Self::Paper,
                _ => panic!(),
            } ,
            Self::Paper => match wld {
                "X" => Self::Rock,
                "Y" => Self::Paper,
                "Z" => Self::Scissors,
                _ => panic!(),
            }
            Self::Scissors => match wld {
                "X" => Self::Paper,
                "Y" => Self::Scissors,
                "Z" => Self::Rock,
                _ => panic!(),
            }
        }.get_score(&self)
    }
}

impl FromStr for Rps {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

pub(crate) fn part1(input: String) {
    println!("{}", input
        .replace("X", "A")
        .replace("Y", "B")
        .replace("Z", "C")
        .split("\n")
        .map(|e| {
            let game: Vec<Rps> = e.split(" ").map(|e| e.parse::<Rps>().unwrap()).collect();
            game[1].get_score(&game[0])
        })
        .sum::<i32>()
    );
}

pub(crate) fn part2(input: String) {
    println!("{}", input
        .split("\n")
        .map(|e| {
            let split: Vec<&str> = e.split(" ").collect();
            let opponent = split[0].parse::<Rps>().unwrap();
            opponent.get_score_wld(split[1])
        })
        .sum::<i32>()
    );
}