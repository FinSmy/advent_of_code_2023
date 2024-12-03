use regex::Regex;
use std::fs;

fn extract_lists(lines: &str) -> (Vec<i32>, Vec<i32>) {
    let num_re = Regex::new(r"(?ms)(\d+)\s.*(\d+)").unwrap();
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let caps = num_re.captures_iter(lines);
    for (_, [a, b]) in caps.map(|c| c.extract()) {
        list1.push(a.parse::<i32>().unwrap());
        list2.push(b.parse::<i32>().unwrap());
    }

    (list1, list2)
}

fn accumulate_diffs(v1: Vec<i32>, v2: Vec<i32>) {
    let mut total_diffs: i32 = 0;

    for i in 0..v1.len() {
        let diff = v1[i] - v2[i];
        total_diffs += diff.abs();
    }

    println!("Total difference is : {}", total_diffs);
}

fn puzzle_1(input: &str) {
    let (mut list1, mut list2) = extract_lists(input);
    list1.sort();
    list2.sort();

    accumulate_diffs(list1, list2);
}

fn main() {
    println!("Day 1: History Hysteria");

    let file = fs::read_to_string("./input.txt").unwrap();

    puzzle_1(&file);
}
