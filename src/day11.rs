use std::str::Lines;

const ROUNDS: u16 = 10000;

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Sum(u64),
    Product(u64),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible_by: u64,
    monkey_index_if_true: usize,
    monkey_index_if_false: usize,
    inspected_items: u64,
}

impl Monkey {
    pub fn new(input: &str) -> Monkey {
        let mut lines = input.trim().lines();

        let (_, starting_items) = lines.next().unwrap().split_once(":").unwrap();
        let items = starting_items
            .split(",")
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect();

        let (_, operation) = lines.next().unwrap().split_once(":").unwrap();
        let operation = Self::parse_operation(operation.trim());

        let (divisible_by, monkey_index_if_true, monkey_index_if_false) = Self::parse_action(lines);

        Monkey {
            items,
            operation,
            divisible_by,
            monkey_index_if_true,
            monkey_index_if_false,
            inspected_items: 0,
        }
    }

    fn parse_operation(op: &str) -> Operation {
        let (_, operation) = op.split_once("=").unwrap();
        let parts = operation.trim().split(" ").collect::<Vec<&str>>();

        match (parts[0], parts[1], parts[2]) {
            ("old", "*", "old") => Operation::Square,
            ("old", "+", value) => Operation::Sum(value.parse::<u64>().unwrap()),
            ("old", "*", value) => Operation::Product(value.parse::<u64>().unwrap()),
            _ => panic!("Unknown operation"),
        }
    }

    fn parse_action(mut lines: Lines) -> (u64, usize, usize) {
        let (_, divisible_by) = lines.next().unwrap().split_once("divisible by").unwrap();
        let (_, monkey_index_if_true) = lines.next().unwrap().split_once("monkey").unwrap();
        let (_, monkey_index_if_false) = lines.next().unwrap().split_once("monkey").unwrap();

        (
            divisible_by.trim().parse::<u64>().unwrap(),
            monkey_index_if_true.trim().parse::<usize>().unwrap(),
            monkey_index_if_false.trim().parse::<usize>().unwrap(),
        )
    }
}

#[cfg(test)]
mod monkey_tests {
    use super::*;

    #[test]
    fn test_new_monkey() {
        let input = r#"
            Starting items: 79, 98
            Operation: new = old * 19
            Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
        "#;

        let monkey = Monkey::new(input);

        assert_eq!(monkey.items, vec![79, 98]);
        assert_eq!(monkey.operation, Operation::Product(19));
        assert_eq!(monkey.divisible_by, 23);
        assert_eq!(monkey.monkey_index_if_true, 2);
        assert_eq!(monkey.monkey_index_if_false, 3);
        assert_eq!(monkey.inspected_items, 0);
    }
}

pub fn run(input: &str) {
    let mut monkeys = input
        .split("\n\n")
        .map(parse_monkey)
        .collect::<Vec<Monkey>>();

    let number_space = monkeys.iter().map(|x| x.divisible_by).product::<u64>();

    for _round in 1..=ROUNDS {
        for monkey_index in 0..monkeys.len() {
            monkey_run(&mut monkeys, monkey_index, number_space);
        }
    }

    let mut inspected_items = monkeys
        .iter()
        .map(|x| x.inspected_items)
        .collect::<Vec<u64>>();

    inspected_items.sort();
    inspected_items.reverse();
    println!(
        "Monkey business: {}",
        inspected_items[0] * inspected_items[1]
    );
}

fn parse_monkey(input: &str) -> Monkey {
    let (_, rest) = input.split_once("\n").unwrap();
    Monkey::new(rest)
}

fn monkey_run(monkeys: &mut Vec<Monkey>, index: usize, number_space: u64) {
    let items = monkeys[index].items.clone();
    let operation = monkeys[index].operation.clone();
    let divisible_by = monkeys[index].divisible_by.clone();
    let monkey_index_if_true = monkeys[index].monkey_index_if_true.clone();
    let monkey_index_if_false = monkeys[index].monkey_index_if_false.clone();

    for item in &items {
        let new_item = match operation {
            Operation::Sum(value) => item + value,
            Operation::Product(value) => item * value,
            Operation::Square => item * item,
        };

        let new_item = new_item % number_space;

        let throw_index = if new_item % divisible_by == 0 {
            monkey_index_if_true
        } else {
            monkey_index_if_false
        };

        monkeys[throw_index].items.push(new_item);
    }

    monkeys[index].items = Vec::new();
    monkeys[index].inspected_items += items.len() as u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monkey_run() {
        let mut monkeys = vec![
            Monkey {
                items: vec![5, 2],
                operation: Operation::Product(3),
                divisible_by: 2,
                monkey_index_if_true: 1,
                monkey_index_if_false: 2,
                inspected_items: 0,
            },
            Monkey {
                items: vec![1, 2],
                operation: Operation::Product(9),
                divisible_by: 2,
                monkey_index_if_true: 2,
                monkey_index_if_false: 0,
                inspected_items: 0,
            },
            Monkey {
                items: vec![],
                operation: Operation::Square,
                divisible_by: 10,
                monkey_index_if_true: 0,
                monkey_index_if_false: 1,
                inspected_items: 0,
            },
        ];

        monkey_run(&mut monkeys, 1, 1000000);

        assert_eq!(monkeys[0].items, vec![5, 2, 9]);
        assert_eq!(monkeys[1].items, vec![]);
        assert_eq!(monkeys[1].inspected_items, 2);
        assert_eq!(monkeys[2].items, vec![18]);
    }
}
