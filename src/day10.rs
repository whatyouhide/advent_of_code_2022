pub fn run() {
    let input = include_str!("../inputs/day10.txt");

    let mut register: i32 = 1;
    let mut lines = input.lines();

    let mut addx_cycles = 0;
    let mut next_addx_value = 0;

    for cycle in 0..240 {
        let row_cycle = cycle % 40;

        if row_cycle == register || row_cycle == register - 1 || row_cycle == register + 1 {
            print!("#");
        } else {
            print!(".");
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

        if cycle % 40 == 39 {
            println!("");
        }
    }
}
