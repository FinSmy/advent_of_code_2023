use regex::Regex;
use std::fs;

fn extract_numbers(lines: &str) {
    let Some(numbers_re) = Regex::new(r"(\d+)").unwrap().captures(lines) else {
        return;
    };

    println!("{:?}", numbers_re.extract());

    /*
    for (_, substr) in numbers_re {
        if digit_re.is_match(&c.to_string()) {
            num_comps = num_comps * 10 + c.to_digit(10).unwrap();
        } else if !(num_comps == 0) {
            nums.push(num_comps);
            num_comps = 0;
        }
    }
    */
}

fn parse_input(input: &str) -> Vec<&str> {
    let pattern: &str = "seeds";
    let seeds: Vec<&str> = input
        .trim()
        .lines()
        .filter(|&x| x.contains(pattern))
        .collect();
    seeds
}

fn puzzle_1(input: &str) {
    let seeds = parse_input(input);
    println!("Seeds: {:?}", seeds);
    extract_numbers(seeds[0]);
}

fn main() {
    println!("Day 5: If You Give A Seed A Fertilizer");

    let file = fs::read_to_string("./input.txt").unwrap();

    puzzle_1(&file);

    // puzzle_2(&file);
}
