use std::fs;

pub fn part_one() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut sum: usize = 0;

    'hand_drawings: for (i, line) in input.lines().enumerate() {
        let index = i + 1;
        let games: Vec<&str> = line.split(':').collect();
        if let Some(draws) = games.last() {
            for drawings in draws.split(';').collect::<Vec<&str>>() {
                for pick in drawings.split(',').collect::<Vec<&str>>() {
                    let hand = pick.split_whitespace().collect::<Vec<&str>>();
                    let amount: usize = hand.first().unwrap().parse().unwrap();
                    let color = hand.last().unwrap();

                    match *color {
                        "red" => {
                            if amount > 12 {
                                continue 'hand_drawings;
                            }
                        }
                        "green" => {
                            if amount > 13 {
                                continue 'hand_drawings;
                            }
                        }
                        "blue" => {
                            if amount > 14 {
                                continue 'hand_drawings;
                            }
                        }
                        _ => {
                            println!("Invalid color");
                        }
                    }
                }
            }

            sum += index;
        }
    }

    println!("Sum of IDS {}", sum);
}
