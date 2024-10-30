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

fn parse_input(input_data: &str) -> Vec<Card> {
    let cards: Vec<_> = input_data
        .trim()
        .lines()
        .map(|l| parse_input_line(l.trim()))
        .map(|l| extract_nums_from_line(l))
        .flatten()
        .collect();

    cards
}

fn get_score(cards: Vec<Card>) {
    let mut score: u32 = 0;

    for card in cards {
        let mut num_of_wins: u32 = 0;
        for winning_num in card.winning {
            if card.chosen.contains(&winning_num) {
                num_of_wins = num_of_wins + 1;
            }
        }

        if num_of_wins > 0 {
            score = score + 2_u32.pow(num_of_wins - 1);
        }
    }
    println!("Total score = {}", score);
}

fn get_total_scratchcards(cards: Vec<Card>) {
    let mut unchecked: Vec<usize> = vec![1; cards.len()];
    let mut checked: Vec<usize> = vec![0; cards.len()];

    for (idx, card) in cards.iter().enumerate() {
        while !(unchecked[idx] == 0) {
            let mut num_of_wins: usize = 0;
            for winning_num in &card.winning {
                if card.chosen.contains(&winning_num) {
                    num_of_wins = num_of_wins + 1;
                }
            }

            for copy_added in 1..num_of_wins+1 {
                unchecked[idx + copy_added] += 1;
            }
            unchecked[idx] = unchecked[idx] - 1;
            checked[idx] = checked[idx] + 1;
        }
    }
    println!("Total number of scratchcards = {}", checked.iter().fold(0, |sum, i| sum + *i));
}

fn puzzle_1(input: &str) /*-> Result<i32 , PuzzleErr>*/ {
    let cards = parse_input(input);
    get_score(cards);
}

fn puzzle_2(input: &str) {
    let cards = parse_input(input);
    get_total_scratchcards(cards);
}

fn main() {
    println!("Day 4: Scratchcards");

    let file = fs::read_to_string("./input.txt").unwrap();

    puzzle_1(&file);

    puzzle_2(&file);
}
