use std::{fmt::Debug, str::FromStr, time::Instant};

use regex::Regex;

const TOTAL_MINUTES: u16 = 24;

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
    fn max_ore_cost(&self) -> Ore {
        vec![
            self.ore_robot_cost,
            self.clay_robot_cost,
            self.obsidian_robot_cost.0,
            self.geode_robot_cost.0,
        ]
        .iter()
        .max()
        .unwrap()
        .clone()
    }

    fn max_clay_cost(&self) -> Clay {
        self.obsidian_robot_cost.1.clone()
    }

    fn max_obsidian_cost(&self) -> Obsidian {
        self.geode_robot_cost.1.clone()
    }
}

impl FromStr for Blueprint {
    type Err = &'static str;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
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

        Ok(Self {
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        })
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

    max_ore_needed: Ore,
    max_clay_needed: Clay,
    max_obsidian_needed: Obsidian,
}

impl State {
    fn from_blueprint(blueprint: &Blueprint) -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: Ore(0),
            clay: Clay(0),
            obsidian: Obsidian(0),
            cracked_geodes: 0,
            max_ore_needed: blueprint.max_ore_cost(),
            max_clay_needed: blueprint.max_clay_cost(),
            max_obsidian_needed: blueprint.max_obsidian_cost(),
        }
    }

    fn possible_next_states(&self, blueprint: &Blueprint, time_left: u16) -> Vec<State> {
        let possible_next_states_with_new_robots = vec![
            self.build_ore_robot(blueprint, time_left),
            self.build_clay_robot(blueprint, time_left),
            self.build_obsidian_robot(blueprint, time_left),
            self.build_geode_robot(blueprint),
        ]
        .iter()
        .filter_map(|state| state.clone())
        .collect::<Vec<Self>>();

        let mut next_states = possible_next_states_with_new_robots;

        // Only have an "idle" state (where we only produce and don't build any robots)
        // if we don't have enough robots to build the max of each resource.
        if self.ore < self.max_ore_needed
            || self.clay < self.max_clay_needed
            || self.obsidian < self.max_obsidian_needed
        {
            next_states.push(self.clone());
        }

        for mut state in &mut next_states {
            state.ore = Ore(state.ore.0 + self.ore_robots);
            state.clay = Clay(state.clay.0 + self.clay_robots);
            state.obsidian = Obsidian(state.obsidian.0 + self.obsidian_robots);
            state.cracked_geodes = state.cracked_geodes + self.geode_robots;
        }

        next_states
    }

    fn build_ore_robot(&self, blueprint: &Blueprint, time_left: u16) -> Option<Self> {
        if self.ore_robots * time_left + self.ore.0 >= self.max_ore_needed.0 * time_left {
            None
        } else if self.ore >= blueprint.ore_robot_cost {
            Some(Self {
                ore_robots: self.ore_robots + 1,
                ore: Ore(self.ore.0 - blueprint.ore_robot_cost.0),
                ..self.clone()
            })
        } else {
            None
        }
    }

    fn build_clay_robot(&self, blueprint: &Blueprint, time_left: u16) -> Option<Self> {
        if self.clay_robots * time_left + self.clay.0 >= self.max_clay_needed.0 * time_left {
            None
        } else if self.ore >= blueprint.clay_robot_cost {
            Some(Self {
                clay_robots: self.clay_robots + 1,
                ore: Ore(self.ore.0 - blueprint.clay_robot_cost.0),
                ..self.clone()
            })
        } else {
            None
        }
    }

    fn build_obsidian_robot(&self, blueprint: &Blueprint, time_left: u16) -> Option<Self> {
        let (ore_cost, clay_cost) = blueprint.obsidian_robot_cost;

        if self.obsidian_robots * time_left + self.obsidian.0
            >= self.max_obsidian_needed.0 * time_left
        {
            None
        } else if self.ore >= ore_cost && self.clay >= clay_cost {
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
                ore: Ore(self.ore.0 - ore_cost.0),
                obsidian: Obsidian(self.obsidian.0 - obsidian_cost.0),
                ..self.clone()
            })
        } else {
            None
        }
    }

    fn utopistic_cracked_geodes(&self, time_left: u16) -> u32 {
        (self.cracked_geodes + time_left) as u32
    }
}

pub fn run(input: &str) {
    let blueprints = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
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
        State::from_blueprint(blueprint),
        blueprint,
        TOTAL_MINUTES,
        0,
        &mut explored_simulations,
    );

    (max_open_geodes, explored_simulations)
}

fn simulate_all_(
    current_state: State,
    blueprint: &Blueprint,
    time_left: u16,
    max_geodes_seen: u32,
    explored_simulations: &mut u64,
) -> u32 {
    // Simulation is finished, so we return its number of open geodes and count it as explored.
    if time_left == 0 {
        *explored_simulations += 1;
        return current_state.cracked_geodes as u32;
    }

    current_state
        .possible_next_states(blueprint, time_left)
        .iter()
        .filter_map(|next_state| {
            if next_state.utopistic_cracked_geodes(time_left) < max_geodes_seen {
                // No point in pursuing this simulation, as it will not yield more open geodes
                // than what we've already seen.
                None
            } else {
                Some(simulate_all_(
                    next_state.clone(),
                    blueprint,
                    time_left - 1,
                    next_state.cracked_geodes as u32,
                    explored_simulations,
                ))
            }
        })
        .max()
        .unwrap()
}
