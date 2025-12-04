use std::fs;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut battery_banks = Vec::new();

    for line in input.trim().lines() {
        let battery_bank = line.chars().filter_map(|c| Some(c.to_digit(10)? as i32)).collect::<Vec<i32>>();
        battery_banks.push(battery_bank);
    }

    return battery_banks;
}



fn puzzle_1(input: &str) {
    let battery_banks = parse_input(input);

    let mut best_battery_sum = 0;
    for bank in battery_banks {
        let (i, max_start) = bank[0..(bank.len() - 1)].iter().enumerate().min_by_key(|(_, x)| -1 * *x).unwrap();

        let max_end = bank[(i+1)..].iter().max().unwrap();
        println!("max_start = {}, max_end = {}", max_start, max_end);

        best_battery_sum += 10 * max_start + max_end;
    }

    println!("best_battery_sum = {}", best_battery_sum);
}

fn main() {
    println!("Day 3: Lobby");

    let file = fs::read_to_string("./input.txt").unwrap();

    puzzle_1(&file);

}
