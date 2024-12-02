use regex::Regex;
use std::fs;

struct Map {
    dest: Vec<u32>,
    source: Vec<u32>,
    range: Vec<u32>,
}

fn extract_seeds(lines: &str) -> Vec<u32> {
    let numbers_re = Regex::new(r"(\d+)").unwrap();
    let captures = numbers_re.captures_iter(lines);

    let numbers_vec: Vec<_> = captures
        .map(|c| c.extract::<1>())
        .map(|(c, _)| c.parse::<u32>().unwrap())
        .collect();

    numbers_vec
}

fn get_maps(input: &str) -> Vec<Map> {
    let vec_of_map_lines: Vec<&str> = input.split("\r\n\r\n").collect();
    let digit_re = Regex::new(r"(?sx)(\d+).?(\d+).?(\d+)").unwrap();
    let mut maps = Vec::new();

    for line in &vec_of_map_lines[1..] {
        let mut map = Map {
            dest: Vec::new(),
            source: Vec::new(),
            range: Vec::new(),
        };

        for (_, [dest_cap, source_cap, range_cap]) in
            digit_re.captures_iter(line).map(|caps| caps.extract())
        {
            map.dest.push(dest_cap.parse::<u32>().unwrap());
            map.source.push(source_cap.parse::<u32>().unwrap());
            map.range.push(range_cap.parse::<u32>().unwrap());
        }

        maps.push(map);
    }
    maps
}

fn parse_input(input: &str) -> Vec<&str> {
    let lines: Vec<&str> = input.trim().lines().collect();
    lines
}

fn find_location(seed: u32, maps: &Vec<Map>) -> u32 {
    let mut next_input: u32 = seed;
    let mut map_i = 0;

    for map in maps {
        let mut result: u32 = 0;
        for i in 0..map.dest.len() {
            let dest = map.dest[i];
            let source = map.source[i];
            let range = map.range[i];
            result = dest;

            if (next_input >= source) && (next_input < source + range) {
                result = next_input - source;
                result = result + dest;
                break;
            }
        }
        next_input = result;
    }
    println!("Map: {}, Result: {}", map_i, next_input);
    map_i += 1;
    println!("Location: {}", next_input);
    next_input
}

fn puzzle_1(input: &str) {
    let lines = parse_input(input);
    let seeds = extract_seeds(lines[0]);
    let all_maps = get_maps(input);

    let mut locations: Vec<u32> = Vec::new();
    for seed in seeds {
        locations.push(find_location(seed, &all_maps));
        println!("\n\n");
    }

    println!("Locations {:?}", locations);
    println!("Output: {}", locations.iter().min().unwrap())
}

fn main() {
    println!("Day 5: If You Give A Seed A Fertilizer");

    let file = fs::read_to_string("./input.txt").unwrap();

    puzzle_1(&file);

    // puzzle_2(&file);
}
