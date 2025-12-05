use std::fs;

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    let mut roll_map = Vec::new();

    for (i, line) in input.trim().lines().enumerate() {
        let roll_line = [&[false], &line.chars().map(|c| match c {'@' => true, _ => false}).collect::<Vec<bool>>()[..], &[false]].concat();
        if i == 0 {
            roll_map.push(vec![false; roll_line.len()]);
        }
        roll_map.push(roll_line);

    }

    roll_map.push(vec![false; roll_map[0].len()]);

    return roll_map;
}



fn puzzle_1(input: &str) {
    let roll_map = parse_input(input);

    let mut num_moveable_rolls = 0;
    for (i, roll_line) in roll_map.iter().enumerate() {
        if i == 0 || i == roll_map.len() - 1 {
            continue;
        }
        for (j, cell) in roll_line.iter().enumerate() {
            if j == 0 || j == roll_line.len() - 1 || roll_map[i][j] == false {
                continue;
            }
            let mut num_neighbour_rolls = 0;
            // println!("i = {}, j = {}", i, j);
            if roll_map[i-1][j-1] {
                num_neighbour_rolls += 1;
            }
            if roll_map[i-1][j] {
                num_neighbour_rolls += 1;
            }
            if roll_map[i-1][j+1] {
                num_neighbour_rolls += 1;
            }

            if roll_map[i][j-1] {
                num_neighbour_rolls += 1;
            }
            if roll_map[i][j+1] {
                num_neighbour_rolls += 1;
            }

            if roll_map[i+1][j-1] {
                num_neighbour_rolls += 1;
            }
            if roll_map[i+1][j] {
                num_neighbour_rolls += 1;
            }
            if roll_map[i+1][j+1] {
                num_neighbour_rolls += 1;
            }

            println!("i = {}, j = {}, num_neighbour_rolls = {}, valid = {}", i, j, num_neighbour_rolls, (num_neighbour_rolls < 4));
            if num_neighbour_rolls < 4 {
                num_moveable_rolls += 1;
            }
        }
        println!(" ");
    }
    println!("num_neighbour_rolls = {}", num_moveable_rolls);
}

fn puzzle_2(input: &str) {
    let mut roll_map = parse_input(input);

    let mut num_moveable_rolls = 0;
    let mut num_moveable_rolls_last = -1;

    while num_moveable_rolls != num_moveable_rolls_last {
        for i in 1..roll_map.len() - 1 {
            for j in 1..roll_map[0].len() - 1 {
                if roll_map[i][j] == false {
                    continue;
                }
                let mut num_neighbour_rolls = 0;
                if roll_map[i-1][j-1] {
                    num_neighbour_rolls += 1;
                }
                if roll_map[i-1][j] {
                    num_neighbour_rolls += 1;
                }
                if roll_map[i-1][j+1] {
                    num_neighbour_rolls += 1;
                }

                if roll_map[i][j-1] {
                    num_neighbour_rolls += 1;
                }
                if roll_map[i][j+1] {
                    num_neighbour_rolls += 1;
                }

                if roll_map[i+1][j-1] {
                    num_neighbour_rolls += 1;
                }
                if roll_map[i+1][j] {
                    num_neighbour_rolls += 1;
                }
                if roll_map[i+1][j+1] {
                    num_neighbour_rolls += 1;
                }

                roll_map[i][j] = false;
                // println!("i = {}, j = {}, num_neighbour_rolls = {}, valid = {}", i, j, num_neighbour_rolls, (num_neighbour_rolls < 4));
                if num_neighbour_rolls < 4 {
                    num_moveable_rolls += 1;
                }
            }
            // println!(" ");
        }
        num_moveable_rolls_last = num_moveable_rolls
    }
    println!("num_neighbour_rolls = {}", num_moveable_rolls);
}

fn main() {
    println!("Day 4: Lobby");

    let file = fs::read_to_string("./input.txt").unwrap();

    puzzle_1(&file);
    puzzle_2(&file);
}
