use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cube(i32, i32, i32);

impl Cube {
    pub fn from_string(string: &str) -> Self {
        let parts = string
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        Self(parts[0], parts[1], parts[2])
    }

    fn neighbors(&self) -> [Self; 6] {
        [
            Self(self.0 + 1, self.1, self.2),
            Self(self.0 - 1, self.1, self.2),
            Self(self.0, self.1 + 1, self.2),
            Self(self.0, self.1 - 1, self.2),
            Self(self.0, self.1, self.2 + 1),
            Self(self.0, self.1, self.2 - 1),
        ]
    }

    fn exposed_sides(&self, other_cubes: &HashSet<Self>) -> u16 {
        let adjacent_count = HashSet::from(self.neighbors())
            .intersection(other_cubes)
            .count();

        6 - (adjacent_count as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let cube = Cube::from_string("1,2,3");
        assert_eq!(cube.0, 1);
        assert_eq!(cube.1, 2);
        assert_eq!(cube.2, 3);
    }

    #[test]
    fn test_exposed_sides() {
        let base_cube = Cube::from_string("5,5,5");

        assert_eq!(base_cube.exposed_sides(&HashSet::new()), 6);

        assert_eq!(
            base_cube.exposed_sides(&HashSet::from([Cube::from_string("5,5,4")])),
            5
        );
        assert_eq!(
            base_cube.exposed_sides(&HashSet::from([
                Cube::from_string("5,5,4"),
                Cube::from_string("5,5,6")
            ])),
            4
        );
    }
}

pub fn run(input: &str) {
    let cubes: HashSet<Cube> = HashSet::from_iter(input.trim().lines().map(Cube::from_string));

    let total_sides = total_exposed_sides(&cubes);

    println!("Total exposed sides: {}", total_sides);

    let inverted = invert(&cubes);
    let mut outer = HashSet::new();
    collect(Cube(-1, -1, -1), &inverted.clone(), &mut outer);

    let internal = HashSet::from_iter(inverted.difference(&outer).cloned());
    println!(
        "Internal-facing exposed sides: {}",
        total_sides - total_exposed_sides(&internal)
    );
}

fn invert(cubes: &HashSet<Cube>) -> HashSet<Cube> {
    let (min_x, max_x, min_y, max_y, min_z, max_z) = bounds(&cubes);
    let mut inverted = HashSet::new();

    for x in min_x - 1..=max_x + 1 {
        for y in min_y - 1..=max_y + 1 {
            for z in min_z - 1..=max_z + 1 {
                if !cubes.contains(&Cube(x, y, z)) {
                    inverted.insert(Cube(x, y, z));
                }
            }
        }
    }

    inverted
}

fn collect(pos: Cube, cubes: &HashSet<Cube>, visited: &mut HashSet<Cube>) {
    for neighbor in pos.neighbors() {
        if cubes.contains(&neighbor) && !visited.contains(&neighbor) {
            visited.insert(neighbor.clone());
            collect(neighbor, cubes, visited);
        }
    }
}

fn bounds(cubes: &HashSet<Cube>) -> (i32, i32, i32, i32, i32, i32) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    let mut min_z = 0;
    let mut max_z = 0;

    for cube in cubes.iter() {
        min_x = cube.0.min(min_x);
        max_x = cube.0.max(max_x);
        min_y = cube.1.min(min_y);
        max_y = cube.1.max(max_y);
        min_z = cube.2.min(min_z);
        max_z = cube.2.max(max_z);
    }

    (min_x, max_x, min_y, max_y, min_z, max_z)
}

fn total_exposed_sides(cubes: &HashSet<Cube>) -> u32 {
    cubes
        .iter()
        .map(|cube| {
            let set_with_cube = HashSet::from([cube.clone()]);
            let difference: HashSet<Cube> =
                HashSet::from_iter(cubes.difference(&set_with_cube).cloned());
            cube.exposed_sides(&difference) as u32
        })
        .sum::<u32>()
}
