use std::fs;

fn puzzle_1(input: &str) {
    let lines = input.trim().lines();

    let mut pos = 50;
    let mut zero_counter = 0;
    let mut zero_clicks_counter = 0;

    for line in lines {
        let dir_sign = match line.chars().nth(0) {
            Some('L') => -1,
            Some('R') => 1,
            _ => 0,
        };

        let dir_mag = line[1..].to_string().trim().parse::<i32>().unwrap();

        if dir_sign > 0 {
            zero_clicks_counter += (pos + dir_mag) / 100;
            pos += dir_mag;
        }
        else {
            zero_clicks_counter += (99 - pos + dir_mag).abs() / 100;
            pos += 100 - dir_mag;
        }
        pos = pos % 100;

        println!("sign = {}, mag = {}, pos = {}, zero_counter = {}, zero_clicks_counter = {}", dir_sign, dir_mag, pos, zero_counter, zero_clicks_counter);
        if pos == 0 {
            zero_counter += 1;
        }
        if (pos < 0) {
            pos += 100;
        }
    }
    println!("Number of zeros hit = {}", zero_counter);
    println!("Number of zero clicks = {}", zero_counter + zero_clicks_counter);
}

fn main() {
    println!("Day 1: Secret Entrance");

    let file = fs::read_to_string("./input.txt").unwrap();

    puzzle_1(&file);

}
