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

    println!("{:?}", numbers_vec);
    numbers_vec
}

fn get_maps(lines: Vec<&str>) /*-> Vec<Map>*/
{
    let block = lines.join("\n");
    let vec_of_map_lines: Vec<&str> = block.split("\n\n").skip(1).collect();
    //println!("map lines: {:?}", vec_of_map_lines);

    let digit_re = Regex::new(r"(\d+)[.|\\n]?(\d+)[.|\\n]?(\d+)").unwrap();
    let mut maps = Vec::new();

    for line in vec_of_map_lines {
        let mut map = Map {
            dest: Vec::new(),
            source: Vec::new(),
            range: Vec::new(),
        };

        digit_re.captures_iter(line).map(|caps| {
            let (_, [dest_cap, source_cap, range_cap]) = caps.extract();
            map.dest.push(dest_cap.parse::<u32>().unwrap());
            map.source.push(source_cap.parse::<u32>().unwrap());
            map.range.push(range_cap.parse::<u32>().unwrap());
        });

        maps.push(map);
    }
}

fn parse_input(input: &str) -> Vec<&str> {
    let lines: Vec<&str> = input.trim().lines().collect();
    lines
}

fn puzzle_1(input: &str) {
    let seeds = parse_input(input);
    extract_seeds(seeds[0]);
    get_maps(seeds)
}

fn main() {
    println!("Day 5: If You Give A Seed A Fertilizer");

    let file = fs::read_to_string("./input.txt").unwrap();

    puzzle_1(&file);

    // puzzle_2(&file);
}
