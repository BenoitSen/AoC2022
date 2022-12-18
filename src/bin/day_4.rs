use std::fs;

fn main() {
    let mut res = fs::read_to_string("src/input/input_4.txt").unwrap();
    res.retain(|c| c != '\r');
    let mut included = 0;
    let mut overlap = 0;

    for line in res.split('\n') {
        let a = line
            .split(',')
            .nth(0)
            .unwrap()
            .split('-')
            .nth(0)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let b = line
            .split(',')
            .nth(0)
            .unwrap()
            .split('-')
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let x = line
            .split(',')
            .nth(1)
            .unwrap()
            .split('-')
            .nth(0)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let y = line
            .split(',')
            .nth(1)
            .unwrap()
            .split('-')
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        if (a <= x && b >= y) || (x <= a && y >= b) {
            included += 1;
            overlap += 1;
        } else if (a <= x && b >= x)
            || (a <= y && b >= y)
            || (x <= a && x >= b)
            || (y <= a && y >= b)
        {
            overlap += 1;
        }
    }

    println!("Included : {}", included);
    println!("Overlap : {}", overlap);
}
