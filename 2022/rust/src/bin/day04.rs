use std::{fs::read_to_string, ops::RangeInclusive};

fn main() {
    let input = read_to_string("src/inputs/04.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn complete_overlap(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.start() <= b.start() && a.end() >= b.end()
}

fn partial_overlap(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.start() <= b.end() && a.end() >= b.start()
}

fn part1(input: &String) -> () {
    let overlap_count = input
        .lines()
        .map(to_ranges)
        .filter(|(a, b)| complete_overlap(a, b) || complete_overlap(b, a))
        .count();
    println!("{}", overlap_count);
}

fn part2(input: &String) -> () {
    let overlap_count = input
        .lines()
        .map(to_ranges)
        .filter(|(a, b)| partial_overlap(a, b) || partial_overlap(b, a))
        .count();
    println!("{}", overlap_count);
}

fn to_ranges(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let split = line.split_once(',').unwrap();
    let split0 = split.0;
    let temp0 = split0.split_once('-').unwrap();
    let range0 = temp0.0.parse::<u32>().unwrap()..=temp0.1.parse::<u32>().unwrap();

    let split1 = split.1;
    let temp1 = split1.split_once('-').unwrap();
    let range1 = temp1.0.parse::<u32>().unwrap()..=temp1.1.parse::<u32>().unwrap();
    (range0, range1)
}
