use std::fmt::Display;

use itertools::Itertools;

const DECRYPTION_KEY: i64 = 811589153;

#[derive(Clone)]
struct CircularList {
    elems: Vec<(i64, usize)>,
    size: usize,
}

impl FromIterator<(i64, usize)> for CircularList {
    fn from_iter<I: IntoIterator<Item = (i64, usize)>>(iter: I) -> Self {
        let elems: Vec<(i64, usize)> = iter.into_iter().collect();
        let size = elems.len();
        Self { elems, size }
    }
}

impl Display for CircularList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.elems.iter().map(|(n, _)| n).join(", "))
    }
}

impl CircularList {
    fn move_element(&mut self, element: (i64, usize)) {
        let len = self.size as i64;
        let (number, target_id) = element;

        // First, find the number's current position.
        let current_index = self
            .elems
            .iter()
            .position(|(_, id)| *id == target_id)
            .unwrap();

        // Compute the new index.
        let mut new_index = (current_index as i64 + number) % (len - 1);

        if new_index < 0 {
            new_index = (len - 1) + new_index;
        }

        // Remove the element from its current position.
        let element = self.elems.remove(current_index);
        self.elems.insert(new_index as usize, element);
    }

    fn get_element_from_zero(&self, offset_from_zero: u32) -> i64 {
        let zero_index = self.elems.iter().position(|(n, _)| *n == 0).unwrap();
        let offset = (offset_from_zero + zero_index as u32) % self.size as u32;
        self.elems[offset as usize].0
    }
}

pub fn run(input: &str) {
    let original_numbers = input
        .lines()
        .enumerate()
        .map(|(id, number)| (number.parse::<i64>().unwrap(), id))
        .map(|(number, id)| (number * DECRYPTION_KEY, id))
        .collect::<CircularList>();

    let mut numbers = original_numbers.clone();

    for _ in 1..=10 {
        for element in original_numbers.elems.clone() {
            numbers.move_element(element);
        }
    }

    let n1 = numbers.get_element_from_zero(1000);
    let n2 = numbers.get_element_from_zero(2000);
    let n3 = numbers.get_element_from_zero(3000);

    println!(
        "1000th is {}, 2000th is {}, 3000th is {}, sum is {}",
        n1,
        n2,
        n3,
        n1 + n2 + n3
    );
}
