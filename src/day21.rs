use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

    fn apply(&self, a: isize, b: isize) -> isize {
        match self {
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
        }
    }

    fn inverse(&self) -> Self {
        match self {
            Self::Add => Self::Sub,
            Self::Sub => Self::Add,
            Self::Mul => Self::Div,
            Self::Div => Self::Mul,
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
    YellingMonkey(String, isize),
    MathMonkey(String, Operation, String, String),
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (name, rest) = string.split_once(":").unwrap();
        let rest = rest.trim();

        match rest.parse::<isize>() {
            Ok(number) => Ok(Self::YellingMonkey(name.to_string(), number)),
            _ => {
                let (monkey1, rest) = rest.split_at(4);
                let (operation, monkey2) = rest.trim().split_once(" ").unwrap();

                Ok(Self::MathMonkey(
                    name.to_string(),
                    Operation::parse(operation).unwrap(),
                    monkey1.to_string(),
                    monkey2.to_string(),
                ))
            }
        }
    }
}

impl Monkey {
    fn name(&self) -> String {
        match self {
            Self::YellingMonkey(name, _) => name.to_string(),
            Self::MathMonkey(name, _, _, _) => name.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum ASTNode {
    Human,
    Number(isize),
    Operation(Operation, Box<ASTNode>, Box<ASTNode>),
}

impl ASTNode {
    fn from_monkey(monkey: &str, monkeys: &HashMap<String, Monkey>) -> Self {
        match &monkeys[monkey] {
            Monkey::YellingMonkey(name, _) if *name == "humn" => Self::Human,
            Monkey::YellingMonkey(_, number) => Self::Number(*number),
            Monkey::MathMonkey(_, operation, left, right) => Self::Operation(
                operation.clone(),
                Box::new(Self::from_monkey(monkeys[left].name().as_str(), monkeys)),
                Box::new(Self::from_monkey(monkeys[right].name().as_str(), monkeys)),
            ),
        }
    }

    fn simplify(&self) -> Self {
        match self {
            Self::Human => Self::Human,
            Self::Number(number) => Self::Number(*number),
            Self::Operation(operation, left, right) => {
                let left = left.simplify();
                let right = right.simplify();

                match (left, right) {
                    (Self::Number(left), Self::Number(right)) => {
                        Self::Number(operation.apply(left, right))
                    }
                    (left, right) => {
                        Self::Operation(operation.clone(), Box::new(left), Box::new(right))
                    }
                }
            }
        }
    }

    // left is always an expression with "humn" in it, right is always a number.
    fn simplify_equation(left: &Self, right: &Self) -> (Self, Self) {
        match (left, right) {
            // We solved it!
            (Self::Human, Self::Number(_)) => (left.clone(), right.clone()),

            (Self::Number(_), _) => panic!("left is a number, but should be humn"),

            // If there's an operation on the left, we simplify it.
            (Self::Operation(op, x, y), Self::Number(right_number)) => {
                // The equation is number • y = right_number, so we can simplify it as
                // y = right_number ¬ number, where ¬ is the opposite of •.
                match (x.as_ref(), y.as_ref()) {
                    (Self::Number(number), _) => {
                        let (simplified_left, op) = if *op == Operation::Sub {
                            (
                                Box::new(Self::Operation(
                                    Operation::Mul,
                                    Box::new(Self::Number(-1)),
                                    y.clone(),
                                )),
                                Operation::Add,
                            )
                        } else {
                            (y.clone(), *op)
                        };

                        let simplified_right =
                            Self::Number(op.inverse().apply(*right_number, *number));

                        Self::simplify_equation(&simplified_left, &simplified_right)
                    }

                    // The equation is x • number = right_number, so we can simplify it as
                    // x = right_number ¬ number, where ¬ is the opposite of •.
                    (_, Self::Number(number)) => {
                        let simplified_left = x.clone();
                        let simplified_right =
                            Self::Number(op.inverse().apply(*right_number, *number));

                        Self::simplify_equation(&simplified_left, &simplified_right)
                    }

                    _ => {
                        panic!("found an operation on the left where neither operand is a number")
                    }
                }
            }

            _ => todo!(),
        }
    }
}

impl Display for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Human => write!(f, "humn"),
            Self::Number(number) => write!(f, "{}", number),
            Self::Operation(operation, left, right) => write!(
                f,
                "({} {} {})",
                *left,
                match operation {
                    Operation::Add => "+",
                    Operation::Sub => "-",
                    Operation::Mul => "*",
                    Operation::Div => "/",
                },
                *right
            ),
        }
    }
}

pub fn run(input: &str) {
    let monkeys = input
        .lines()
        .map(|line| line.parse::<Monkey>().unwrap())
        .map(|monkey| (monkey.name(), monkey.clone()))
        .collect::<HashMap<String, Monkey>>();

    let (left_ast, right_ast) = match &monkeys["root"] {
        Monkey::MathMonkey(_, _, left, right) => (
            ASTNode::from_monkey(&left, &monkeys),
            ASTNode::from_monkey(&right, &monkeys),
        ),
        _ => panic!("root is not a math monkey"),
    };

    let simplified = (left_ast.simplify(), right_ast.simplify());
    println!("Simplified: {} = {}", simplified.0, simplified.1);

    let reduced = ASTNode::simplify_equation(&simplified.0, &simplified.1);
    println!("Reduced: {} = {}", reduced.0, reduced.1);
}
