use std::collections::HashMap;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use ansi_term::Style;

type Position = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn to_password(&self) -> usize {
        match self {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::East => write!(f, ">"),
            Direction::South => write!(f, "v"),
            Direction::West => write!(f, "<"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    TurnLeft,
    TurnRight,
    Move(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}

impl Cell {
    fn from_char(c: char) -> Option<Cell> {
        match c {
            '.' => Some(Cell::Empty),
            '#' => Some(Cell::Wall),
            _ => None,
        }
    }
}

struct Board {
    map: Vec<Vec<Option<Cell>>>,
    current_position: Position,
    current_direction: Direction,
    visited_positions: HashMap<Position, Direction>,
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let line_count = s.lines().count();
        let col_count = s.lines().map(str::len).max().unwrap();

        let mut current_position = None;
        let mut map = Vec::with_capacity(line_count);

        for row_index in 0..line_count {
            let line = s.lines().nth(row_index).unwrap();
            let mut row = Vec::with_capacity(col_count);

            for col_index in 0..col_count {
                match line.chars().nth(col_index).and_then(Cell::from_char) {
                    cell @ Some(_) => {
                        if current_position.is_none() {
                            current_position = Some((row_index, col_index));
                        }

                        row.push(cell);
                    }
                    cell @ None => row.push(cell),
                }
            }

            map.push(row);
        }

        Ok(Self {
            map,
            current_position: current_position.unwrap(),
            current_direction: Direction::East,
            visited_positions: HashMap::new(),
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} by {} board:", self.map.len(), self.map[0].len())?;

        for (row_index, row) in self.map.iter().enumerate() {
            for (cell_index, cell) in row.iter().enumerate() {
                match cell {
                    Some(_) if self.current_position == (row_index, cell_index) => {
                        let style = Style::new().bold().on(ansi_term::Color::Green);
                        write!(f, "{}", style.paint(format!("{}", self.current_direction)))?
                    }
                    Some(_)
                        if self
                            .visited_positions
                            .contains_key(&(row_index, cell_index)) =>
                    {
                        write!(f, "{}", self.visited_positions[&(row_index, cell_index)])?
                    }
                    Some(Cell::Empty) => write!(f, ".")?,
                    Some(Cell::Wall) => write!(f, "#")?,
                    None => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Board {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::TurnLeft => self.current_direction = self.current_direction.turn_left(),
            Instruction::TurnRight => self.current_direction = self.current_direction.turn_right(),
            Instruction::Move(n) => self.move_forward(*n),
        }
    }

    fn move_forward(&mut self, count: usize) {
        for _ in 0..count {
            let (row_index, col_index) = self.current_position;

            let maybe_new_position = match self.current_direction {
                Direction::North => (row_index.checked_sub(1), Some(col_index)),
                Direction::East => (Some(row_index), col_index.checked_add(1)),
                Direction::South => (row_index.checked_add(1), Some(col_index)),
                Direction::West => (Some(row_index), col_index.checked_sub(1)),
            };

            let new_position = maybe_new_position
                .0
                .and_then(|row| maybe_new_position.1.map(|col| (row, col)));

            let new_cell =
                new_position.and_then(|(row, col)| self.map.get(row).and_then(|row| row.get(col)));

            match new_cell {
                Some(None) | None => {
                    let new_position = self.wrap(&self.current_position);
                    self.visited_positions
                        .insert(self.current_position, self.current_direction);
                    self.current_position = new_position;
                }
                Some(Some(Cell::Wall)) => {
                    self.visited_positions
                        .insert(self.current_position, self.current_direction);
                    break;
                }

                Some(Some(Cell::Empty)) => {
                    self.visited_positions
                        .insert(self.current_position, self.current_direction);
                    self.current_position = new_position.unwrap();
                }
            }
        }
    }

    fn wrap(&self, position: &Position) -> Position {
        let (row_index, col_index) = position;

        match self.current_direction {
            Direction::East => {
                let row = &self.map[*row_index];
                let new_col_index = row.iter().position(|cell| cell.is_some()).unwrap();

                match self.map[*row_index][new_col_index] {
                    Some(Cell::Empty) => (*row_index, new_col_index),
                    Some(Cell::Wall) => position.clone(),
                    _ => unreachable!(),
                }
            }
            Direction::West => {
                let row = &self.map[*row_index];
                let new_col_index = row.iter().rposition(|cell| cell.is_some()).unwrap();
                match self.map[*row_index][new_col_index] {
                    Some(Cell::Empty) => (*row_index, new_col_index),
                    Some(Cell::Wall) => position.clone(),
                    _ => unreachable!(),
                }
            }
            Direction::North => {
                let new_row_index = self
                    .map
                    .iter()
                    .rposition(|row| row[*col_index].is_some())
                    .unwrap();
                match self.map[new_row_index][*col_index] {
                    Some(Cell::Empty) => (new_row_index, *col_index),
                    Some(Cell::Wall) => position.clone(),
                    _ => unreachable!(),
                }
            }
            Direction::South => {
                let new_row_index = self
                    .map
                    .iter()
                    .position(|row| row[*col_index].is_some())
                    .unwrap();
                match self.map[new_row_index][*col_index] {
                    Some(Cell::Empty) => (new_row_index, *col_index),
                    Some(Cell::Wall) => position.clone(),
                    _ => unreachable!(),
                }
            }
        }
    }

    fn password(&self) -> usize {
        1000 * (self.current_position.0 + 1)
            + 4 * (self.current_position.1 + 1)
            + self.current_direction.to_password()
    }
}

pub fn run(input: &str) {
    let (map_string, instruction_string) = input.split_once("\n\n").unwrap();

    let mut board = map_string.parse::<Board>().unwrap();
    let instructions = parse_instructions(instruction_string.trim()).unwrap();

    println!("Board: {}", board);

    assert_eq!(board.map.len(), 200);
    for row in &board.map {
        assert_eq!(row.len(), 150);
    }

    for instruction in &instructions {
        board.apply_instruction(instruction);
    }

    println!("Final password: {}", board.password());
}

fn parse_instructions(s: &str) -> Result<Vec<Instruction>, ParseIntError> {
    let mut left = s;
    let mut instructions = Vec::new();

    loop {
        match left.get(0..1) {
            Some("L") => {
                instructions.push(Instruction::TurnLeft);
                left = &left[1..];
            }
            Some("R") => {
                instructions.push(Instruction::TurnRight);
                left = &left[1..];
            }
            None => break,
            _ => {
                let (int, rest) = parse_next_int(left)?;
                instructions.push(Instruction::Move(int as usize));
                left = rest;
            }
        }
    }

    Ok(instructions)
}

fn parse_next_int(s: &str) -> Result<(i32, &str), ParseIntError> {
    let (int_str, rest) = s.split_at(s.find(|c: char| !c.is_numeric()).unwrap_or(s.len()));
    let int = int_str.parse::<i32>()?;

    // Return the integer and the slice to the rest of the string
    Ok((int, rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        let input = "10R5L5R10L4R5L5";
        let instructions = parse_instructions(input).unwrap();
        assert_eq!(
            instructions,
            vec![
                Instruction::Move(10),
                Instruction::TurnRight,
                Instruction::Move(5),
                Instruction::TurnLeft,
                Instruction::Move(5),
                Instruction::TurnRight,
                Instruction::Move(10),
                Instruction::TurnLeft,
                Instruction::Move(4),
                Instruction::TurnRight,
                Instruction::Move(5),
                Instruction::TurnLeft,
                Instruction::Move(5),
            ]
        );
    }

    #[test]
    fn test_parse_next_int() {
        let input = "123,456";
        let (int, rest) = parse_next_int(input).unwrap();
        assert_eq!(int, 123);
        assert_eq!(rest, ",456");
    }
}
