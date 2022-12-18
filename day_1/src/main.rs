use std::fs;

fn main() {
    let mut res = fs::read_to_string("input.txt").unwrap();
    res.retain(|c| c != '\r');

    let mut all_cals: Vec<i32> = res
        .split("\n\n")
        .map(|stack| {
            stack
                .split("\n")
                .map(|s| s.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();
    all_cals.sort();
    all_cals.reverse();

    println!("Maximum = {:?}", all_cals[0]);
    println!("3 Maximum = {:?}", all_cals[0] + all_cals[1] + all_cals[2]);
}
