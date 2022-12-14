use std::collections::HashMap;

type Point = (usize, usize);

struct World {
    points: HashMap<Point, char>,
    floor_y: usize,
}

impl World {
    fn at_point(&self, point: &Point) -> Option<&char> {
        match self.points.get(point) {
            Some(char) => Some(char),
            None if point.1 == self.floor_y => Some(&'#'),
            None if point.1 < self.floor_y => Some(&'.'),
            None => None,
        }
    }
}

#[cfg(test)]
mod world_tests {
    use super::*;

    #[test]
    fn test_at_point() {
        let world = World {
            points: HashMap::from([((500, 0), '#')]),
            floor_y: 10,
        };

        assert_eq!(world.at_point(&(500, 0)), Some(&'#'));
        assert_eq!(world.at_point(&(500, 1)), Some(&'.'));
        assert_eq!(world.at_point(&(499, 9)), Some(&'.'));
        assert_eq!(world.at_point(&(501, 9)), Some(&'.'));
        assert_eq!(world.at_point(&(500, 10)), Some(&'#'));
        assert_eq!(world.at_point(&(500, 10)), Some(&'#'));
        assert_eq!(world.at_point(&(499, 10)), Some(&'#'));
        assert_eq!(world.at_point(&(501, 10)), Some(&'#'));
        assert_eq!(world.at_point(&(500, 11)), None);
    }
}

pub fn run(input: &str) {
    let points = input
        .lines()
        .map(parse_line)
        .flatten()
        .collect::<Vec<Point>>();

    let sand_starting_point = (500, 0);

    let min_x = points.iter().map(|(x, _)| x).min().unwrap();
    let max_x = points.iter().map(|(x, _)| x).max().unwrap();
    let min_y = points.iter().map(|(_, y)| y).min().unwrap().min(&0);
    let max_y = points.iter().map(|(_, y)| y).max().unwrap();

    let mut world = World {
        points: HashMap::new(),
        floor_y: max_y + 2,
    };

    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            if let Some((_, _)) = points.iter().find(|(x1, y1)| x1 == &x && y1 == &y) {
                world.points.insert((x, y), '#');
            } else {
                world.points.insert((x, y), '.');
            }
        }
    }

    println!("Starting world:");
    draw_world(&world);

    let mut units_of_send_to_rest = 0;

    loop {
        let rest_point = pour_sand(&mut world, sand_starting_point);
        units_of_send_to_rest += 1;

        if rest_point == sand_starting_point {
            break;
        }
    }

    println!("\nEnd world:");
    draw_world(&world);

    println!("\nTotal units of sand that came to rest: {units_of_send_to_rest}");
}

fn parse_line(line: &str) -> Vec<Point> {
    line.split("->")
        .map(|s| {
            let (x, y) = s.trim().split_once(",").unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<Vec<Point>>()
        .windows(2)
        .flat_map(|pair_of_points| {
            let (x1, y1) = pair_of_points[0];
            let (x2, y2) = pair_of_points[1];

            let mut path = vec![(x1, y1)];

            if x1 == x2 {
                let min_y = y1.min(y2);
                let max_y = y1.max(y2);

                for y in min_y..=max_y {
                    path.push((x1, y));
                }
            } else {
                let min_x = x1.min(x2);
                let max_x = x1.max(x2);

                for x in min_x..=max_x {
                    path.push((x, y1));
                }
            }

            path
        })
        .collect::<Vec<Point>>()
}

fn draw_world(world: &World) {
    let min_x = world.points.iter().map(|((x, _), _)| x).min().unwrap();
    let max_x = world.points.iter().map(|((x, _), _)| x).max().unwrap();
    let min_y = world
        .points
        .iter()
        .map(|((_, y), _)| y)
        .min()
        .unwrap()
        .min(&0);

    for y in *min_y..=world.floor_y {
        for x in *min_x..=*max_x {
            print!("{}", world.at_point(&(x, y)).unwrap());
        }
        println!();
    }
}

// Returns the point where the sand comes to rest.
fn pour_sand(world: &mut World, sand_starting_point: Point) -> Point {
    let mut sand_point = sand_starting_point;

    loop {
        let down = (sand_point.0, sand_point.1 + 1);
        let down_left = (sand_point.0 - 1, sand_point.1 + 1);
        let down_right = (sand_point.0 + 1, sand_point.1 + 1);

        match (
            world.at_point(&down),
            world.at_point(&down_left),
            world.at_point(&down_right),
        ) {
            // There is space right below, so we move the sand down and keep going.
            (Some('.'), _, _) => {
                world.points.insert(sand_point, '.');
                world.points.insert(down, '+');
                sand_point = down;
            }

            // Space below is taken by sand or rock, but down left is free.
            (Some('#' | 'o'), Some('.'), _) => {
                world.points.insert(sand_point, '.');
                world.points.insert(down_left, '+');
                sand_point = down_left;
            }

            // Spaces below *and* down left are taken by sand or rock, but down right is free.
            (Some('#' | 'o'), Some('#' | 'o'), Some('.')) => {
                world.points.insert(sand_point, '.');
                world.points.insert(down_right, '+');
                sand_point = down_right;
            }

            // All spaces are taken, so the sand comes to rest at the current point.
            (Some('#' | 'o'), Some('#' | 'o'), Some('#' | 'o')) => {
                world.points.insert(sand_point, 'o');
                return sand_point;
            }

            (a, b, c) => {
                println!(
                    "Unexpected state at {:?}: {:?} is {:?}, {:?} is {:?}, {:?} is {:?}",
                    sand_point, down, a, down_left, b, down_right, c
                );
                panic!();
            }
        }
    }
}
