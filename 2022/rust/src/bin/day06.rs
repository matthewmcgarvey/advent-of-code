use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("src/inputs/06.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) -> () {
    let signal = input.lines().next().unwrap();
    println!("{}", find_unique_section(signal, 4));
}

fn part2(input: &String) -> () {
    let signal = input.lines().next().unwrap();
    println!("{}", find_unique_section(signal, 14));
}

fn find_unique_section(signal: &str, size: usize) -> usize {
    let chars: Vec<char> = signal.chars().collect();
    for (i, window) in chars[..].windows(size).enumerate() {
        if window.iter().collect::<HashSet<&char>>().len() == size {
            return i + size;
        }
    }
    return 0;
}
