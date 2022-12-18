use std::fs;

fn main() {
    let mut res = fs::read_to_string("input.txt").unwrap();
    res.retain(|c| c != '\r');
    let mut points = 0;
    let mut points_2 = 0;
    for s in res.split('\n') {
        match &s[0..1] {
            "A" => {
                match &s[2..3] {
                    "X" => {
                        points += 1 + 3; // Rock - Rock
                        points_2 += 3 + 0; // Rock - Scissors
                    }
                    "Y" => {
                        points += 2 + 6; // Rock - Paper
                        points_2 += 1 + 3; // Rock - Rock
                    }
                    "Z" => {
                        points += 3 + 0; // Rock - Scissors
                        points_2 += 2 + 6; // Rock - Paper
                    }
                    _ => panic!("{:?}", s),
                }
            }
            "B" => {
                match &s[2..3] {
                    "X" => {
                        points += 1 + 0; // Paper - Rock
                        points_2 += 1 + 0 // Paper - Rock
                    }
                    "Y" => {
                        points += 2 + 3; // Paper - Paper
                        points_2 += 2 + 3; // Paper - Paper
                    }
                    "Z" => {
                        points += 3 + 6; // Paper - Scissors
                        points_2 += 3 + 6 // Paper - Scissors
                    }
                    _ => panic!("{:?}", s),
                }
            }
            "C" => {
                match &s[2..3] {
                    "X" => {
                        points += 1 + 6; // Scissors - Rock
                        points_2 += 2 + 0; // Scissors - Paper
                    }
                    "Y" => {
                        points += 2 + 0; // Scissors - Paper
                        points_2 += 3 + 3; // Scissors - Scissors
                    }
                    "Z" => {
                        points += 3 + 3; // Scissors - Scissors
                        points_2 += 1 + 6; // Scissors - Rock
                    }
                    _ => panic!("{:?}", s),
                }
            }
            _ => panic!("{:?}", s),
        }
    }
    println!("First method {}", points);
    println!("Second method {}", points_2);
}
