use std::collections::HashSet;

pub fn run(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let chunks = lines.chunks(3).collect::<Vec<&[&str]>>();

    let mut total: i32 = 0;

    for chunk in chunks {
        let (line1, line2, line3) = (chunk[0], chunk[1], chunk[2]);

        let set1: HashSet<char> = HashSet::from_iter(line1.chars());
        let set2: HashSet<char> = HashSet::from_iter(line2.chars());
        let set3: HashSet<char> = HashSet::from_iter(line3.chars());

        let common: char = set1
            .intersection(&set2)
            .map(|c| *c)
            .collect::<HashSet<char>>()
            .intersection(&set3)
            .map(|c| *c)
            .collect::<Vec<char>>()[0];

        total = total + priority(common);
    }

    println!("{:?}", total);
}

fn priority(c: char) -> i32 {
    let ascii_value = c as i32;

    if c.is_lowercase() {
        ascii_value - ('a' as i32) + 1
    } else {
        ascii_value - ('A' as i32) + 27
    }
}
