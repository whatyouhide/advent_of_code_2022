#[derive(Debug, PartialEq)]
struct Move {
    start_stack: u16,
    end_stack: u16,
    crates_to_move: u16,
}

#[derive(Debug)]
struct Stack {
    // Crates are ordered from bottm to top (that is, the top crate is the last crate in the vector)
    crates: Vec<char>,
}

pub fn run() {
    let input = include_str!("../inputs/day5.txt");

    let mut world = Vec::new();

    for column_index in 0..count_stacks(input) {
        let stack = parse_stack(input, column_index as usize);
        world.push(stack);
    }

    assert_eq!(world.len(), 9);
    assert_eq!(world[0].crates, vec!['W', 'D', 'G', 'B', 'H', 'R', 'V']);

    let moves: Vec<Move> = input
        .lines()
        .filter(|line| line.starts_with("move"))
        .map(parse_move)
        .collect();

    assert_eq!(
        moves[0],
        Move {
            start_stack: 2,
            end_stack: 7,
            crates_to_move: 2
        }
    );

    for move_ in moves {
        move_crates_9001(&mut world, move_);
    }

    let top_chars_iter = world.iter().map(|stack| stack.crates.last().unwrap());
    println!("{}", String::from_iter(top_chars_iter));
}

fn move_crates_9001(world: &mut Vec<Stack>, move_: Move) {
    let start_stack = &mut world[(move_.start_stack - 1) as usize];
    let to_move = pop_many(&mut start_stack.crates, move_.crates_to_move);
    let end_stack = &mut world[(move_.end_stack - 1) as usize];
    end_stack.crates.extend(to_move);
}

fn pop_many<T>(vec: &mut Vec<T>, count: u16) -> Vec<T> {
    let mut popped = Vec::new();
    for _ in 0..count {
        popped.insert(0, vec.pop().unwrap());
    }
    popped
}

fn move_crates(world: &mut Vec<Stack>, move_: Move) {
    for _ in 0..move_.crates_to_move {
        let start_stack = &mut world[(move_.start_stack - 1) as usize];
        let crate_to_move = start_stack.crates.pop().unwrap();
        let end_stack = &mut world[(move_.end_stack - 1) as usize];
        end_stack.crates.push(crate_to_move);
    }
}

fn count_stacks(input: &str) -> u16 {
    let indexes_line = input
        .lines()
        .find(|line| line.trim().starts_with("1"))
        .unwrap();

    indexes_line
        .split_ascii_whitespace()
        .count()
        .try_into()
        .unwrap()
}

fn parse_stack(input: &str, column_index: usize) -> Stack {
    let mut crates: Vec<char> = Vec::new();

    for line in input.lines().take_while(|line| !line.trim().is_empty()) {
        let range = (column_index * 4)..(column_index * 4 + 2);
        let cleaned_line = line[range].trim();

        if cleaned_line.is_empty() {
            continue;
        } else if cleaned_line.starts_with("[") {
            let char = cleaned_line
                .trim_start_matches("[")
                .trim_end_matches("]")
                .chars()
                .next()
                .unwrap();

            crates.push(char);
        }
    }

    crates.reverse();
    Stack { crates }
}

fn parse_move(string: &str) -> Move {
    let words: Vec<&str> = string.split_ascii_whitespace().collect();
    let crates_to_move = words[1].parse::<u16>().unwrap();
    let start_stack = words[3].parse::<u16>().unwrap();
    let end_stack = words[5].parse::<u16>().unwrap();

    Move {
        start_stack,
        end_stack,
        crates_to_move,
    }
}
