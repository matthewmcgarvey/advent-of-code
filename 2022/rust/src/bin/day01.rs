use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/inputs/01.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) -> () {
    let mut largest = 0;
    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            if current > largest {
                largest = current;
            }
            current = 0;
            continue;
        }

        let calorie: u32 = line.parse().unwrap();
        current += calorie;
    }
    println!("{largest}")
}

fn part2(input: &String) -> () {
    let mut counts: Vec<u32> = vec![0];
    for line in input.lines() {
        if line.is_empty() {
            counts.push(0);
            continue;
        }

        let current = counts.last_mut().unwrap();
        *current += line.parse::<u32>().unwrap();
    }
    counts.sort();
    let total: u32 = counts.iter().rev().take(3).sum();
    println!("{total}");
}
