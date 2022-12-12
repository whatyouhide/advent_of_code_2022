use std::{env, fs};

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
    let day_arg = env::args()
        .nth(1)
        .expect("Specify a day (day11) or a day with tests (day11_test)");

    let day = if day_arg.ends_with("_test") {
        &day_arg[0..day_arg.len() - 5]
    } else {
        day_arg.as_str()
    };

    let input = read_file_for_day(&day_arg);

    println!("== Running {day_arg} ==\n");

    match day {
        "day1" => day1::run(input.as_str()),
        "day2" => day2::run(input.as_str()),
        "day3" => day3::run(input.as_str()),
        "day4" => day4::run(input.as_str()),
        "day5" => day5::run(input.as_str()),
        "day6" => day6::run(input.as_str()),
        "day7" => day7::run(input.as_str()),
        "day8" => day8::run(input.as_str()),
        "day9" => day9::run(input.as_str()),
        "day10" => day10::run(input.as_str()),
        "day11" => day11::run(input.as_str()),
        "day12" => day12::run(input.as_str()),
        _ => println!("No such day: {}", day),
    }
}

fn read_file_for_day(day: &str) -> String {
    fs::read_to_string(format!("inputs/{day}.txt")).unwrap()
}
