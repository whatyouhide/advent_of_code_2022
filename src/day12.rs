use infinitable::*;

use std::{
    collections::{HashMap, HashSet},
    i32,
};

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, PartialOrd, Ord)]
struct Node(i32, i32);

type DistanceMap = HashMap<Node, Infinitable<i32>>;
type Graph = HashMap<Node, char>;

pub fn run(input: &str) {
    let mut graph = parse_graph(input);
    let start_node = find_node(&graph, 'S');
    let end_node = find_node(&graph, 'E');

    graph.insert(start_node, 'a');
    graph.insert(end_node, 'z');

    let (start_node, hops) = dijkstra(&mut graph, start_node, end_node);

    println!("Hops from starting position {:?}: {:?}", start_node, hops);
}

fn dijkstra(graph: &mut Graph, start_node: Node, end_node: Node) -> (Node, u32) {
    let mut unvisited_set: HashSet<Node> = HashSet::new();
    let mut distances: DistanceMap = HashMap::new();
    let mut prev: HashMap<Node, Option<Node>> = HashMap::new();

    for node in graph.keys() {
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

        for v in connected_nodes(u, &graph)
            .iter()
            .filter(|node| unvisited_set.contains(*node))
        {
            let alt =
                distances[&u].finite().unwrap() + distance_between_nodes(&graph, u, v.clone());

            if Finite(alt) < distances[v] {
                distances.insert(v.clone(), Finite(alt));
                prev.insert(v.clone(), Some(u));
            }
        }
    }

    let mut s = vec![];
    let mut u = Some(end_node);

    while let Some(u1) = u {
        s.insert(0, u1);
        u = prev[&u1];
    }

    (start_node, (s.len() - 1) as u32)
}

fn connected_nodes(node: Node, graph: &Graph) -> Vec<Node> {
    let mut nodes = vec![];
    let char_at_node = graph[&node];

    let north_node = Node(node.0 - 1, node.1);
    let south_node = Node(node.0 + 1, node.1);
    let east_node = Node(node.0, node.1 + 1);
    let west_node = Node(node.0, node.1 - 1);

    if let Some(char) = graph.get(&north_node) {
        if chars_are_connectable(&char_at_node, &char) {
            nodes.push(north_node);
        }
    }
    if let Some(char) = graph.get(&south_node) {
        if chars_are_connectable(&char_at_node, &char) {
            nodes.push(south_node);
        }
    }
    if let Some(char) = graph.get(&east_node) {
        if chars_are_connectable(&char_at_node, &char) {
            nodes.push(east_node);
        }
    }
    if let Some(char) = graph.get(&west_node) {
        if chars_are_connectable(&char_at_node, &char) {
            nodes.push(west_node);
        }
    }

    nodes
}

fn chars_are_connectable(char1: &char, char2: &char) -> bool {
    (*char2 as i32) <= (*char1 as i32) + 1
}

fn parse_graph(input: &str) -> Graph {
    let mut graph = HashMap::new();

    for (row_index, row) in input.trim().lines().enumerate() {
        for (column_index, char) in row.trim().chars().enumerate() {
            graph.insert(Node(row_index as i32, column_index as i32), char);
        }
    }

    graph
}

fn find_node(graph: &Graph, target: char) -> Node {
    for (node, char) in graph {
        if *char == target {
            return node.clone();
        }
    }

    panic!("Destination not found");
}

fn find_node_with_min_distance(distances: &DistanceMap, set: &HashSet<Node>) -> Node {
    set.iter()
        .min_by_key(|node| distances[*node])
        .unwrap()
        .clone()
}

fn distance_between_nodes(graph: &Graph, node1: Node, node2: Node) -> i32 {
    let node1_val = graph[&node1] as i32;
    let node2_val = graph[&node2] as i32;

    if node2_val == node1_val {
        2
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid() {
        let input = "abc\ndef\nghi\n";

        assert_eq!(
            parse_graph(input),
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
        let graph = parse_graph("abc\ndef\nEhi\n");

        assert_eq!(find_node(&graph, 'E'), Node(2, 0));
        assert_eq!(find_node(&graph, 'a'), Node(0, 0));
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

        let set = HashSet::from([Node(0, 0), Node(1, 0)]);
        assert_eq!(find_node_with_min_distance(&distances, &set), Node(0, 0));
    }

    #[test]
    fn test_distance_between_nodes() {
        let graph = parse_graph("abc\ndef\ndhi\n");

        assert_eq!(distance_between_nodes(&graph, Node(0, 0), Node(0, 1)), 1);
        assert_eq!(distance_between_nodes(&graph, Node(1, 1), Node(1, 0)), 1);
        assert_eq!(distance_between_nodes(&graph, Node(1, 0), Node(2, 0)), 0);
    }

    #[test]
    fn test_connected_nodes() {
        let graph = parse_graph(
            r#"
        abc
        def
        dfi
        "#,
        );

        assert_eq!(connected_nodes(Node(0, 0), &graph), vec![Node(0, 1)]);

        let mut cns = connected_nodes(Node(0, 1), &graph);
        cns.sort();
        assert_eq!(cns, vec![Node(0, 0), Node(0, 2)]);

        let mut cns = connected_nodes(Node(1, 0), &graph);
        cns.sort();
        assert_eq!(cns, vec![Node(0, 0), Node(1, 1), Node(2, 0)]);

        let mut cns = connected_nodes(Node(1, 1), &graph);
        cns.sort();
        assert_eq!(cns, vec![Node(0, 1), Node(1, 0), Node(1, 2), Node(2, 1)]);
    }
}
