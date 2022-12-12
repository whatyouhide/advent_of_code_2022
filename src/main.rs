use std::env;

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let day = env::args().nth(1).expect("No day to run was specified");

    match day.as_str() {
        "day1" => day1::run(include_str!("../inputs/day1.txt")),
        "day2" => day2::run(include_str!("../inputs/day2.txt")),
        "day3" => day3::run(include_str!("../inputs/day3.txt")),
        "day4" => day4::run(include_str!("../inputs/day4.txt")),
        "day5" => day5::run(include_str!("../inputs/day5.txt")),
        "day6" => day6::run(include_str!("../inputs/day6.txt")),
        "day7" => day7::run(include_str!("../inputs/day7.txt")),
        "day8" => day8::run(include_str!("../inputs/day8.txt")),
        "day9" => day9::run(include_str!("../inputs/day9.txt")),
        "day10" => day10::run(include_str!("../inputs/day10.txt")),
        "day11" => day11::run(include_str!("../inputs/day11.txt")),
        "day12" => day12::run(include_str!("../inputs/day12_test.txt")),
        _ => println!("No such day: {}", day),
    }
}
