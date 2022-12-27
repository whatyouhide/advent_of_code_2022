use Rock::*;

type Position = (i32, i32);

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

struct Chamber(Vec<Vec<char>>);

impl Chamber {
    pub fn new() -> Self {
        Chamber(vec![])
    }

    pub fn add_rock(&mut self, rock: Rock, jet_pattern: &mut impl Iterator<Item = Direction>) {
        let positions = rock.to_positions();

        let left_edge = positions.iter().map(|(_, column)| column).min().unwrap();
        let bottom_edge = positions.iter().map(|(row, _)| row).min().unwrap();

        let height = positions.iter().map(|(row, _)| row).max().unwrap() - bottom_edge + 1;

        let max_occupied_row = self
            .0
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().any(|c| *c != '.'))
            .map(|(index, _)| index)
            .max()
            .map(|row| row as i32);

        // Add rows if necessary.
        for _ in 0..height + 3 {
            self.add_row();
        }

        let mut positions = positions
            .iter()
            .map(|(row, column)| {
                (
                    row + max_occupied_row.unwrap_or(-1) + 4,
                    column + left_edge + 2,
                )
            })
            .collect::<Vec<_>>();

        for (row, column) in positions.iter() {
            self.0[*row as usize][*column as usize] = '@';
        }

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

            // Empty the current positions and fill the new positions.
            for (row, column) in positions.iter() {
                self.0[*row as usize][*column as usize] = '.';
            }
            for (row, column) in new_positions.iter() {
                self.0[*row as usize][*column as usize] = '@';
            }

            // Falling.
            // If the rock would overlap on the bottom by falling, it sets instead.
            if self.should_set(&new_positions) {
                for (row, column) in new_positions.iter() {
                    self.0[*row as usize][*column as usize] = '#';
                }

                self.0 = self
                    .0
                    .iter()
                    .filter(|row| row.iter().any(|c| *c == '#'))
                    .cloned()
                    .collect();

                return;
            } else {
                // Clean the current positions.
                for (row, column) in new_positions.iter() {
                    self.0[*row as usize][*column as usize] = '.';
                }

                // Fall by one row.
                new_positions = new_positions
                    .iter()
                    .map(|(row, col)| (row - 1, *col))
                    .collect::<Vec<Position>>();

                for (row, column) in new_positions.iter() {
                    self.0[*row as usize][*column as usize] = '@';
                }
            }

            positions = new_positions;
        }
    }

    fn can_blow(&self, positions: &Vec<Position>) -> bool {
        for (row, column) in positions.iter() {
            if *column < 0 || *column > 6 || self.0[*row as usize][*column as usize] == '#' {
                return false;
            }
        }

        true
    }

    fn should_set(&self, positions: &Vec<Position>) -> bool {
        for (row, column) in positions.iter() {
            let row = row - 1;

            if row < 0 || self.0[row as usize][*column as usize] == '#' {
                return true;
            }
        }

        false
    }

    fn add_row(&mut self) {
        self.0.push(vec!['.'; 7]);
    }

    pub fn tower_height(&self) -> u32 {
        self.0.len() as u32
    }
}

impl std::fmt::Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Print two dummy rows on top of what's already there.
        for row in self.0.iter().rev() {
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

    println!("The towerr is {} units tall", chamber.tower_height());
}
