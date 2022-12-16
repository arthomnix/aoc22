/*
This solution is based on maximising the function that takes in the order in which the
valves with nonzero flow rate are visited and returns the total pressure released. The
solutions run in an infinite loop and keep printing new highest maxima as they are
found. If it looks like it's stopped printing stuff it's probably found the right answer.
 */

use std::collections::{HashMap, VecDeque};
use rand::seq::SliceRandom;
use rand::Rng;

type ValveGraph = HashMap<[char; 2], (usize, Vec<[char; 2]>)>;
type DistMap = HashMap<([char; 2], [char; 2]), usize>;
type FlowMap = HashMap<[char; 2], usize>;

// stolen from day 12
fn bfs_path_len(graph: &ValveGraph, start: [char; 2], target: [char; 2]) -> Option<usize> {
    let mut explored: Vec<[char; 2]> = vec![start];

    let mut parentage: HashMap<[char; 2], [char; 2]> = Default::default();

    let mut queue: VecDeque<[char; 2]> = VecDeque::new();
    queue.push_back(start);

    while queue.len() > 0 {
        let mut v = queue.pop_front().unwrap();
        if v == target {
            let mut trace: Vec<[char; 2]> = vec![v];
            while let Some(parent) = parentage.get(&v) {
                trace.push(*parent);
                v = *parent;
            }
            return Some(trace.len() - 1);
        }
        for w in graph.get(&v).unwrap().1.iter() {
            if !explored.contains(w) {
                explored.push(*w);
                parentage.insert(*w, v);
                queue.push_back(*w);
            }
        }
    }

    None
}

fn read_graph(input: String) -> ValveGraph {
    let regex = regex::Regex::new(r"Valve (?P<valve>[A-Z]{2}) has flow rate=(?P<flow>\d+); tunnels? leads? to valves? (?P<tunnels>(?:[A-Z]{2}(?:, )?)+)").unwrap();

    let mut graph: ValveGraph = Default::default();

    for caps in regex.captures_iter(&input) {
        let mut valve_chars = (&caps["valve"]).chars();

        graph.insert(
            [valve_chars.next().unwrap(), valve_chars.next().unwrap()],
            (
                (&caps["flow"]).parse::<usize>().unwrap(),
                (&caps["tunnels"]).split(", ").map(|valve| {
                        let mut valve_chars = valve.chars();
                        [valve_chars.next().unwrap(), valve_chars.next().unwrap()]
                    }).collect::<Vec<[char; 2]>>()
                )
        );
    }

    graph
}

fn generate_dist_map(graph: ValveGraph, valves_with_flow: Vec<[char; 2]>) -> DistMap {
    let mut valves_with_flow = valves_with_flow.clone();
    valves_with_flow.push(['A', 'A']);
    let mut dist_map: DistMap = Default::default();

    for n1 in valves_with_flow.iter() {
        for n2 in valves_with_flow.iter() {
            dist_map.insert((*n1, *n2), bfs_path_len(&graph, *n1, *n2).unwrap());
        }
    }

    dist_map
}

fn score(perm: &Vec<[char; 2]>, dist_map: &DistMap, flow_map: &FlowMap) -> usize {
    let mut t: isize = 30;
    let mut score: isize = 0;

    let mut node = ['A', 'A'];

    for next in perm {
        t -= *dist_map.get(&(node, *next)).unwrap() as isize + 1;
        if t > 0 {
            score += *flow_map.get(next).unwrap() as isize * t;
        } else {
            break;
        }
        node = *next;
    }

    score as usize
}

fn maximise_score(valves: &mut Vec<[char; 2]>, dist_map: &DistMap, flow_map: &FlowMap) -> usize {
    let mut rng = rand::thread_rng();
    let mut r: Vec<usize> = (0..valves.len()).collect();

    let mut n = 0;
    let mut score_last_1k: usize = 0;

    loop {
        let current_score = score(&valves, &dist_map, &flow_map);
        r.shuffle(&mut rng);
        let tmp = valves[r[0]];
        valves[r[0]] = valves[r[1]];
        valves[r[1]] = tmp;
        if score(&valves, &dist_map, &flow_map) < current_score {
            let tmp = valves[r[0]];
            valves[r[0]] = valves[r[1]];
            valves[r[1]] = tmp;
        }

        n += 1;
        if n % 1000 == 0 {
            n = 0;
            if score_last_1k == current_score {
                break current_score;
            } else {
                score_last_1k = current_score;
            }
        }
    }
}

pub(crate) fn part1(input: String) {
    let graph = read_graph(input);
    let mut valves_with_flow: Vec<[char; 2]> = graph.iter().filter_map(|(k, v)| if v.0 > 0 { Some(*k) } else { None }).collect();
    let flow_map: FlowMap = graph.iter().map(|(k, v)| (*k, v.0)).collect();
    let dist_map = generate_dist_map(graph, valves_with_flow.clone());

    let mut max: usize = 0;
    let mut rng = rand::thread_rng();

    loop {
        let s = maximise_score(&mut valves_with_flow, &dist_map, &flow_map);
        if s > max {
            max = s;
            println!("{s}");
        }
        valves_with_flow.shuffle(&mut rng);
    }
}

// lots of code duplication here but oh well

fn score_elephant(ele_split: usize, perm: &Vec<[char; 2]>, dist_map: &DistMap, flow_map: &FlowMap) -> usize {
    let mut t: isize = 26;
    let mut score: isize = 0;

    let (perm_hum, perm_ele) = perm.split_at(ele_split);

    let mut node = ['A', 'A'];

    for next in perm_hum {
        t -= *dist_map.get(&(node, *next)).unwrap() as isize + 1;
        if t > 0 {
            score += *flow_map.get(next).unwrap() as isize * t;
        } else {
            break;
        }
        node = *next;
    }

    t = 26;
    node = ['A', 'A'];

    for next in perm_ele {
        t -= *dist_map.get(&(node, *next)).unwrap() as isize + 1;
        if t > 0 {
            score += *flow_map.get(next).unwrap() as isize * t;
        } else {
            break;
        }
        node = *next;
    }

    score as usize
}

fn maximise_score_elephant(valves: &mut Vec<[char; 2]>, dist_map: &DistMap, flow_map: &FlowMap) -> usize {
    let mut rng = rand::thread_rng();

    let mut ele_split = rng.gen_range(0..=valves.len());

    let mut r: Vec<usize> = (0..valves.len()).collect();

    let mut n = 0;
    let mut score_last_1k: usize = 0;

    loop {
        let current_score = score_elephant(ele_split, &valves, &dist_map, &flow_map);

        if rng.gen_bool(0.5) {
            r.shuffle(&mut rng);
            let tmp = valves[r[0]];
            valves[r[0]] = valves[r[1]];
            valves[r[1]] = tmp;
            if score_elephant(ele_split, &valves, &dist_map, &flow_map) < current_score {
                let tmp = valves[r[0]];
                valves[r[0]] = valves[r[1]];
                valves[r[1]] = tmp;
            }
        } else {
            if rng.gen_bool(0.5) && ele_split != valves.len() {
                ele_split += 1;
                if score_elephant(ele_split, &valves, &dist_map, &flow_map) < current_score {
                    ele_split -= 1;
                }
            } else if ele_split != 0 {
                ele_split -= 1;
                if score_elephant(ele_split, &valves, &dist_map, &flow_map) < current_score {
                    ele_split += 1;
                }
            }
        }

        n += 1;
        if n % 1000 == 0 {
            n = 0;
            if score_last_1k == current_score {
                break current_score;
            } else {
                score_last_1k = current_score;
            }
        }
    }
}


pub(crate) fn part2(input: String) {
    let graph = read_graph(input);
    let mut valves_with_flow: Vec<[char; 2]> = graph.iter().filter_map(|(k, v)| if v.0 > 0 { Some(*k) } else { None }).collect();
    let flow_map: FlowMap = graph.iter().map(|(k, v)| (*k, v.0)).collect();
    let dist_map = generate_dist_map(graph, valves_with_flow.clone());

    let mut max: usize = 0;
    let mut rng = rand::thread_rng();

    loop {
        let s = maximise_score_elephant(&mut valves_with_flow, &dist_map, &flow_map);
        if s > max {
            max = s;
            println!("{s}");
        }
        valves_with_flow.shuffle(&mut rng);
    }
}