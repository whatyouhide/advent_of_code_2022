use crate::util::util;

#[derive(Copy, Clone, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum RoundEnd {
    Win,
    Lose,
    Draw,
}

pub fn run() {
    let lines = fs::read_to_string("inputs/day1.txt")
        .expect("Failed to read file")
        .lines();

    let mut total = 0;

    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();

        let opponent_choice = match split[0] {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => panic!("Invalid choice"),
        };

        let round_end = match split[1] {
            "X" => RoundEnd::Lose,
            "Y" => RoundEnd::Draw,
            "Z" => RoundEnd::Win,
            _ => panic!("Invalid round end"),
        };

        let my_choice = choose_based_on_end(opponent_choice, round_end);

        total =
            total + calculate_choice_value(my_choice) + calculate_score(opponent_choice, my_choice);
    }

    println!("{:?}", total);
}

fn choose_based_on_end(opponent_choice: Choice, round_end: RoundEnd) -> Choice {
    match (opponent_choice, round_end) {
        (choice, RoundEnd::Draw) => choice,

        (Choice::Paper, RoundEnd::Win) => Choice::Scissors,
        (Choice::Paper, RoundEnd::Lose) => Choice::Rock,

        (Choice::Rock, RoundEnd::Win) => Choice::Paper,
        (Choice::Rock, RoundEnd::Lose) => Choice::Scissors,

        (Choice::Scissors, RoundEnd::Win) => Choice::Rock,
        (Choice::Scissors, RoundEnd::Lose) => Choice::Paper,
    }
}

fn calculate_choice_value(choice: Choice) -> i32 {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}

fn calculate_score(opponent_choice: Choice, my_choice: Choice) -> i32 {
    match (opponent_choice, my_choice) {
        (Choice::Rock, Choice::Rock)
        | (Choice::Paper, Choice::Paper)
        | (Choice::Scissors, Choice::Scissors) => 3,

        (Choice::Paper, Choice::Rock) => 0,
        (Choice::Scissors, Choice::Rock) => 6,

        (Choice::Rock, Choice::Paper) => 6,
        (Choice::Scissors, Choice::Paper) => 0,

        (Choice::Rock, Choice::Scissors) => 0,
        (Choice::Paper, Choice::Scissors) => 6,
    }
}
