use ansi_term::Colour;
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point(i64, i64);

#[derive(Debug)]
struct Grid {
    sensors_and_closest_beacons: HashMap<Point, Point>,
    top_left_corner: Point,
    bottom_right_corner: Point,
}

impl Grid {
    fn new(sensors_and_beacons: Vec<(Point, Point)>) -> Self {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for (sensor, beacon) in &sensors_and_beacons {
            let distance = Self::manhattan_distance(sensor, beacon);
            min_x = min_x.min(sensor.0 + distance).min(sensor.0 - distance);
            max_x = max_x.max(sensor.0 + distance).max(sensor.0 - distance);
            min_y = min_y.min(sensor.1 + distance).min(sensor.1 - distance);
            max_y = max_y.max(sensor.1 + distance).max(sensor.1 - distance);
        }

        Self {
            sensors_and_closest_beacons: HashMap::from_iter(sensors_and_beacons.iter().cloned()),
            top_left_corner: Point(min_x, min_y),
            bottom_right_corner: Point(max_x, max_y),
        }
    }

    #[allow(dead_code)]
    pub fn draw(&self) {
        for y in self.top_left_corner.1..=self.bottom_right_corner.1 {
            print!("{:3} ", y);

            let sensors: HashSet<Point> =
                HashSet::from_iter(self.sensors_and_closest_beacons.keys().cloned());
            let beacons: HashSet<Point> =
                HashSet::from_iter(self.sensors_and_closest_beacons.values().cloned());

            for x in self.top_left_corner.0..=self.bottom_right_corner.0 {
                let point = Point(x, y);

                if x % 5 == 0 {
                    print!("{}", Colour::Blue.prefix());
                }

                if sensors.contains(&point) {
                    print!("{}", Colour::Red.paint("S"));
                } else if beacons.contains(&point) {
                    print!("B");
                } else if self.is_in_sensor_range(&point) {
                    print!("#");
                } else {
                    print!(".");
                }

                print!("{}", Colour::Blue.suffix());
            }
            println!();
        }
    }

    fn is_in_sensor_range(&self, point: &Point) -> bool {
        self.sensors_and_closest_beacons
            .iter()
            .filter(|(sensor, beacon)| point != *beacon && point != *sensor)
            .any(|(sensor, beacon)| {
                Self::manhattan_distance(sensor, point) <= Self::manhattan_distance(sensor, beacon)
            })
    }

    fn manhattan_distance(a: &Point, b: &Point) -> i64 {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }

    fn detected_ranges(&self) -> HashMap<i64, Vec<RangeInclusive<i64>>> {
        let mut ranges_by_row = HashMap::new();

        for y in self.top_left_corner.1..=self.bottom_right_corner.1 {
            ranges_by_row.insert(y, Vec::new());
        }

        for (sensor, beacon) in &self.sensors_and_closest_beacons {
            println!(
                "Building ranges for sensor {:?} and its beacon {:?}",
                sensor, beacon
            );

            let distance = Self::manhattan_distance(sensor, beacon);

            for current_distance in -distance..=distance {
                let y = sensor.1 + current_distance;

                let offset = (current_distance.abs() - distance).abs();

                let range = (sensor.0 - offset)..=(sensor.0 + offset);
                ranges_by_row.get_mut(&y).unwrap().push(range);
            }
        }

        println!("Built all ranges, now merging them.");

        // Merge ranges on each row.
        let mut merged_ranges = HashMap::new();
        for (row, mut ranges) in ranges_by_row {
            ranges.sort_by_key(|range| range.start().clone());

            let mut mut_ranges = ranges.clone();
            mut_ranges.truncate(1);

            let len = &ranges.len();

            for i in 1..*len {
                let next_range = ranges[i].clone();
                let current_range = mut_ranges.pop().unwrap();

                if current_range.end() >= next_range.start() {
                    let min = current_range.start().min(next_range.start());
                    let max = current_range.end().max(next_range.end());
                    mut_ranges.push(*min..=*max);
                } else {
                    mut_ranges.push(current_range);
                    mut_ranges.push(next_range);
                }
            }

            merged_ranges.insert(row, mut_ranges.clone());
        }

        merged_ranges
    }
}

#[cfg(test)]
mod grid_tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {
        let a = Point(23, 12);
        assert_eq!(Grid::manhattan_distance(&a, &a), 0);

        let a = Point(1, 1);
        let b = Point(2, 2);
        assert_eq!(Grid::manhattan_distance(&a, &b), 2);

        let a = Point(1, 2);
        let b = Point(2, 2);
        assert_eq!(Grid::manhattan_distance(&a, &b), 1);

        let a = Point(-1, -1);
        let b = Point(2, 2);
        assert_eq!(Grid::manhattan_distance(&a, &b), 6);
    }

    #[test]
    fn test_is_in_sensor_range() {
        let grid = Grid::new(vec![
            (Point(0, 0), Point(0, 1)),
            (Point(10, 10), Point(5, 5)),
        ]);
        assert!(grid.is_in_sensor_range(&Point(7, 6)));
        assert!(grid.is_in_sensor_range(&Point(5, 6)));
        assert!(!grid.is_in_sensor_range(&Point(4, 4)));
        assert!(!grid.is_in_sensor_range(&Point(0, 0)));
        assert!(!grid.is_in_sensor_range(&Point(0, 1)));
    }
}

pub fn run(input: &str) {
    let sensors_and_beacons = input.lines().map(parse_sensor_and_beacon).collect();
    let grid = Grid::new(sensors_and_beacons);

    // grid.draw();

    println!("Building detected ranges...");
    let detected_ranges = grid.detected_ranges();

    // let target_row = 10;
    // let mut forbidden_positions = 0;

    // for point in grid.points_in_row(target_row) {
    //     if grid.is_in_sensor_range(&point) {
    //         forbidden_positions += 1;
    //     }
    // }

    // println!("On line {target_row} there are {forbidden_positions} forbidden positions");

    for y in 0..=4000000 {
        if y % 1000 == 0 {
            println!("Examining row {}", y);
        }

        let ranges = detected_ranges.get(&y).unwrap();
        let ranges_len = ranges.len();

        if ranges_len > 1 {
            println!("Found the line with a space! It's line {y}");
            println!("It has ranges: {:?}", ranges);
            break;
        }
    }
}

fn parse_sensor_and_beacon(line: &str) -> (Point, Point) {
    let (sensor, beacon) = line.split_once(":").unwrap();
    (parse_coordinates(sensor), parse_coordinates(beacon))
}

fn parse_coordinates(string: &str) -> Point {
    let (x, y) = string.split_once(",").unwrap();
    let (_, x) = x.split_once("=").unwrap();
    let x = x.trim().parse::<i64>().unwrap();

    let (_, y) = y.split_once("=").unwrap();
    let y = y.trim().parse::<i64>().unwrap();

    Point(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coordinates() {
        assert_eq!(parse_coordinates("x=495, y=2"), Point(495, 2));
        assert_eq!(parse_coordinates("x=495, y=2 "), Point(495, 2));
        assert_eq!(parse_coordinates(" x=495, y=2"), Point(495, 2));
        assert_eq!(parse_coordinates(" x=495, y=2 "), Point(495, 2));
    }

    #[test]
    fn test_parse_sensor() {
        assert_eq!(
            parse_sensor_and_beacon("Sensor at x=0, y=11: closest beacon is at x=-2, y=10"),
            (Point(0, 11), Point(-2, 10),)
        );
    }
}
