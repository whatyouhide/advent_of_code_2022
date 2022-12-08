use std::ops::Range;

pub fn run() {
    let input = include_str!("../inputs/day4.txt");

    let mut count: u32 = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(",").unwrap();

        let left_range = parse_into_range(left);
        let right_range = parse_into_range(right);

        if is_contained(&left_range, &right_range) || is_contained(&right_range, &left_range) {
            count += 1;
        }
    }

    println!("Day 4: {}", count);
}
pub fn _run2() {
    assert_eq!(_is_overlapping(&(1..3), &(2..4)), true);
    assert_eq!(_is_overlapping(&(1..3), &(3..4)), true);
    assert_eq!(_is_overlapping(&(3..4), &(1..3)), true);
    assert_eq!(_is_overlapping(&(1..3), &(4..6)), false);
    assert_eq!(_is_overlapping(&(4..6), &(1..3)), false);

    let input = include_str!("../inputs/day4.txt");

    let mut count: u32 = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(",").unwrap();

        let left_range = parse_into_range(left);
        let right_range = parse_into_range(right);

        if _is_overlapping(&left_range, &right_range) {
            count += 1;
        }
    }

    println!("Day 4: {}", count);
}

fn parse_into_range(input: &str) -> Range<u32> {
    let (left, right) = input.split_once("-").unwrap();
    let start = left.parse::<u32>().unwrap();
    let end = right.parse::<u32>().unwrap();
    start..end
}

// Checks if the second range is contained in the first range.
fn is_contained(left: &Range<u32>, right: &Range<u32>) -> bool {
    right.start >= left.start && right.end <= left.end
}

// Returns true if left and right overlap.
fn _is_overlapping(left: &Range<u32>, right: &Range<u32>) -> bool {
    let (min, max) = if left.start < right.start {
        (left, right)
    } else {
        (right, left)
    };

    min.end >= max.start
}
