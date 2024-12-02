use regex::Regex;
use std::fs;

fn parse_input(input: &str) -> Vec<&str> {
    let lines: Vec<&str> = input.trim().lines().collect();
    lines
}

fn extract_lists(lines: Vec<&str>) -> (Vec<i32>, Vec<i32>) {
    let num_re = Regex::new(r"(\d)(\d)").unwrap();
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in lines {
        let _ = num_re.captures_iter(line).map(|caps| {
            let (_, [val1, val2]) = caps.extract();
            list1.push(val1.parse::<i32>().unwrap());
            list2.push(val2.parse::<i32>().unwrap());
        });
    }

    (list1, list2)
}

fn puzzle_1(input: &str) {
    let lines = parse_input(input);
    let (list1, list2) = extract_lists(lines);
}

fn main() {
    println!("Day 1: History Hysteria");

    let file = fs::read_to_string("./input.txt").unwrap();

    puzzle_1(&file);
}
