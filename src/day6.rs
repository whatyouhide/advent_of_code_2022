const MARKER_LENGTH: usize = 14;

pub fn run() {
    let input = include_str!("../inputs/day6.txt");

    let mut seen_chars = input.chars().collect::<Vec<char>>();
    let chars = seen_chars.drain(MARKER_LENGTH..).collect::<Vec<char>>();
    let mut counted_chars = MARKER_LENGTH;

    for char in chars {
        if all_chars_are_different(&seen_chars) {
            println!("{:?}", counted_chars);
            return;
        } else {
            seen_chars.remove(0);
            seen_chars.push(char);
            counted_chars += 1;
        }
    }
}

fn all_chars_are_different(chars: &Vec<char>) -> bool {
    let mut chars = chars.clone();
    let full_length = chars.len();
    chars.sort();
    chars.dedup();
    chars.len() == full_length
}
