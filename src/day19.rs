use std::{fmt::Debug, time::Instant};

use regex::Regex;

const TOTAL_MINUTES: u16 = 10;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Ore(u16);
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Clay(u16);
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Obsidian(u16);

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: Ore,
    clay_robot_cost: Ore,
    obsidian_robot_cost: (Ore, Clay),
    geode_robot_cost: (Ore, Obsidian),
}

impl Blueprint {
    fn from_string(string: &str) -> Self {
        let ore_robot_cost = Ore(Regex::new(r"Each ore robot costs (\d+) ore")
            .unwrap()
            .captures_iter(string)
            .next()
            .unwrap()[1]
            .parse::<u16>()
            .unwrap());

        let clay_robot_cost = Ore(Regex::new(r"Each clay robot costs (\d+) ore")
            .unwrap()
            .captures_iter(string)
            .next()
            .unwrap()[1]
            .parse::<u16>()
            .unwrap());

        let obsidian_robot_costs =
            Regex::new(r"Each obsidian robot costs (\d+) ore and (\d+) clay")
                .unwrap()
                .captures_iter(string)
                .next()
                .unwrap();

        let obsidian_robot_cost = (
            Ore(obsidian_robot_costs[1].parse::<u16>().unwrap()),
            Clay(obsidian_robot_costs[2].parse::<u16>().unwrap()),
        );

        let geode_robot_costs = Regex::new(r"Each geode robot costs (\d+) ore and (\d+) obsidian")
            .unwrap()
            .captures_iter(string)
            .next()
            .unwrap();

        let geode_robot_cost = (
            Ore(geode_robot_costs[1].parse::<u16>().unwrap()),
            Obsidian(geode_robot_costs[2].parse::<u16>().unwrap()),
        );

        Self {
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct State {
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
    ore: Ore,
    clay: Clay,
    obsidian: Obsidian,
    cracked_geodes: u16,
}

impl State {
    fn new() -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: Ore(0),
            clay: Clay(0),
            obsidian: Obsidian(0),
            cracked_geodes: 0,
        }
    }

    fn build_ore_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.ore_robot_cost {
            Some(Self {
                ore_robots: self.ore_robots + 1,
                ore: Ore(self.ore.0 - blueprint.ore_robot_cost.0),
                ..self.clone()
            })
        } else {
            None
        }
    }

    fn build_clay_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.clay_robot_cost {
            Some(Self {
                clay_robots: self.clay_robots + 1,
                ore: Ore(self.ore.0 - blueprint.clay_robot_cost.0),
                ..self.clone()
            })
        } else {
            None
        }
    }

    fn build_obsidian_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        let (ore_cost, clay_cost) = blueprint.obsidian_robot_cost;

        if self.ore >= ore_cost && self.clay >= clay_cost {
            Some(Self {
                obsidian_robots: self.obsidian_robots + 1,
                ore: Ore(self.ore.0 - blueprint.obsidian_robot_cost.0 .0),
                clay: Clay(self.clay.0 - blueprint.obsidian_robot_cost.1 .0),
                ..self.clone()
            })
        } else {
            None
        }
    }

    fn build_geode_robot(&self, blueprint: &Blueprint) -> Option<Self> {
        let (ore_cost, obsidian_cost) = blueprint.geode_robot_cost;

        if self.ore >= ore_cost && self.obsidian >= obsidian_cost {
            Some(Self {
                geode_robots: self.geode_robots + 1,
                ore: Ore(self.ore.0 - blueprint.geode_robot_cost.0 .0),
                obsidian: Obsidian(self.obsidian.0 - blueprint.geode_robot_cost.1 .0),
                ..self.clone()
            })
        } else {
            None
        }
    }

    fn possible_next_states(&self, blueprint: &Blueprint) -> Vec<State> {
        let previous_ore_robots = self.ore_robots;
        let previous_clay_robots = self.clay_robots;
        let previous_obsidian_robots = self.obsidian_robots;
        let previous_geode_robots = self.geode_robots;

        let possible_next_states = vec![
            Some(self.clone()),
            self.build_ore_robot(blueprint),
            self.build_clay_robot(blueprint),
            self.build_obsidian_robot(blueprint),
            self.build_geode_robot(blueprint),
        ];

        let mut next_states = vec![];

        for state in possible_next_states {
            if let Some(mut state) = state {
                state.ore = Ore(state.ore.0 + previous_ore_robots);
                state.clay = Clay(state.clay.0 + previous_clay_robots);
                state.obsidian = Obsidian(state.obsidian.0 + previous_obsidian_robots);
                state.cracked_geodes = state.cracked_geodes + previous_geode_robots;
                next_states.push(state);
            }
        }

        next_states
    }
}

pub fn run(input: &str) {
    let blueprints = input
        .lines()
        .map(Blueprint::from_string)
        .collect::<Vec<Blueprint>>();

    let mut total_quality_level = 0;

    for (index, blueprint) in blueprints.iter().enumerate() {
        let blueprint_index = index + 1;

        let start_time = Instant::now();
        let (max_open_geodes, explored_simulations) = simulate_all(blueprint);
        let elapsed = start_time.elapsed();

        println!("\n== Blueprint {blueprint_index} ==");
        println!("Explored {} unique simulations, simulating {} minutes (took {:.2?}). Max open geodes: {}",
            explored_simulations, TOTAL_MINUTES, elapsed, max_open_geodes
        );

        total_quality_level += max_open_geodes * blueprint_index as u32;
    }

    println!("\n\nTotal quality level: {}", total_quality_level);
}

fn simulate_all(blueprint: &Blueprint) -> (u32, u64) {
    let mut explored_simulations = 0;

    let max_open_geodes = simulate_all_(
        State::new(),
        blueprint,
        TOTAL_MINUTES,
        &mut explored_simulations,
    );

    (max_open_geodes, explored_simulations)
}

fn simulate_all_(
    current_state: State,
    blueprint: &Blueprint,
    time_left: u16,
    explored_simulations: &mut u64,
) -> u32 {
    // Simulation is finished, so we return its number of open geodes and count it as explored.
    if time_left == 0 {
        *explored_simulations += 1;
        return current_state.cracked_geodes as u32;
    }

    current_state
        .possible_next_states(blueprint)
        .iter()
        .map(|s| simulate_all_(s.clone(), blueprint, time_left - 1, explored_simulations))
        .max()
        .unwrap()
}
