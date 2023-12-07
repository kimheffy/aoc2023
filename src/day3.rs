use std::fs;

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new() -> Coordinate {
        Coordinate {
            x: 0,
            y: 0,
        }
    }

    fn update_coordinate(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Debug)]
struct PartNumber {
    val: u32,
    coordinate: (Coordinate, Coordinate),
}

impl PartNumber {
    fn new() -> PartNumber {
        PartNumber {
            val: 0,
            coordinate: (Coordinate::new(), Coordinate::new())
        }
    }

    fn add_start_coordinate(&mut self, x: usize, y: usize) {
        self.coordinate.0.update_coordinate(x, y);
    }

    fn add_end_coordinate(&mut self, x: usize, y: usize) {
        self.coordinate.1.update_coordinate(x, y);
    }

    fn update_val(&mut self, val: u32) {
        self.val = val;
    }
}

pub fn part_one() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    'main_loop: for (row, line) in input.lines().enumerate() {
        let mut number = String::new();
        let mut start_formatting_number: bool = false;

        let mut part_number = PartNumber::new();

        for (char_start_ind, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                if !start_formatting_number {
                    start_formatting_number = true;
                    part_number.add_start_coordinate(char_start_ind, row);
                }

                number.push(char);
            } else if !char.is_digit(10) && start_formatting_number {
                start_formatting_number = false;
                part_number.add_end_coordinate(number.len() - 1, row);
                part_number.update_val(number.parse().unwrap());
                number.clear();
                println!("{:#?}", part_number);
            }
        }

        if row == 2 {
            break 'main_loop;
        }
    }
}
