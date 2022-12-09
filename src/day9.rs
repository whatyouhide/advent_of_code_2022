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
    head: Position,
    tail: Position,
}

impl Rope {
    fn move_head(&mut self, move_: &Move) {
        match move_.direction {
            Direction::Up => self.head.0 += 1 as i32,
            Direction::Down => self.head.0 -= 1 as i32,
            Direction::Left => self.head.1 -= 1 as i32,
            Direction::Right => self.head.1 += 1 as i32,
        }
    }

    fn update_tail(&mut self, visited_positions: &mut HashSet<Position>) {
        // The tail is already close enough to the head.
        if self.distance_between_head_and_tail() <= 1 {
            visited_positions.insert(self.tail.clone());
            return;
        }

        if self.head.0 == self.tail.0 {
            if self.head.1 > self.tail.1 {
                self.tail.1 += 1;
            } else {
                self.tail.1 -= 1;
            }
        } else if self.head.1 == self.tail.1 {
            if self.head.0 > self.tail.0 {
                self.tail.0 += 1;
            } else {
                self.tail.0 -= 1;
            }
        } else {
            // Diagonal.
            if self.head.0 > self.tail.0 {
                self.tail.0 += 1;
            } else {
                self.tail.0 -= 1;
            }

            if self.head.1 > self.tail.1 {
                self.tail.1 += 1;
            } else {
                self.tail.1 -= 1;
            }
        }

        visited_positions.insert(self.tail.clone());
    }

    fn distance_between_head_and_tail(&self) -> usize {
        if (self.head.0 == self.tail.0 + 1 || self.head.0 == self.tail.0 - 1)
            && (self.head.1 == self.tail.1 + 1 || self.head.1 == self.tail.1 - 1)
        {
            1
        } else {
            ((self.head.0 - self.tail.0).abs() + (self.head.1 - self.tail.1).abs()) as usize
        }
    }
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in (-20..20).rev() {
            for column in -20..20 {
                if (row, column) == self.head {
                    write!(f, "H")?;
                } else if (row, column) == self.tail {
                    write!(f, "T")?;
                } else {
                    write!(f, ".")?;
                }
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
        head: (0, 0),
        tail: (0, 0),
    };

    for move_ in input.lines().map(Move::from_line) {
        for _ in 0..move_.distance {
            rope.move_head(&move_);
            rope.update_tail(&mut visited_positions);
        }
    }

    println!("Visited {} positions", visited_positions.len());
}
