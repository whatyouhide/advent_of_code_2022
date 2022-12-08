use std::fs;

pub fn run() {
    // let max = fs::read_to_string("input.txt")
    //     .expect("Should have been able to read the file")
    //     .split("\n\n")
    //     .map(|line_chunks| line_chunks.split("\n"))
    //     .map(|chunk| {
    //         chunk
    //             .map(|calorie| calorie.parse::<i32>().unwrap())
    //             .sum::<i32>()
    //     })
    //     .max()
    //     .expect("List of total calorie counts shouldn't be empty");

    // println!("{:?}", max);

    let mut sorted_asc = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file")
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

fn _runn() {
    // let chunk_sums = fs::read_to_string("inputs/day1.txt")
    //     .expect("Failed to read file")
    //     .split("\n\n") // Split into chunks of lines
    //     .map(|chunk| {
    //         chunk
    //             .split("\n") // Split the chunk into lines
    //             .map(|calorie| calorie.parse::<i32>().unwrap()) // Parse integers
    //             .sum() // Calculate the sum for each chunk
    //     });

    // let max = chunk_sums.max().unwrap();

    // println!("{:?}", top_3_sum);
}
