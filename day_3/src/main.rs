use std::collections::HashSet;
use std::fs;

fn main() {
    let mut res = fs::read_to_string("input.txt").unwrap();
    res.retain(|c| c != '\r');

    let prio: i32 = res
        .split('\n')
        .map(|rs| {
            let mut rucksack = HashSet::new();
            let mut rucksack_2 = HashSet::new();

            for c in rs.split_at(rs.len() / 2).0.chars() {
                rucksack.insert(c);
            }

            for c in rs.split_at(rs.len() / 2).1.chars() {
                rucksack_2.insert(c);
            }

            let key = *rucksack.intersection(&rucksack_2).nth(0).unwrap();
            match key {
                'a'..='z' => key as i32 - 'a' as i32 + 1,
                'A'..='Z' => key as i32 - 'A' as i32 + 27,
                _ => panic!(),
            }
        })
        .sum();

    println!("Total priorities : {}", prio);

    let all_sacks = res
        .split('\n')
        .map(|rs| {
            let mut rucksack = HashSet::new();
            for c in rs.chars() {
                rucksack.insert(c);
            }
            rucksack
        })
        .collect::<Vec<HashSet<char>>>();

    let mut badges = 0;
    for index in (0..all_sacks.len()).step_by(3) {
        let first_intersect = all_sacks[index + 1]
            .intersection(&all_sacks[index + 2])
            .map(|c| c.clone())
            .collect::<HashSet<char>>();
        let key = *all_sacks[index]
            .intersection(&first_intersect)
            .nth(0)
            .unwrap();
        match key {
            'a'..='z' => badges += key as i32 - 'a' as i32 + 1,
            'A'..='Z' => badges += key as i32 - 'A' as i32 + 27,
            _ => panic!(),
        };
    }
    println!("Badges : {}", badges);
}
