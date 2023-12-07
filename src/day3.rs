use std::fs;

struct Coordinate {
    x: u32,
    y: u32,
}

struct PartNumber {
    val: u32,
    coordinate: (Coordinate, Coordinate),
}

pub fn part_one() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    for (row, line) in input.lines().enumerate() {
        let mut number = String::new();
        let mut is_formatting_number: bool = false;
        for (char_start_ind, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                number.push(char);
                // println!("{char} @ ({}, {})", char_start_ind, row);
            } else {
                if number.len() != 0 {
                    println!("number is {number}");
                    number.clear();
                }
            }
        }
    }
}
