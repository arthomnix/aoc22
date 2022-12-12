use std::collections::{HashMap, VecDeque};

fn bfs_path_len(graph: HashMap<(usize, usize), Vec<(usize, usize)>>, start: (usize, usize), target: (usize, usize)) -> Option<usize> {
    let mut explored: Vec<(usize, usize)> = vec![start];

    let mut parentage: HashMap<(usize, usize), (usize, usize)> = Default::default();

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(start);

    while queue.len() > 0 {
        let mut v = queue.pop_front().unwrap();
        if v == target {
            let mut trace: Vec<(usize, usize)> = vec![v];
            while let Some(parent) = parentage.get(&v) {
                trace.push(*parent);
                v = *parent;
            }
            return Some(trace.len() - 1);
        }
        for w in graph.get(&v).unwrap() {
            if !explored.contains(w) {
                explored.push(*w);
                parentage.insert(*w, v);
                queue.push_back(*w);
            }
        }
    }

    None
}

fn get_chars(input: String) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| {
            l.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

fn get_start_end(chars: Vec<Vec<char>>) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;

    for (y, line) in chars.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'S' {
                start = Some((x, y));
            }
            if *char == 'E' {
                end = Some((x, y));
            }
        }
    }

    (start, end)
}

fn get_heightmap(chars: Vec<Vec<char>>) -> Vec<Vec<usize>> {
    chars
        .into_iter()
        .map(|l| {
            l.into_iter()
                .map(|c| match c {
                    'S' => 0,
                    'E' => 25,
                    c => c as usize - 97,
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn get_graph(heightmap: Vec<Vec<usize>>) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = Default::default();

    for (y, line) in heightmap.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            let mut connections: Vec<(usize, usize)> = vec![];
            if y > 0 && heightmap[y - 1][x] <= height + 1 {
                connections.push((x, y - 1));
            }
            if y < heightmap.len() - 1 && heightmap[y + 1][x] <= height + 1 {
                connections.push((x, y + 1));
            }
            if x > 0 && heightmap[y][x - 1] <= height + 1 {
                connections.push((x - 1, y));
            }
            if x < line.len() - 1 && heightmap[y][x + 1] <= height + 1 {
                connections.push((x + 1, y));
            }
            graph.insert((x, y), connections);
        }
    }

    graph
}

pub(crate) fn part1(input: String) {
    let chars = get_chars(input);
    let (start, end) = get_start_end(chars.clone());
    let graph = get_graph(get_heightmap(chars));

    println!("{}", bfs_path_len(graph, start.unwrap(), end.unwrap()).unwrap());
}

// this is very unoptimised but it runs fast enough on release builds
pub(crate) fn part2(input: String) {
    let chars = get_chars(input);
    let (_, end) = get_start_end(chars.clone());
    let heightmap = get_heightmap(chars);
    let graph = get_graph(heightmap.clone());
    let mut possible_starts: Vec<(usize, usize)> = vec![];

    for (y, line) in heightmap.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 0 {
                possible_starts.push((x, y));
            }
        }
    }

    let mut lengths: Vec<usize> = vec![];

    for start in possible_starts {
        if let Some(len) = bfs_path_len(graph.clone(), start, end.unwrap()) {
            lengths.push(len);
        }
    }

    println!("{}", lengths.into_iter().min().unwrap());
}