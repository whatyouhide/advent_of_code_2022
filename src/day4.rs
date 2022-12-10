use std::ops::Range;

pub fn run() {
    let input = include_str!("../inputs/day4.txt");

    let mut count: u32 = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(",").unwrap();

        let left_range = parse_into_range(left);
        let right_range = parse_into_range(right);

        if is_overlapping(&left_range, &right_range) {
            count += 1;
        }
    }

    println!("Day 4: {}", count);
}

fn parse_into_range(string: &str) -> Range<u32> {
    let (left, right) = string.split_once("-").unwrap();
    let start = left.parse::<u32>().unwrap();
    let end = right.parse::<u32>().unwrap();
    start..end
}

// Returns true if left and right overlap.
fn is_overlapping(left: &Range<u32>, right: &Range<u32>) -> bool {
    let (min, max) = if left.start < right.start {
        (left, right)
    } else {
        (right, left)
    };

    min.end >= max.start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_into_range() {
        assert_eq!(parse_into_range("1-2"), 1..2);
        assert_eq!(parse_into_range("1-3"), 1..3);
        assert_eq!(parse_into_range("2-3"), 2..3);
    }

    #[test]
    fn test_is_overlapping() {
        assert!(is_overlapping(&(1..3), &(2..4)));
        assert!(is_overlapping(&(1..3), &(3..4)));
        assert!(is_overlapping(&(3..4), &(1..3)));
        assert!(!is_overlapping(&(1..3), &(4..6)));
        assert!(!is_overlapping(&(4..6), &(1..3)));
    }
}
