use std::env;

mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let day = env::args().nth(1).unwrap_or(String::from("day9"));

    match day.as_str() {
        "day1" => day1::run(),
        "day2" => day2::run(),
        "day3" => day3::run(),
        "day4" => day4::run(),
        "day5" => day5::run(),
        "day6" => day6::run(),
        "day7" => day7::run(),
        "day8" => day8::run(),
        "day9" => day9::run(),
        "day10" => day10::run(),
        _ => println!("No such day: {}", day),
    }
}
