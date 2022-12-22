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
    Space,
    Wall,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Cell::Space,
            '#' => Cell::Wall,
            ' ' => Cell::Empty,
            _ => unreachable!(),
        }
    }

    fn is_some(&self) -> bool {
        match self {
            Cell::Empty => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod cell_tests {
    use super::*;

    #[test]
    fn from_char() {
        assert_eq!(Cell::from_char('.'), Cell::Space);
        assert_eq!(Cell::from_char('#'), Cell::Wall);
        assert_eq!(Cell::from_char(' '), Cell::Empty);
    }

    #[test]
    fn is_some() {
        assert_eq!(Cell::Space.is_some(), true);
        assert_eq!(Cell::Wall.is_some(), true);
        assert_eq!(Cell::Empty.is_some(), false);
    }
}

struct Board {
    map: Vec<Vec<Cell>>,
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
                let cell = line
                    .chars()
                    .nth(col_index)
                    .map_or(Cell::Empty, Cell::from_char);

                if cell == Cell::Space && current_position.is_none() {
                    current_position = Some((row_index, col_index));
                }

                row.push(cell);
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
                    _ if self.current_position == (row_index, cell_index) => {
                        let style = Style::new().bold().on(ansi_term::Color::Green);
                        write!(f, "{}", style.paint(format!("{}", self.current_direction)))?
                    }
                    _ if self
                        .visited_positions
                        .contains_key(&(row_index, cell_index)) =>
                    {
                        write!(f, "{}", self.visited_positions[&(row_index, cell_index)])?
                    }
                    Cell::Space => write!(f, ".")?,
                    Cell::Wall => write!(f, "#")?,
                    Cell::Empty => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Board {
    pub fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::TurnLeft => self.current_direction = self.current_direction.turn_left(),
            Instruction::TurnRight => self.current_direction = self.current_direction.turn_right(),
            Instruction::Move(n) => self.move_(*n),
        }
    }

    fn move_(&mut self, count: usize) {
        for _ in 0..count {
            self.move_forward();
        }
    }

    fn move_forward(&mut self) {
        let (row_index, col_index) = self.current_position;

        let new_position = match self.current_direction {
            Direction::East => {
                let row = &self.map[row_index];

                match self.map[row_index].get(col_index + 1) {
                    Some(Cell::Space) => (row_index, col_index + 1),
                    Some(Cell::Wall) => self.current_position.clone(),
                    Some(Cell::Empty) | None => {
                        let new_col_index = row.iter().position(|cell| cell.is_some()).unwrap();

                        match self.map[row_index][new_col_index] {
                            Cell::Space => (row_index, new_col_index),
                            Cell::Wall => self.current_position.clone(),
                            _ => unreachable!(),
                        }
                    }
                }
            }
            Direction::West => {
                let row = &self.map[row_index];

                match col_index
                    .checked_sub(1)
                    .and_then(|i| self.map[row_index].get(i))
                {
                    Some(Cell::Space) => (row_index, col_index - 1),
                    Some(Cell::Wall) => self.current_position.clone(),
                    Some(Cell::Empty) | None => {
                        let new_col_index = row.iter().rposition(|cell| cell.is_some()).unwrap();

                        match self.map[row_index][new_col_index] {
                            Cell::Space => (row_index, new_col_index),
                            Cell::Wall => self.current_position.clone(),
                            _ => unreachable!(),
                        }
                    }
                }
            }
            Direction::North => match row_index
                .checked_sub(1)
                .and_then(|i| self.map.get(i))
                .map(|row| &row[col_index])
            {
                Some(Cell::Space) => (row_index - 1, col_index),
                Some(Cell::Wall) => self.current_position.clone(),
                Some(Cell::Empty) | None => {
                    let new_row_index = self
                        .map
                        .iter()
                        .rposition(|row| row[col_index].is_some())
                        .unwrap();

                    match self.map[new_row_index][col_index] {
                        Cell::Space => (new_row_index, col_index),
                        Cell::Wall => self.current_position.clone(),
                        _ => unreachable!(),
                    }
                }
            },
            Direction::South => match self.map.get(row_index + 1).map(|row| &row[col_index]) {
                Some(Cell::Space) => (row_index + 1, col_index),
                Some(Cell::Wall) => self.current_position.clone(),
                Some(Cell::Empty) | None => {
                    let new_row_index = self
                        .map
                        .iter()
                        .position(|row| row[col_index].is_some())
                        .unwrap();

                    match self.map[new_row_index][col_index] {
                        Cell::Space => (new_row_index, col_index),
                        Cell::Wall => self.current_position.clone(),
                        _ => unreachable!(),
                    }
                }
            },
        };

        self.current_position = new_position;
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
