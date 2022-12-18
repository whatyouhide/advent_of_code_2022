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

    fn exposed_sides(&self, other_cubes: &HashSet<Self>) -> u16 {
        let adjacent_cubes = HashSet::from([
            Self(self.0 + 1, self.1, self.2),
            Self(self.0 - 1, self.1, self.2),
            Self(self.0, self.1 + 1, self.2),
            Self(self.0, self.1 - 1, self.2),
            Self(self.0, self.1, self.2 + 1),
            Self(self.0, self.1, self.2 - 1),
        ]);

        let adjacent_count = adjacent_cubes.intersection(other_cubes).count();

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

    let total_exposed_sides = cubes
        .iter()
        .map(|cube| {
            let set_with_cube = HashSet::from([cube.clone()]);
            let difference: HashSet<Cube> =
                HashSet::from_iter(cubes.difference(&set_with_cube).cloned());
            cube.exposed_sides(&difference) as u32
        })
        .sum::<u32>();

    println!("Total exposed sides: {}", total_exposed_sides);
}
