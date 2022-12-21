use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn parse(string: &str) -> Option<Self> {
        match string {
            "+" => Some(Self::Add),
            "-" => Some(Self::Sub),
            "*" => Some(Self::Mul),
            "/" => Some(Self::Div),
            _ => None,
        }
    }

    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
        }
    }
}

#[cfg(test)]
mod operation_test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Operation::parse("+"), Some(Operation::Add));
        assert_eq!(Operation::parse("-"), Some(Operation::Sub));
        assert_eq!(Operation::parse("*"), Some(Operation::Mul));
        assert_eq!(Operation::parse("/"), Some(Operation::Div));
        assert_eq!(Operation::parse("a"), None);
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Monkey {
    YellingMonkey(String, u64),
    MathMonkey(String, Operation, String, String),
}

#[derive(Debug, PartialEq)]
struct Node {
    monkey: Monkey,
}

use Monkey::*;

impl FromStr for Node {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (name, rest) = string.split_once(":").unwrap();
        let rest = rest.trim();

        match rest.parse::<u64>() {
            Ok(number) => Ok(Node {
                monkey: YellingMonkey(name.to_string(), number),
            }),
            _ => {
                let (monkey1, rest) = rest.split_at(4);
                let (operation, monkey2) = rest.trim().split_once(" ").unwrap();

                Ok(Node {
                    monkey: MathMonkey(
                        name.to_string(),
                        Operation::parse(operation).unwrap(),
                        monkey1.to_string(),
                        monkey2.to_string(),
                    ),
                })
            }
        }
    }
}

#[cfg(test)]
mod node_test {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            Node::from_str("aaaa: 1").unwrap(),
            Node {
                monkey: YellingMonkey("aaaa".to_string(), 1)
            }
        );
        assert_eq!(
            Node::from_str("cccc: aaaa + bbbb").unwrap(),
            Node {
                monkey: MathMonkey(
                    "cccc".to_string(),
                    Operation::Add,
                    "aaaa".to_string(),
                    "bbbb".to_string()
                )
            }
        );
    }
}

pub fn run(input: &str) {
    let monkeys = input
        .lines()
        .map(|line| line.parse::<Node>().unwrap())
        .collect::<Vec<Node>>();

    let mut computed_monkeys = HashMap::new();
    let mut loop_index = 0;

    loop {
        loop_index += 1;

        if computed_monkeys.len() == monkeys.len() {
            break;
        } else {
            println!(
                "{} uncomputed monkeys left in loop {}",
                monkeys.len() - computed_monkeys.len(),
                loop_index
            );
        }

        for node in &monkeys {
            match node.monkey.clone() {
                YellingMonkey(name, number) => {
                    computed_monkeys.insert(name, number);
                }
                MathMonkey(name, operation, monkey1, monkey2) => {
                    match (
                        computed_monkeys.get(&monkey1),
                        computed_monkeys.get(&monkey2),
                    ) {
                        (Some(number1), Some(number2)) => {
                            let result = operation.apply(*number1, *number2);
                            computed_monkeys.insert(name, result);
                        }
                        _ => continue,
                    }
                }
            }
        }
    }

    println!("The 'root' monkey yells: {}", computed_monkeys["root"]);
}
