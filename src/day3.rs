use std::str::FromStr;

#[derive(Clone, Debug)]
struct Rucksack {
    comp1: Vec<char>,
    comp2: Vec<char>,
}

impl Rucksack {
    fn get_common_item(self) -> Option<char> {
        for item in self.comp1 {
            if self.comp2.contains(&item) {
                return Some(item);
            }
        }
        None
    }

    fn common_item_priority(self) -> u8 {
        Self::item_priority(self.get_common_item().unwrap())
    }

    fn item_priority(item: char) -> u8 {
        if item.is_uppercase() {
            (item as u8) - 38
        } else {
            (item as u8) - 96
        }
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            comp1: s[0..s.len() / 2].chars().collect::<Vec<char>>(),
            comp2: s[s.len() / 2..s.len()].chars().collect::<Vec<char>>(),
        })
    }
}

struct ElfGroup<'a> {
    rucksacks: &'a [Rucksack; 3],
}

impl ElfGroup<'_> {
    fn get_common_item(self) -> Option<char> {
        let sacks = self.rucksacks.clone();
        let joined_sacks = sacks.map(|r| {
            let mut sack = r.clone();
            sack.comp1.append(&mut sack.comp2);
            sack.comp1
        });

        for item in &joined_sacks[0] {
            if joined_sacks[1].contains(&item) && joined_sacks[2].contains(&item) {
                return Some(*item);
            }
        }

        None
    }

    fn common_item_priority(self) -> u8 {
        Rucksack::item_priority(self.get_common_item().unwrap())
    }
}

pub(crate) fn part1(input: String) {
    println!(
        "{}",
        input
            .split("\n")
            .map(|e| e.parse::<Rucksack>().unwrap().common_item_priority() as usize)
            .sum::<usize>()
    );
}

pub(crate) fn part2(input: String) {
    let mut ctr = 1;
    println!(
        "{}",
        input
            .split(|c| {
                let mut res = false;
                if c == '\n' {
                    res = ctr % 3 == 0;
                    ctr += 1;
                }
                res
            })
            .map(|e| {
                let rucksacks = e
                    .split("\n")
                    .map(|e| e.parse::<Rucksack>().unwrap())
                    .collect::<Vec<Rucksack>>();
                ElfGroup {
                    rucksacks: rucksacks.as_slice().try_into().unwrap(),
                }
                .common_item_priority() as usize
            })
            .sum::<usize>()
    );
}
