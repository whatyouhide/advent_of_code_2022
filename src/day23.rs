use std::{collections::HashMap, fmt::Display, str::FromStr};
use Direction::*;

type Position = (isize, isize);

#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

struct Elves {
    elves: Vec<Position>,
    directions_to_consider: [Direction; 4],
    round_proposals: HashMap<Position, Direction>,
}

impl FromStr for Elves {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elves = Vec::new();

        for (x, line) in s.lines().enumerate() {
            for (y, char) in line.trim().chars().enumerate() {
                match char {
                    '#' => elves.push((x as isize, y as isize)),
                    '.' => continue,
                    _ => return Err(()),
                };
            }
        }

        Ok(Self {
            elves,
            directions_to_consider: [North, South, West, East],
            round_proposals: HashMap::new(),
        })
    }
}

impl Display for Elves {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let min_x = self.elves.iter().min_by_key(|(x, _)| *x).unwrap().0;
        let min_y = self.elves.iter().min_by_key(|(_, y)| *y).unwrap().1;
        let max_x = self.elves.iter().max_by_key(|(x, _)| *x).unwrap().0;
        let max_y = self.elves.iter().max_by_key(|(_, y)| *y).unwrap().1;

        let mut last_row = min_x;

        for x in min_x..=max_x {
            if x != last_row {
                writeln!(f)?;
                last_row = x;
            }

            for y in min_y..=max_y {
                let pos = (x, y);

                if self.elves.contains(&pos) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }

        Ok(())
    }
}

impl Elves {
    pub fn rotate_directions_to_consider(&mut self) {
        let first = self.directions_to_consider[0].clone();

        for i in 0..self.directions_to_consider.len() - 1 {
            self.directions_to_consider[i] = self.directions_to_consider[i + 1].clone();
        }

        self.directions_to_consider[self.directions_to_consider.len() - 1] = first.clone();
    }

    pub fn perform_first_half_of_round(&mut self) {
        for elf in &self.elves {
            let neighbors = Self::adjacent_elves(elf);

            // If no other Elves are in one of the neighbor positions, the Elf does not do
            // anything during this round.
            if neighbors.iter().all(|(_, pos)| !self.elves.contains(&pos)) {
                continue;
            }

            let proposed_direction = self
                .directions_to_consider
                .iter()
                .find(|dir| self.is_viable_direction(&neighbors, **dir));

            match proposed_direction {
                Some(dir) => {
                    self.round_proposals.insert(*elf, *dir);
                }

                None => {
                    continue;
                }
            }
        }
    }

    pub fn perform_second_half_of_round(&mut self) -> u32 {
        let mut proposed_positions: HashMap<Position, Vec<Position>> = HashMap::new();

        for (elf, direction) in &self.round_proposals {
            let (x, y) = elf.clone();

            let new_pos = match direction {
                North => (x - 1, y),
                NorthEast => (x - 1, y + 1),
                East => (x, y + 1),
                SouthEast => (x + 1, y + 1),
                South => (x + 1, y),
                SouthWest => (x + 1, y - 1),
                West => (x, y - 1),
                NorthWest => (x - 1, y - 1),
            };

            match proposed_positions.get_mut(&new_pos) {
                Some(elves) => {
                    elves.push(*elf);
                }

                None => {
                    proposed_positions.insert(new_pos, vec![*elf]);
                }
            };
        }

        let mut moved_elves = 0;

        for (new_pos, elves) in proposed_positions.iter() {
            if elves.len() > 1 {
                continue;
            }

            let elf = elves[0];
            let elf_index = self.elves.iter().position(|e| *e == elf).unwrap();
            self.elves[elf_index] = *new_pos;
            moved_elves += 1;
        }

        self.round_proposals.clear();
        moved_elves
    }

    pub fn empty_tiles(&self) -> u32 {
        let min_x = self.elves.iter().min_by_key(|(x, _)| *x).unwrap().0;
        let min_y = self.elves.iter().min_by_key(|(_, y)| *y).unwrap().1;
        let max_x = self.elves.iter().max_by_key(|(x, _)| *x).unwrap().0;
        let max_y = self.elves.iter().max_by_key(|(_, y)| *y).unwrap().1;

        let mut total = 0;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let pos = (x, y);

                if !self.elves.contains(&pos) {
                    total += 1;
                }
            }
        }

        total
    }

    fn adjacent_elves(elf: &Position) -> HashMap<Direction, Position> {
        let (x, y) = elf.clone();

        HashMap::from([
            (North, (x - 1, y)),
            (NorthEast, (x - 1, y + 1)),
            (East, (x, y + 1)),
            (SouthEast, (x + 1, y + 1)),
            (South, (x + 1, y)),
            (SouthWest, (x + 1, y - 1)),
            (West, (x, y - 1)),
            (NorthWest, (x - 1, y - 1)),
        ])
    }

    fn is_viable_direction(
        &self,
        neighbors: &HashMap<Direction, Position>,
        direction: Direction,
    ) -> bool {
        let three_neighbors = match direction {
            North => [
                neighbors[&North],
                neighbors[&NorthEast],
                neighbors[&NorthWest],
            ],
            South => [
                neighbors[&South],
                neighbors[&SouthEast],
                neighbors[&SouthWest],
            ],
            West => [
                neighbors[&West],
                neighbors[&NorthWest],
                neighbors[&SouthWest],
            ],
            East => [
                neighbors[&East],
                neighbors[&NorthEast],
                neighbors[&SouthEast],
            ],
            _ => panic!("Invalid direction: {:?}", direction),
        };

        three_neighbors.iter().all(|pos| !self.elves.contains(pos))
    }
}

#[cfg(test)]
mod elves_tests {
    use super::*;

    #[test]
    fn test_rotate_directions_to_consider() {
        let mut elves = Elves::from_str("").unwrap();

        elves.rotate_directions_to_consider();
        assert_eq!(elves.directions_to_consider, [South, West, East, North]);

        elves.rotate_directions_to_consider();
        assert_eq!(elves.directions_to_consider, [West, East, North, South]);
    }

    #[test]
    fn test_from_str() {
        let input = ".#.\n..#\n#..";
        let elves = Elves::from_str(input.trim()).unwrap();
        assert_eq!(elves.elves, vec![(0, 1), (1, 2), (2, 0)]);

        assert_eq!(format!("{}", elves), input);
    }

    #[test]
    fn test_display() {}
}

pub fn run(input: &str) {
    let mut elves = input.parse::<Elves>().unwrap();

    for round in 1.. {
        println!("Round {round}...");
        elves.perform_first_half_of_round();
        if elves.perform_second_half_of_round() == 0 {
            break;
        }
        elves.rotate_directions_to_consider();
    }

    println!("The number of empty tiles is {}", elves.empty_tiles());
}
