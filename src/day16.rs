use regex::Regex;
use std::{collections::HashMap, fmt::Debug, str::FromStr};

const MINUTES: u16 = 30;

type ValveID = String;

#[derive(Hash, PartialEq, Eq, Clone)]
struct Valve {
    id: ValveID,
    flow_rate: u32,
    connected_valves: Vec<ValveID>,
    open: bool,
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
            open: false,
        })
    }
}

impl Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "V({}): flow_rate={}, open={}, connected_valves={:?}",
            self.id, self.flow_rate, self.open, self.connected_valves
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
        assert!(!valve.open);
    }
}

#[derive(Clone)]
struct State {
    valves: HashMap<ValveID, Valve>,
    current_valve: ValveID,
    released_pressure: u32,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Current valve is {} and released pressure is {}",
            self.current_valve, self.released_pressure
        )?;

        for (_, valve) in self.valves.iter() {
            writeln!(f, "{:?}", valve)?;
        }

        writeln!(f, "\n")?;

        Ok(())
    }
}

impl State {
    fn new(valves: Vec<Valve>) -> Self {
        Self {
            valves: HashMap::from_iter(valves.into_iter().map(|v| (v.id.clone(), v))),
            current_valve: "AA".to_string(),
            released_pressure: 0,
        }
    }

    fn releasable_pressure(&self) -> u32 {
        self.valves
            .values()
            .filter(|v| v.open)
            .map(|v| v.flow_rate)
            .sum()
    }

    fn next_possible(&self) -> Vec<Self> {
        let current_valve = self.valves.get(&self.current_valve).unwrap();

        let mut states = current_valve
            .connected_valves
            .iter()
            .map(|next_valve| Self {
                current_valve: next_valve.clone(),
                ..self.clone()
            })
            .collect::<Vec<Self>>();

        // If the current valve is not open and has a flow rate, we also add a state where what we
        // do in this minute is opening the valve.
        if !current_valve.open && current_valve.flow_rate > 0 {
            let mut state_with_open_valve = self.clone();
            let open_valve = Valve {
                open: true,
                ..current_valve.clone()
            };
            state_with_open_valve
                .valves
                .insert(open_valve.id.clone(), open_valve);
            states.push(state_with_open_valve);
        }

        for mut state in states.iter_mut() {
            let releasable_pressure = state.releasable_pressure();
            state.released_pressure += releasable_pressure;
        }

        states
    }
}

pub fn run(input: &str) {
    let valves = input
        .lines()
        .map(|line| line.parse::<Valve>().unwrap())
        .collect::<Vec<Valve>>();

    let max_pressure = simulate_max_pressure(State::new(valves), MINUTES);
    println!("Max pressure: {}", max_pressure);
}

fn simulate_max_pressure(state: State, time_left: u16) -> u32 {
    if time_left == 0 {
        return state.released_pressure;
    }

    state
        .next_possible()
        .into_iter()
        .map(|s| simulate_max_pressure(s, time_left - 1))
        .max()
        .unwrap()
}
