use std::fs;
use thiserror::Error;
use regex::Regex;
//use itertools::Itertools;

#[derive(Error, Debug, PartialEq, Eq)]
enum PuzzleErr {
    #[error("Could not parse game: '{}'.", .0)]
    GameParsingError(String),
}

#[derive(Debug, Clone)]
struct Card {
    winning: Vec<u32>,
    chosen: Vec<u32>,
}

fn extract_nums_from_line(line: &str) -> Result<Card, PuzzleErr> {
    let mut nums = Vec::new();
    let digit_re = Regex::new(r"\d").unwrap();
    let mut num_comps: u32 = 0;
    for c in line.chars() {
        if digit_re.is_match(&c.to_string()) {
            num_comps = num_comps * 10 + c.to_digit(10).unwrap();
        } else if !(num_comps == 0) {
            nums.push(num_comps);
            num_comps = 0;
        }
    }

    if !(num_comps == 0) {
        nums.push(num_comps);
    }

    Ok(Card {
        winning: (nums[0..10].to_vec()),
        chosen: (nums[10..].to_vec()),
    })
}

fn parse_input_line(input_line: &str) -> &str {
    let parts = input_line.split(": ").collect::<Vec<_>>()[1];
    parts
}

fn parse_input(input_data: &str) /*-> Result<Vec<cards>, PuzzleErr>*/ {
    let cards: Vec<_> = input_data
        .trim()
        .lines()
        .map(|l| parse_input_line(l.trim()))
        .map(|l| extract_nums_from_line(l))
        .flatten()
        .collect();

    let mut score: u32 = 0;

    for card in cards {
        let mut num_of_wins: u32 = 0;
        for winning_num in card.winning {
            if card.chosen.contains(&winning_num) {
                num_of_wins = (num_of_wins + 1);
            }
        }

        if num_of_wins > 0 {
            score = score + 2_u32.pow(num_of_wins - 1);
        }
    }
    println!("Total score = {}", score);
}

fn puzzle_1(input: &str) /*-> Result<i32 , PuzzleErr>*/ {
    let _cards = parse_input(input);
}

fn main() {
    println!("Day 4: Scratchcards");

    let file = fs::read_to_string("./input.txt").unwrap();

    puzzle_1(&file);
    // let answer_1 = puzzle_1(&file);
    // match answer_1 {
    //     Ok(x) => println!("Puzzle 1 answer {}", x),
    //     Err(e) => panic!("No solution found for puzzle_1: {}.", e),
    // }
}
