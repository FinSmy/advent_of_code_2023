use std::fs;
use regex::Regex;

fn extract_seeds(lines: &str) {

    let numbers_re = Regex::new(r"(\d+)"
        .unwrap()
        .captures(lines)
}

fn parse_input(input: &str) {
    lines = input
        .trim()
        .lines();

    seeds = extract_seeds(lines);
}

fn main() {
    println!("Day 5: If You Give A Seed A Fertilizer");

    let file = fs::read_to_string("./input.txt").unwrap();

    puzzle_1(&file);

    // puzzle_2(&file);
}
