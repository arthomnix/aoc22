use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
struct Blueprint {
    number: usize,
    ore_cost: usize,
    clay_cost: usize,
    obs_cost: (usize, usize),
    geo_cost: (usize, usize),
}

impl Blueprint {
    fn read(input: String) -> Vec<Self> {
        let regex = regex::Regex::new(r"Blueprint (?P<n>\d+): Each ore robot costs (?P<ore_cost>\d+) ore\. Each clay robot costs (?P<clay_cost>\d+) ore\. Each obsidian robot costs (?P<obs_ore>\d+) ore and (?P<obs_clay>\d+) clay\. Each geode robot costs (?P<geo_ore>\d+) ore and (?P<geo_obs>\d+) obsidian\.").unwrap();

        let mut blueprints: Vec<Self> = vec![];

        for caps in regex.captures_iter(&input) {
            blueprints.push(Self {
                number: (&caps["n"]).parse::<usize>().unwrap(),
                ore_cost: (&caps["ore_cost"]).parse::<usize>().unwrap(),
                clay_cost: (&caps["clay_cost"]).parse::<usize>().unwrap(),
                obs_cost: (
                        (&caps["obs_ore"]).parse::<usize>().unwrap(),
                        (&caps["obs_clay"]).parse::<usize>().unwrap(),
                    ),
                geo_cost: (
                        (&caps["geo_ore"]).parse::<usize>().unwrap(),
                        (&caps["geo_obs"]).parse::<usize>().unwrap(),
                    ),
            });
        }

        blueprints
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

use RobotType::*;

#[derive(Clone)]
struct State {
    time: usize,
    max_time: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

impl State {
    fn can_make(&self, robot: RobotType, blueprint: Blueprint) -> bool {
        match robot {
            Ore => self.ore >= blueprint.ore_cost,
            Clay => self.ore >= blueprint.clay_cost,
            Obsidian => self.ore >= blueprint.obs_cost.0 && self.clay >= blueprint.obs_cost.1,
            Geode => self.ore >= blueprint.geo_cost.0 && self.obsidian >= blueprint.geo_cost.1,
        }
    }

    fn make(&mut self, robot: RobotType, blueprint: Blueprint) {
        match robot {
            Ore => {
                self.ore -= blueprint.ore_cost;
                self.ore_robots += 1;
            }
            Clay => {
                self.ore -= blueprint.clay_cost;
                self.clay_robots += 1;
            }
            Obsidian => {
                self.ore -= blueprint.obs_cost.0;
                self.clay -= blueprint.obs_cost.1;
                self.obsidian_robots += 1;
            }
            Geode => {
                self.ore -= blueprint.geo_cost.0;
                self.obsidian -= blueprint.geo_cost.1;
                self.geode_robots += 1;
            }
        }
    }

    fn next(&mut self, robot: RobotType, blueprint: Blueprint) -> bool {
        loop {
            self.time += 1;
            let can_make = self.can_make(robot, blueprint);
            self.ore += self.ore_robots;
            self.clay += self.clay_robots;
            self.obsidian += self.obsidian_robots;
            self.geodes += self.geode_robots;
            if self.time == self.max_time {
                break false;
            }
            if can_make {
                self.make(robot, blueprint);
                break true;
            }
        }
    }
}

// quick and dirty
static mut CURRENT_BEST: usize = 0;

fn dfs(path: &mut Vec<RobotType>, allowed_robots: &mut HashSet<RobotType>, blueprint: Blueprint, state: State) -> usize {
    let max_ore = blueprint.ore_cost.max(blueprint.clay_cost).max(blueprint.obs_cost.0).max(blueprint.geo_cost.0);
    let max_clay = blueprint.obs_cost.1;
    let max_obs = blueprint.geo_cost.1;

    let mut max: usize = 0;

    for robot in allowed_robots.iter() {
        unsafe {
            if state.geodes + state.geode_robots * (state.max_time - state.time) + (state.max_time - state.time) * (state.max_time - state.time - 1) / 2 < CURRENT_BEST {
                continue;
            }
        }

        let mut new_allowed_robots = allowed_robots.clone();
        if *robot == Clay && !allowed_robots.contains(&Obsidian) {
            new_allowed_robots.insert(Obsidian);
        }
        if *robot == Obsidian && !allowed_robots.contains(&Geode) {
            new_allowed_robots.insert(Geode);
        }

        if path.iter().filter(|r| **r == Ore).collect::<Vec<&RobotType>>().len() == if robot == &Ore { max_ore - 1 } else { max_ore } {
            new_allowed_robots.remove(&Ore);
        }
        if path.iter().filter(|r| **r == Clay).collect::<Vec<&RobotType>>().len() == if robot == &Clay { max_clay - 1 } else { max_clay } {
            new_allowed_robots.remove(&Clay);
        }
        if path.iter().filter(|r| **r == Obsidian).collect::<Vec<&RobotType>>().len() == if robot == &Obsidian { max_obs - 1 } else { max_obs } {
            new_allowed_robots.remove(&Obsidian);
        }

        let mut new_path = path.clone();
        new_path.push(*robot);
        let mut new_state = state.clone();
        if new_state.next(*robot, blueprint) {
            let r_max = dfs(&mut new_path, &mut new_allowed_robots, blueprint, new_state);
            if r_max > max {
                max = r_max;
            }
        } else {
            if new_state.geodes > max {
                max = new_state.geodes;
            }
            unsafe {
                if new_state.geodes > CURRENT_BEST {
                    CURRENT_BEST = new_state.geodes
                }
            }
        }
    }

    max
}

fn get_geodes(blueprint: Blueprint, max_time: usize) -> usize {
    dfs(
        &mut vec![],
        &mut HashSet::from([Ore, Clay]),
        blueprint,
        State {
            time: 0,
            max_time,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    )
}

fn get_quality(blueprint: Blueprint) -> usize {
    get_geodes(blueprint, 24) * blueprint.number
}

pub(crate) fn part1(input: String) {
    let blueprints = Blueprint::read(input);
    println!("{}", blueprints.into_iter().map(|b| get_quality(b)).sum::<usize>());
}

pub(crate) fn part2(input: String) {
    let blueprints = Blueprint::read(input);
    println!("{}", blueprints[0..3.min(blueprints.len())].into_iter().map(|b| get_geodes(*b, 32)).product::<usize>());
}