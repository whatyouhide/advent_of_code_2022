use Rock::*;

type Position = (i128, i128);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Rock {
    MinusSign,
    PlusSign,
    ReverseL,
    VerticalLine,
    Square,
}

impl std::str::FromStr for Rock {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s == "####" {
            Ok(MinusSign)
        } else if s == ".#.\n###\n.#." {
            Ok(PlusSign)
        } else if s == "..#\n..#\n###" {
            Ok(ReverseL)
        } else if s == "#\n#\n#\n#" {
            Ok(VerticalLine)
        } else if s == "##\n##" {
            Ok(Square)
        } else {
            Err(())
        }
    }
}

impl Rock {
    pub fn to_positions(&self) -> Vec<Position> {
        match self {
            MinusSign => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            VerticalLine => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            PlusSign => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            ReverseL => vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            Square => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Unknown direction: {}", c),
        }
    }
}

struct Chamber {
    rows: Vec<Vec<char>>,
    tallest_row: i128,
    row_offset: usize,
}

impl Chamber {
    pub fn new() -> Self {
        Self {
            rows: vec![],
            tallest_row: -1,
            row_offset: 0,
        }
    }

    pub fn add_rock(&mut self, rock: Rock, jet_pattern: &mut impl Iterator<Item = Direction>) {
        let positions = rock.to_positions();

        let height = positions.iter().map(|(row, _)| row).max().unwrap() + 1;

        // Add rows if necessary.
        for _ in 0..height + 3 {
            self.add_row();
        }

        let mut positions = positions
            .iter()
            .map(|(row, column)| (*row as i128 + self.tallest_row + 4, column + 2))
            .collect::<Vec<_>>();

        loop {
            let dir = jet_pattern.next().unwrap();

            // Pushing.
            let mut new_positions = match dir {
                Direction::Left => {
                    let new_positions =
                        positions.iter().map(|(row, col)| (*row, col - 1)).collect();

                    if self.can_blow(&new_positions) {
                        new_positions
                    } else {
                        // Don't blow.
                        positions.clone()
                    }
                }
                Direction::Right => {
                    let new_positions =
                        positions.iter().map(|(row, col)| (*row, col + 1)).collect();

                    if self.can_blow(&new_positions) {
                        new_positions
                    } else {
                        positions.clone()
                    }
                }
            };

            // Falling.
            // If the rock would overlap on the bottom by falling, it sets instead.
            if self.should_set(&new_positions) {
                for (row, column) in new_positions.iter() {
                    self.rows[*row as usize][*column as usize] = '#';
                }

                self.rows = self
                    .rows
                    .iter()
                    .filter(|row| row.iter().any(|c| *c == '#'))
                    .cloned()
                    .collect();

                self.compress();
                self.update_tallest_row();

                return;
            } else {
                // Clean the current positions.
                for (row, column) in new_positions.iter() {
                    self.rows[*row as usize][*column as usize] = '.';
                }

                // Fall by one row.
                new_positions = new_positions
                    .iter()
                    .map(|(row, col)| (row - 1, *col))
                    .collect::<Vec<Position>>();
            }

            positions = new_positions;
        }
    }

    fn update_tallest_row(&mut self) {
        self.tallest_row = self
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().any(|c| *c == '#'))
            .map(|(index, _)| index)
            .max()
            .map(|row| row as i128)
            .unwrap_or(-1);
    }

    fn can_blow(&self, positions: &Vec<Position>) -> bool {
        for (row, column) in positions.iter() {
            if *column < 0 || *column > 6 || self.rows[*row as usize][*column as usize] == '#' {
                return false;
            }
        }

        true
    }

    fn should_set(&self, positions: &Vec<Position>) -> bool {
        for (row, column) in positions.iter() {
            let row = row - 1;

            if row < 0 || self.rows[row as usize][*column as usize] == '#' {
                return true;
            }
        }

        false
    }

    fn add_row(&mut self) {
        self.rows.push(vec!['.'; 7]);
    }

    fn compress(&mut self) {
        let cutoff = self
            .rows
            .iter()
            .position(|row| row.iter().all(|c| *c == '#'));

        if let Some(cutoff_row) = cutoff {
            self.row_offset += cutoff_row;
            self.rows.drain(0..cutoff_row);
        }
    }

    pub fn tower_height(&self) -> u32 {
        self.rows.len() as u32 + self.row_offset as u32
    }
}

impl std::fmt::Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Print two dummy rows on top of what's already there.
        for row in self.rows.iter().rev() {
            write!(f, "|")?;

            for c in row.iter() {
                write!(f, "{}", c)?;
            }

            write!(f, "|")?;
            writeln!(f)?;
        }

        writeln!(f, "+-------+")?;

        Ok(())
    }
}

pub fn run(input: &str) {
    let rocks = [MinusSign, PlusSign, ReverseL, VerticalLine, Square]
        .iter()
        .cloned()
        .cycle();

    let mut jet_pattern = input.trim().chars().map(Direction::from_char).cycle();

    let mut chamber = Chamber::new();

    for rock in rocks.take(2022) {
        chamber.add_rock(rock, &mut jet_pattern);
    }

    println!(
        "The tower is {} units tall (finished with {} rows in memory)",
        chamber.tower_height(),
        chamber.rows.len()
    );
}
