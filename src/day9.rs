use core::fmt;
use std::collections::HashSet;

type Position = (i32, i32);

// Grid looks like this:
// (2, 0) (2, 1) (2, 2) (2, 3)
// (1, 0) (1, 1) (1, 2) (1, 3)
// (0, 0) (0, 1) (0, 2) (0, 3)

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    direction: Direction,
    distance: usize,
}

impl Move {
    pub fn from_line(line: &str) -> Move {
        let (direction, distance) = line.split_at(1);
        let distance = distance.trim().parse::<usize>().unwrap();

        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction: {}", direction),
        };

        Move {
            direction,
            distance,
        }
    }
}

#[derive(Debug)]
struct Rope {
    knots: [Position; 10],
}

impl Rope {
    fn move_head(&mut self, move_: &Move) {
        match move_.direction {
            Direction::Up => self.knots[0].0 += 1 as i32,
            Direction::Down => self.knots[0].0 -= 1 as i32,
            Direction::Left => self.knots[0].1 -= 1 as i32,
            Direction::Right => self.knots[0].1 += 1 as i32,
        }
    }

    fn update_other_knots(&mut self, visited_positions: &mut HashSet<Position>) {
        for index in 1..self.knots.len() {
            // This knot is already close enough to the next.
            if self.distance_between_knot_and_next(index) <= 1 {
                break;
            }

            if self.knots[index - 1].0 == self.knots[index].0 {
                if self.knots[index - 1].1 > self.knots[index].1 {
                    self.knots[index].1 += 1;
                } else {
                    self.knots[index].1 -= 1;
                }
            } else if self.knots[index - 1].1 == self.knots[index].1 {
                if self.knots[index - 1].0 > self.knots[index].0 {
                    self.knots[index].0 += 1;
                } else {
                    self.knots[index].0 -= 1;
                }
            } else {
                // Diagonal.
                if self.knots[index - 1].0 > self.knots[index].0 {
                    self.knots[index].0 += 1;
                } else {
                    self.knots[index].0 -= 1;
                }

                if self.knots[index - 1].1 > self.knots[index].1 {
                    self.knots[index].1 += 1;
                } else {
                    self.knots[index].1 -= 1;
                }
            }
        }

        visited_positions.insert(self.knots[self.knots.len() - 1].clone());
    }

    fn distance_between_knot_and_next(&self, knot_index: usize) -> usize {
        if (self.knots[knot_index - 1].0 == self.knots[knot_index].0 + 1
            || self.knots[knot_index - 1].0 == self.knots[knot_index].0 - 1)
            && (self.knots[knot_index - 1].1 == self.knots[knot_index].1 + 1
                || self.knots[knot_index - 1].1 == self.knots[knot_index].1 - 1)
        {
            1
        } else {
            ((self.knots[knot_index - 1].0 - self.knots[knot_index].0).abs()
                + (self.knots[knot_index - 1].1 - self.knots[knot_index].1).abs())
                as usize
        }
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (-20..20).rev() {
            for column in -20..20 {
                let mut found = false;
                for index in 0..self.knots.len() {
                    if self.knots[index] == (row, column) {
                        write!(f, "{}", index)?;
                        found = true;
                    }
                }

                if !found {
                    write!(f, ".")?
                };
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub fn run() {
    let input = include_str!("../inputs/day9.txt");
    let mut visited_positions: HashSet<Position> = HashSet::new();

    let mut rope = Rope {
        knots: [(0, 0); 10],
    };

    for move_ in input.lines().map(Move::from_line) {
        for _ in 0..move_.distance {
            rope.move_head(&move_);
            rope.update_other_knots(&mut visited_positions);
        }
    }

    println!("Visited {} positions", visited_positions.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let move_ = Move::from_line("U 1");
        assert_eq!(move_.direction, Direction::Up);
        assert_eq!(move_.distance, 1);

        let move_ = Move::from_line("D 5");
        assert_eq!(move_.direction, Direction::Down);
        assert_eq!(move_.distance, 5);

        let move_ = Move::from_line("L 11");
        assert_eq!(move_.direction, Direction::Left);
        assert_eq!(move_.distance, 11);

        let move_ = Move::from_line("R 1");
        assert_eq!(move_.direction, Direction::Right);
        assert_eq!(move_.distance, 1);
    }
}
