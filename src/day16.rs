use std::collections::{HashMap, HashSet};

use petgraph::{
    adj::NodeIndex,
    dot::{Config, Dot},
    prelude::DiGraph,
    visit::{EdgeRef, IntoNeighbors},
};
use regex::Regex;

struct Valves {
    open_valves: HashMap<NodeIndex, u16>,
}

impl Valves {
    fn new() -> Self {
        Self {
            open_valves: HashMap::new(),
        }
    }

    fn state_as_string(&self) -> String {
        if self.open_valves.is_empty() {
            String::from("No valves are open")
        } else {
            String::from(format!(
                "Valves {:?} are open, releasing {} pressure",
                self.open_valves
                    .keys()
                    .map(|node| format!("{:?}", node))
                    .collect::<Vec<String>>()
                    .join(", "),
                self.open_valves.values().sum::<u16>()
            ))
        }
    }
}

pub fn run(input: &str) {
    let re = Regex::new(
        r"
        Valve (?P<valve>\w{2}) has flow rate=(?P<flow_rate>\d+); tunnel(s?) lead(s?) to valve(s?) (?P<valves>.+)
    "
        .trim(),
    )
    .unwrap();

    let mut graph: DiGraph<u16, ()> = DiGraph::new();
    let mut node_index_map = HashMap::new();

    let mut valves = Valves::new();

    let captured_lines = input.trim().lines().map(|line| re.captures(line).unwrap());

    for cap in captured_lines.clone() {
        let valve = cap["valve"].to_string();
        let flow_rate = cap["flow_rate"].parse::<u16>().unwrap();
        let node_index = graph.add_node(flow_rate);
        node_index_map.insert(valve, node_index);
    }

    for cap in captured_lines.clone() {
        let valve = cap["valve"].to_string();
        let valves = cap["valves"]
            .split(",")
            .map(|v| v.trim())
            .collect::<Vec<&str>>();

        for other_valve in valves {
            let node_index = node_index_map.get(&valve).unwrap();

            if let None = node_index_map.get(&other_valve.to_string()) {
                panic!("No node for valve: {}", other_valve);
            }

            let other_node_index = node_index_map.get(&other_valve.to_string()).unwrap();
            graph.add_edge(*node_index, *other_node_index, ());
        }
    }

    println!("{:?}", Dot::with_config(&graph, &[Config::NodeIndexLabel]));

    let mut current_node = node_index_map.get("AA").unwrap().clone();

    for minute in 1..=30 {
        println!("\n== Minute {minute} ==");
        println!("{}", valves.state_as_string());

        match valves.open_valves.get(&(current_node.index() as u32)) {
            // Current valve is already open, so we move to the next valve.
            Some(flow_rate) => {
                println!(
                    "You are at valve {:?} with flow rate {}",
                    current_node, flow_rate
                );
            }
            None if graph.node_weight(current_node).unwrap() == &0 => {
                println!(
                    "You are at valve {:?} with no flow, so moving on.",
                    current_node
                );
            }
            None => {
                println!(
                    "You are at valve {:?} with flow rate {}",
                    current_node,
                    graph.node_weight(current_node).unwrap()
                );
                println!("Opening valve: {:?}", current_node);
                valves.open_valves.insert(
                    current_node.index() as u32,
                    *graph.node_weight(current_node).unwrap(),
                );
                continue;
            }
        }

        let next_node = graph
            .neighbors(current_node)
            .max_by_key(|node| graph.node_weight(*node))
            .unwrap();

        println!("You move to valve {:?}", next_node);

        current_node = next_node;
    }
}
