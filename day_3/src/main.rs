use std::fs;
use regex::Regex;

pub struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,   
}

fn parse_to_map(file: &str) -> Map {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    for line in file.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        width = row.len();
        map.push(row);
        height += 1;
    }
    
    Map {
        map,
        width,
        height,
    }
}

fn solve(map: Map) {
    for y in 0..map.height-1 {
        println!("{:?}", map.map[y]);
        for x in 0..map.width-1 {
            let c = map.map[y][x];

            // Find full part number
            if c.is_digit(10) {
                let mut curr_part = c.to_digit(10).unwrap();
                
                let mut look_ahead: usize;
                if x < map.width{
                    look_ahead = 1;
                } else {
                    look_ahead = 0;
                }
                while map.map[y][x+look_ahead].is_digit(10){
                    let add = map.map[y][x+look_ahead].to_digit(10).unwrap();
                    curr_part = curr_part*10 + add;
                    if x+look_ahead+1 < map.width {
                        look_ahead += 1;
                    } else {
                        break;
                    }
                }
                
                // Check for symbol above and below
                if (y > 0) && (y < map.height) {
                    let row_above = y - 1;
                    let row_below = y + 1;
                }

                if (x > 0) && (x < map.width) {
                    let left = x - 1;
                    let right = x + 1;
                }
            }
        }
    }
}

fn main() {
    println!("Day 3: Gear Ratios");

    let file = fs::read_to_string("./input.txt").unwrap();

    let engine_map = parse_to_map(&file);

    solve(engine_map);
}
