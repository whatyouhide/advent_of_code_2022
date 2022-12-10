pub fn run() {
    let mut sorted_asc = include_str!("../inputs/day1.txt")
        .split("\n\n")
        .map(|line_chunks| line_chunks.split("\n"))
        .map(|chunk| {
            chunk
                .map(|calorie| calorie.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    sorted_asc.sort();
    sorted_asc.reverse();
    sorted_asc.truncate(3);

    let top_3_sum: i32 = sorted_asc.iter().sum();

    println!("{:?}", top_3_sum);
}
