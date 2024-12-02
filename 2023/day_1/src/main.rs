use std::fs;

pub fn solve(input: &str) {
    // iterate over each line and get the two digits in the string
    let ans: u32 = input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|c| c.is_digit(10));

            // if there is no last, then double the first char
            let first = chars
                .next()
                .expect("the line should have at least one digit");
            let num = match chars.last() {
                Some(last) => format!("{}{}", first, last),
                None => format!("{}{}", first, first),
            };
            num.parse::<u32>().unwrap()
        })
        .sum();
    println!("Answer: {}", ans)
}

fn main() {
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("error");

    solve(&contents);
}
