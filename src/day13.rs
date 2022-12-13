use std::cmp::Ordering::{self, *};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Item {
    Integer(usize),
    List(Vec<Item>),
}

use Item::*;

impl FromStr for Item {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "[]" {
            Ok(List(vec![]))
        } else if &s[0..1] == "[" {
            let mut inner = &s[1..s.len() - 1];
            let mut level: usize = 0;
            let mut inner_items: Vec<Item> = vec![];

            'parse_loop: loop {
                for (index, char) in inner.chars().enumerate() {
                    if char == '[' {
                        level += 1;
                    } else if char == ']' {
                        level -= 1;
                    } else if char == ',' && level == 0 {
                        let (l, r) = inner.split_at(index);
                        inner_items.push(l.parse::<Item>().unwrap());
                        inner = &r[1..];
                        break;
                    }
                    if index == inner.len() - 1 {
                        inner_items.push(inner.parse::<Item>().unwrap());
                        break 'parse_loop Ok(List(inner_items));
                    }
                }
            }
        } else {
            Ok(Integer(s.parse::<usize>()?))
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Equal)
        } else {
            match self.compare(other) {
                None => Some(Equal),
                Some(true) => Some(Less),
                Some(false) => Some(Greater),
            }
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Item {
    fn compare(&self, other: &Item) -> Option<bool> {
        match (self, other) {
            (Integer(a), Integer(b)) => if a < b { Some(true) } else if a > b { Some(false) } else { None },
            (List(a), List(b)) => {
                for (index, left) in a.iter().enumerate() {
                    let right = match b.get(index) {
                        Some(v) => v,
                        None => return Some(false),
                    };

                    if let Some(b) = left.compare(right) {
                        return Some(b);
                    }
                }
                return if a.len() == b.len() {
                    None
                } else {
                    Some(true)
                }
            }
            (Integer(a), List(b)) => List(vec![Integer(*a)]).compare(&List(b.clone())),
            (List(a), Integer(b)) => List(a.clone()).compare(&List(vec![Integer(*b)])),
        }
    }
}

pub(crate) fn part1(input: String) {
    let packet_pairs: Vec<(Item, Item)> = input.split("\n\n").map(|pair| {
        let mut packets = pair.lines().map(|line| line.parse::<Item>().unwrap());
        (packets.next().unwrap(), packets.next().unwrap())
    }).collect();

    let mut index_sum: usize = 0;

    for (index, (a, b)) in packet_pairs.into_iter().enumerate() {
        if a.compare(&b).unwrap() == true {
            index_sum += index + 1;
        }
    }

    println!("{index_sum}");
}

pub(crate) fn part2(input: String) {
    let divider_0 = List(vec![List(vec![Integer(2)])]);
    let divider_1 = List(vec![List(vec![Integer(6)])]);

    let mut packets: Vec<Item> = input.replace("\n\n", "\n").lines().map(|line| line.parse::<Item>().unwrap()).collect();
    packets.push(divider_0.clone());
    packets.push(divider_1.clone());

    packets.sort();

    let mut decoder_key: usize = 1;

    for (index, packet) in packets.into_iter().enumerate() {
        if packet == divider_0 || packet == divider_1 {
            decoder_key *= index + 1;
        }
    }

    println!("{decoder_key}");
}