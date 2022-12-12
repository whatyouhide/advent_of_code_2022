use infinitable::*;

use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, PartialOrd, Ord)]
struct Node(i32, i32);

type DistanceMap = HashMap<Node, Infinitable<i32>>;

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<Node, char>,
}

impl Graph {
    pub fn from(input: &str) -> Graph {
        let mut nodes = HashMap::new();

        for (row_index, row) in input.trim().lines().enumerate() {
            for (column_index, char) in row.trim().chars().enumerate() {
                nodes.insert(Node(row_index as i32, column_index as i32), char);
            }
        }

        Graph { nodes }
    }

    fn distance_between_nodes(&self, node1: Node, node2: Node) -> u16 {
        let node1_val = self.nodes[&node1] as i32;
        let node2_val = self.nodes[&node2] as i32;

        if node2_val == node1_val {
            2
        } else {
            1
        }
    }

    fn find_node(&self, target: char) -> Option<Node> {
        for (node, char) in &self.nodes {
            if *char == target {
                return Some(node.clone());
            }
        }

        return None;
    }

    fn neighbors(&self, node: Node) -> Vec<Node> {
        let mut nodes = vec![];
        let char_at_node = self.nodes[&node];

        let north_node = Node(node.0 - 1, node.1);
        let south_node = Node(node.0 + 1, node.1);
        let east_node = Node(node.0, node.1 + 1);
        let west_node = Node(node.0, node.1 - 1);

        if let Some(char) = self.nodes.get(&north_node) {
            if chars_are_connectable(&char_at_node, &char) {
                nodes.push(north_node);
            }
        }
        if let Some(char) = self.nodes.get(&south_node) {
            if chars_are_connectable(&char_at_node, &char) {
                nodes.push(south_node);
            }
        }
        if let Some(char) = self.nodes.get(&east_node) {
            if chars_are_connectable(&char_at_node, &char) {
                nodes.push(east_node);
            }
        }
        if let Some(char) = self.nodes.get(&west_node) {
            if chars_are_connectable(&char_at_node, &char) {
                nodes.push(west_node);
            }
        }

        nodes
    }
}

pub fn run(input: &str) {
    let mut graph = Graph::from(input);
    let start_node = graph.find_node('S').unwrap();
    let end_node = graph.find_node('E').unwrap();

    graph.nodes.insert(start_node, 'a');
    graph.nodes.insert(end_node, 'z');

    let mut dijkstras = vec![];

    for (node, char) in &graph.nodes {
        if *char == 'a' {
            println!("Running Dijkstra's from node {:?}", node);

            let mut graph = graph.clone();

            match dijkstra(&mut graph, *node, end_node) {
                Some(hops) => dijkstras.push(hops),
                None => continue,
            };
        }
    }

    let hops = dijkstras.iter().min().unwrap();

    println!("Path with the least hops has {} hops", hops);
}

fn dijkstra(graph: &mut Graph, start_node: Node, end_node: Node) -> Option<u32> {
    let mut unvisited_set: HashSet<Node> = HashSet::new();
    let mut distances: DistanceMap = HashMap::new();
    let mut prev: HashMap<Node, Option<Node>> = HashMap::new();

    for node in graph.nodes.keys() {
        unvisited_set.insert(node.clone());
        distances.insert(node.clone(), Infinity);
        prev.insert(node.clone(), None);
    }

    distances.insert(start_node, Finite(0));

    while unvisited_set.len() > 0 {
        // Pick the position with minimum distance that is in the unvisited set.
        let u = find_node_with_min_distance(&distances, &unvisited_set);

        if u == end_node {
            break;
        }

        unvisited_set.remove(&u);

        for v in graph
            .neighbors(u)
            .iter()
            .filter(|node| unvisited_set.contains(*node))
        {
            let alt = match distances[&u] {
                Infinity => continue,
                NegativeInfinity => continue,
                Finite(alt) => alt,
            } + graph.distance_between_nodes(u, v.clone()) as i32;

            if Finite(alt) < distances[v] {
                distances.insert(v.clone(), Finite(alt));
                prev.insert(v.clone(), Some(u));
            }
        }
    }

    let mut s = vec![];
    let mut u = Some(end_node);

    if let Some(_) = prev[&u.unwrap()] {
        while let Some(u1) = u {
            s.insert(0, u1);
            u = prev[&u1];
        }
    } else {
        return None;
    }

    Some((s.len() - 1) as u32)
}

fn chars_are_connectable(char1: &char, char2: &char) -> bool {
    (*char2 as i32) <= (*char1 as i32) + 1
}

fn find_node_with_min_distance(distances: &DistanceMap, set: &HashSet<Node>) -> Node {
    set.iter()
        .min_by_key(|node| distances[*node])
        .unwrap()
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid() {
        let input = "abc\ndef\nghi\n";

        assert_eq!(
            Graph::from(input).nodes,
            HashMap::from([
                (Node(0, 0), 'a'),
                (Node(0, 1), 'b'),
                (Node(0, 2), 'c'),
                (Node(1, 0), 'd'),
                (Node(1, 1), 'e'),
                (Node(1, 2), 'f'),
                (Node(2, 0), 'g'),
                (Node(2, 1), 'h'),
                (Node(2, 2), 'i'),
            ])
        );
    }

    #[test]
    fn test_find_node() {
        let graph = Graph::from("abc\ndef\nEhi\n");

        assert_eq!(graph.find_node('E').unwrap(), Node(2, 0));
        assert_eq!(graph.find_node('a').unwrap(), Node(0, 0));
    }

    #[test]
    fn test_chars_are_connectable() {
        assert!(chars_are_connectable(&'a', &'a'));
        assert!(chars_are_connectable(&'a', &'b'));
        assert!(chars_are_connectable(&'b', &'a'));
        assert!(!chars_are_connectable(&'a', &'c'));
        assert!(chars_are_connectable(&'c', &'a'));
    }

    #[test]
    fn test_find_position_with_min_distance() {
        let distances = HashMap::from([
            (Node(0, 0), Finite(1)),
            (Node(0, 1), Finite(0)),
            (Node(1, 0), Finite(1)),
            (Node(1, 1), Finite(2)),
        ]);

        let set = HashSet::from([Node(0, 0), Node(0, 1), Node(1, 0)]);
        assert_eq!(find_node_with_min_distance(&distances, &set), Node(0, 1));
    }

    #[test]
    fn test_distance_between_nodes() {
        let graph = Graph::from("abc\ndef\ndhi\n");

        assert_eq!(graph.distance_between_nodes(Node(0, 0), Node(0, 1)), 1);
        assert_eq!(graph.distance_between_nodes(Node(1, 1), Node(1, 0)), 1);
        assert_eq!(graph.distance_between_nodes(Node(1, 0), Node(2, 0)), 2);
    }

    #[test]
    fn test_connected_nodes() {
        let graph = Graph::from(
            r#"
        abc
        def
        dfi
        "#,
        );

        assert_eq!(graph.neighbors(Node(0, 0)), vec![Node(0, 1)]);

        let mut cns = graph.neighbors(Node(0, 1));
        cns.sort();
        assert_eq!(cns, vec![Node(0, 0), Node(0, 2)]);

        let mut cns = graph.neighbors(Node(1, 0));
        cns.sort();
        assert_eq!(cns, vec![Node(0, 0), Node(1, 1), Node(2, 0)]);

        let mut cns = graph.neighbors(Node(1, 1));
        cns.sort();
        assert_eq!(cns, vec![Node(0, 1), Node(1, 0), Node(1, 2), Node(2, 1)]);
    }
}
