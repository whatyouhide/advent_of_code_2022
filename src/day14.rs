use std::collections::HashMap;

type Point = (usize, usize);

type World = HashMap<Point, char>;

pub fn run(input: &str) {
    let mut points = input
        .lines()
        .map(parse_line)
        .flatten()
        .collect::<Vec<Point>>();

    points.sort();
    points.dedup();

    let sand_starting_point = (500, 0);

    let mut world = HashMap::new();

    let min_x = points.iter().map(|(x, _)| x).min().unwrap();
    let max_x = points.iter().map(|(x, _)| x).max().unwrap();
    let min_y = points.iter().map(|(_, y)| y).min().unwrap().min(&0);
    let max_y = points.iter().map(|(_, y)| y).max().unwrap();

    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            if let Some((_, _)) = points.iter().find(|(x1, y1)| x1 == &x && y1 == &y) {
                world.insert((x, y), '#');
            } else {
                world.insert((x, y), '.');
            }
        }
    }

    draw_world(&world);

    let mut units_of_send_to_rest = 0;

    loop {
        if !pour_sand(&mut world, sand_starting_point) {
            break;
        }

        units_of_send_to_rest += 1;
    }

    println!("");
    draw_world(&world);

    println!("Total units of sand that came to rest: {units_of_send_to_rest}");
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
    let min_x = world.iter().map(|((x, _), _)| x).min().unwrap();
    let max_x = world.iter().map(|((x, _), _)| x).max().unwrap();
    let min_y = world.iter().map(|((_, y), _)| y).min().unwrap().min(&0);
    let max_y = world.iter().map(|((_, y), _)| y).max().unwrap();

    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            print!(
                "{}",
                world
                    .get(&(x, y))
                    .expect(format!("No point at ({}, {})", x, y).as_str())
            );
        }
        println!();
    }
}

// Returns true if the sand comes to rest, false if it falls to the endless void.
fn pour_sand(world: &mut World, sand_starting_point: Point) -> bool {
    let mut sand_point = sand_starting_point;

    loop {
        let down = (sand_point.0, sand_point.1 + 1);
        let down_left = (sand_point.0 - 1, sand_point.1 + 1);
        let down_right = (sand_point.0 + 1, sand_point.1 + 1);

        match (
            world.get(&down),
            world.get(&down_left),
            world.get(&down_right),
        ) {
            // There is space right below, so we move the sand down and keep going.
            (Some('.'), _, _) => {
                world.insert(sand_point, '.');
                world.insert(down, '+');
                sand_point = down;
            }

            // Space below is taken by sand or rock, but down left is free.
            (Some('#' | 'o'), Some('.'), _) => {
                world.insert(sand_point, '.');
                world.insert(down_left, '+');
                sand_point = down_left;
            }

            // Spaces below *and* down left are taken by sand or rock, but down right is free.
            (Some('#' | 'o'), Some('#' | 'o'), Some('.')) => {
                world.insert(sand_point, '.');
                world.insert(down_right, '+');
                sand_point = down_right;
            }

            // All spaces are taken, so the sand comes to rest at the current point.
            (Some(_), Some(_), Some(_)) => {
                world.insert(sand_point, 'o');
                return true;
            }

            // The sand has fallen to the endless void.
            _ => {
                return false;
            }
        }
    }
}
