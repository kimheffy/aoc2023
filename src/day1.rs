use std::fs;

pub fn part_one() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut stack: Vec<char> = Vec::new();

        for char in line.chars() {
            if char.is_ascii_digit() {
                stack.push(char);
            }
        }

        let combine_char = format!("{}{}", stack[0], stack[stack.len() - 1]);

        sum += combine_char.parse::<u32>().unwrap();
    }

    println!("The total sum is {sum}.");
}

pub fn part_two() {
    let numbers = [
        ("one", 3),
        ("two", 3),
        ("three", 5),
        ("four", 4),
        ("five", 4),
        ("six", 3),
        ("seven", 5),
        ("eight", 5),
        ("nine", 4),
        ("1", 1),
        ("2", 1),
        ("3", 1),
        ("4", 1),
        ("5", 1),
        ("6", 1),
        ("7", 1),
        ("8", 1),
        ("9", 1),
    ];

    let input = fs::read_to_string("src/input.txt").unwrap();

    for line in input.lines() {
        let mut line = line.to_string();

        for (number_literal, number_len) in numbers {
            if line.contains(&number_literal) {
                let ind = line.find(&number_literal).unwrap();
                println!("{}", &line[ind..ind + number_len]);
                break;
            }
        }
    }
}

fn match_string_to_value(str_literal: &str) -> Option<u32> {
    match str_literal {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}
