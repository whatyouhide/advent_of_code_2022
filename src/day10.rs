const TARGET_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

pub fn run() {
    let input = include_str!("../inputs/day10.txt");

    let mut total = 0;
    let mut register: i32 = 1;
    let mut lines = input.lines();

    let mut addx_cycles = 0;
    let mut next_addx_value = 0;

    for cycle in 0..*TARGET_CYCLES.iter().max().unwrap() {
        let corrected_cycle = cycle + 1;
        if TARGET_CYCLES.contains(&corrected_cycle) {
            let strength = register * corrected_cycle as i32;
            println!(
                "Register at cycle {}: {} (strength: {})",
                corrected_cycle, register, strength
            );

            total += strength;
        }

        if addx_cycles == 1 {
            register += next_addx_value;
            addx_cycles = 0;
            next_addx_value = 0;
        } else {
            let next_line = lines.next().expect("No more lines");

            if next_line == "noop" {
                ();
            } else if next_line.starts_with("addx") {
                let (_, x) = next_line.split_at("addx".len());
                next_addx_value = x.trim().parse::<i32>().unwrap();
                addx_cycles = 1;
            }
        }
    }

    println!("Register at the end: {}", register);
    println!("Total of target cycles: {}", total);
}
