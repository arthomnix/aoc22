use std::collections::HashMap;

fn mix(file: &mut Vec<isize>, mut index_map: HashMap<usize, usize>) -> HashMap<usize, usize> {
    for i in 0..file.len() {
        let index = *index_map.get(&i).unwrap();
        let val = file[index];
        file.remove(index);
        let new_index = (index as isize + val).rem_euclid(file.len() as isize) as usize;
        file.insert(new_index, val);
        index_map = index_map.into_iter().map(|(k, v)| {
            if k == i {
                (k, new_index)
            } else if new_index > index && v >= index && v <= new_index {
                (k, (v - 1) % file.len())
            } else if new_index < index && v <= index && v >= new_index {
                (k, (v + 1) % file.len())
            } else {
                (k, v)
            }
        }).collect();
    }
    index_map
}

pub(crate) fn part1(input: String) {
    let mut zero_index: Option<usize> = None;
    let mut file: Vec<isize> = input.lines().enumerate().map(|(i, l)| {
        let v = l.parse::<isize>().unwrap();
        if v == 0 {
            zero_index = Some(i);
        }
        v
    }).collect();

    let mut index_map: HashMap<usize, usize> = (0..file.len()).map(|i| (i, i)).collect();

    index_map = mix(&mut file, index_map);

    let new_zero_index = *index_map.get(&zero_index.unwrap()).unwrap();
    println!("{}", file[(new_zero_index + 1000) % file.len()] + file[(new_zero_index + 2000) % file.len()] + file[(new_zero_index + 3000) % file.len()]);
}

pub(crate) fn part2(input: String) {
    let mut zero_index: Option<usize> = None;
    let mut file: Vec<isize> = input.lines().enumerate().map(|(i, l)| {
        let v = l.parse::<isize>().unwrap();
        if v == 0 {
            zero_index = Some(i);
        }
        v * 811589153
    }).collect();

    let mut index_map: HashMap<usize, usize> = (0..file.len()).map(|i| (i, i)).collect();

    for _ in 0..10 {
        index_map = mix(&mut file, index_map);
    }

    let new_zero_index = *index_map.get(&zero_index.unwrap()).unwrap();
    println!("{}", file[(new_zero_index + 1000) % file.len()] + file[(new_zero_index + 2000) % file.len()] + file[(new_zero_index + 3000) % file.len()]);
}