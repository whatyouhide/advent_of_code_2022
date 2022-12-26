use infinitable::Infinitable;
use petgraph::graphmap;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::FromStr,
};

const MINUTES: u16 = 30;

type ValveID = String;
type ValveGraph<'a> = graphmap::UnGraphMap<&'a str, ()>;

#[derive(Debug)]
struct DistanceMatrix<'a>(HashMap<(&'a str, &'a str), u32>);

impl<'a> DistanceMatrix<'a> {
    pub fn from_graph(graph: &ValveGraph<'a>) -> DistanceMatrix<'a> {
        // If a node is not reachable from another node, the (u, v) pair in the returned matrix
        // is just not present.
        let mut dist = HashMap::new();

        // Initialize edges to their weight (hard-coded 1 on our case).
        for (u, v, ()) in graph.all_edges() {
            dist.insert((u, v), 1);
        }

        // Initialize the diagonals to 0.
        for v in graph.nodes() {
            dist.insert((v, v), 0);
        }

        for k in graph.nodes() {
            for i in graph.nodes() {
                for j in graph.nodes() {
                    let d_ik = Self::get_(&dist, i, k);
                    let d_kj = Self::get_(&dist, k, j);
                    let d_ij = Self::get_(&dist, i, j);

                    let sum = d_ik.and_then(|f| d_kj.map(|s| f + s));

                    if Infinitable::finite_or_infinity(d_ij) > Infinitable::finite_or_infinity(sum)
                    {
                        dist.insert((i, j), sum.unwrap());
                    }
                }
            }
        }

        Self(dist)
    }

    pub fn get(&self, u: &'a str, v: &'a str) -> Option<u32> {
        Self::get_(&self.0, u, v)
    }

    fn get_(map: &HashMap<(&str, &str), u32>, u: &str, v: &str) -> Option<u32> {
        map.get(&(u, v)).or(map.get(&(v, u))).copied()
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Valve {
    id: ValveID,
    flow_rate: u32,
    connected_valves: Vec<ValveID>,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"
            Valve (?P<valve>\w{2}) has flow rate=(?P<flow_rate>\d+); tunnel(s?) lead(s?) to valve(s?) (?P<valves>.+)
        "
            .trim()
        )
        .unwrap();

        let cap = re.captures(s).unwrap();

        Ok(Self {
            id: cap["valve"].to_string(),
            flow_rate: cap["flow_rate"].parse::<u32>().unwrap(),
            connected_valves: cap["valves"]
                .split(",")
                .map(|v| v.trim().to_string())
                .collect::<Vec<ValveID>>(),
        })
    }
}

impl Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "V({}): flow_rate={}, connected_valves={:?}",
            self.id, self.flow_rate, self.connected_valves
        )
    }
}

#[cfg(test)]
mod valve_tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = "Valve AA has flow rate=1; tunnels lead to valves BB, CC";
        let valve = Valve::from_str(input).unwrap();

        assert_eq!(valve.id, "AA");
        assert_eq!(valve.flow_rate, 1);
        assert_eq!(valve.connected_valves, vec!["BB", "CC"]);
    }
}

#[derive(Clone)]
struct State {
    open_valves: HashSet<ValveID>,
    current_valve: ValveID,
    released_pressure: u32,
    time_left: u16,
    path: String,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "open valves: {:?}, time_left: {}. {}\n",
            self.open_valves, self.time_left, self.path
        )
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            open_valves: HashSet::new(),
            current_valve: "AA".to_string(),
            released_pressure: 0,
            time_left: MINUTES,
            path: String::from("AA"),
        }
    }

    pub fn next_states(
        &self,
        graph: &ValveGraph,
        distance_matrix: &DistanceMatrix,
        flow_rates: &HashMap<String, u32>,
    ) -> Vec<Self> {
        let mut next_states = Vec::new();

        // For each *other* valve that is still closed and has a positive flow rate, a next state
        // is to go to that valve and open it.
        for closed_valve in graph
            .nodes()
            .filter(|v| !self.open_valves.contains(*v))
            .filter(|v| flow_rates[*v] > 0)
            .filter(|v| v != &self.current_valve)
        {
            let distance = distance_matrix
                .get(&self.current_valve.as_str(), closed_valve.clone())
                .unwrap() as u16;

            // Adding one to simulate opening the valve.
            if distance + 1 >= self.time_left {
                continue;
            }

            let mut next_state = self.clone();

            // First, we detract the time it takes to get to the valve.
            next_state.time_left -= distance + 1;

            // Then, we calculate the increased pressure with the current open valves.
            next_state.increase_released_pressure(distance + 1, &flow_rates);

            // Then we update the current valve and "open it" (insert it int he current open
            // valves).
            next_state.current_valve = closed_valve.to_string();
            next_state.open_valves.insert(closed_valve.to_string());

            // We update the path for debugging purposes.
            next_state
                .path
                .push_str(format!("->{}->[open {}]", closed_valve, closed_valve).as_str());

            next_states.push(next_state);
        }

        // If there are no valves we can go to, we can still simulate that
        // we basically wait for the remaining time and let pressure flow through
        // the open valves.
        if next_states.is_empty() && self.time_left > 0 {
            let mut next_state = self.clone();
            next_state
                .path
                .push_str(format!("->wait({})", self.time_left).as_str());
            next_state.increase_released_pressure(self.time_left, &flow_rates);
            next_state.time_left = 0;
            next_states.push(next_state);
        }

        next_states
    }

    fn increase_released_pressure(&mut self, time: u16, flow_rates: &HashMap<String, u32>) {
        let increment = self
            .open_valves
            .iter()
            .map(|v| flow_rates[v.as_str()])
            .sum::<u32>();

        self.released_pressure += increment * time as u32;

        self.path
            .push_str(format!("(+{} times {})", increment, time).as_str());
    }
}

// Main program.

pub fn run(input: &str) {
    // let input = r#"
    // Valve AA has flow rate=0; tunnels lead to valves BB
    // Valve BB has flow rate=13; tunnels lead to valves AA
    // "#
    // .trim();

    let valves = input
        .lines()
        .map(|line| line.parse::<Valve>().unwrap())
        .collect::<Vec<Valve>>();

    let mut explored_states = 0;
    let graph = graph_from_valves(&valves);
    let distance_matrix = DistanceMatrix::from_graph(&graph);
    let flow_rates: HashMap<String, u32> =
        HashMap::from_iter(valves.iter().map(|v| (v.id.clone(), v.flow_rate)));

    let max_pressure = run_simulation(
        State::new(),
        &distance_matrix,
        &graph,
        &flow_rates,
        &mut explored_states,
    );

    println!(
        "Max pressure found after exploring {explored_states} states: {}",
        max_pressure
    );
}

fn run_simulation(
    state: State,
    distance_matrix: &DistanceMatrix,
    graph: &ValveGraph,
    flow_rates: &HashMap<String, u32>,
    explored_states: &mut u32,
) -> u32 {
    *explored_states += 1;

    state
        .next_states(graph, distance_matrix, flow_rates)
        .into_iter()
        .map(|s| run_simulation(s, distance_matrix, graph, flow_rates, explored_states))
        .max()
        .unwrap_or(state.released_pressure)
}

fn graph_from_valves(valves: &Vec<Valve>) -> ValveGraph {
    let mut graph: ValveGraph = graphmap::UnGraphMap::new();

    for valve in valves.iter() {
        graph.add_node(valve.id.as_str());
    }

    for valve in valves.iter() {
        for connected_valve in valve.connected_valves.iter() {
            graph.add_edge(valve.id.as_str(), connected_valve.as_str(), ());
        }
    }

    graph
}
