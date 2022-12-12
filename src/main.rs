use std::{env, fs};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
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
        "day13" => day13::run(input.as_str()),
        "day14" => day14::run(input.as_str()),
        "day15" => day15::run(input.as_str()),
        "day16" => day16::run(input.as_str()),
        "day17" => day17::run(input.as_str()),
        "day18" => day18::run(input.as_str()),
        "day19" => day19::run(input.as_str()),
        "day20" => day20::run(input.as_str()),
        "day21" => day21::run(input.as_str()),
        "day22" => day22::run(input.as_str()),
        "day23" => day23::run(input.as_str()),
        "day24" => day24::run(input.as_str()),
        _ => println!("No such day: {}", day),
    }
}

fn read_file_for_day(day: &str) -> String {
    let contents = fs::read_to_string(format!("inputs/{day}.txt"));

    match contents {
        Ok(contents) => contents,
        Err(_) => {
            if day.ends_with("_test") {
                panic!("No test input file found for day {}", day);
            } else {
                panic!("{day} not implemented yet");
            };
        }
    }
}
