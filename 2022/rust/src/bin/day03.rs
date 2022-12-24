use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/inputs/03.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) -> () {
    let score: u32 = input
        .lines()
        .map(|line| {
            let halfway: usize = line.len() / 2;
            let split_str = line.split_at(halfway);
            let char_idx = split_str.0.find(|char| split_str.1.contains(char)).unwrap();
            let letter = split_str.0.chars().nth(char_idx).unwrap();
            match letter {
                'a'..='z' => u32::from(letter) - 96,
                'A'..='Z' => u32::from(letter) - 38,
                _ => panic!("woops"),
            }
        })
        .sum();
    println!("{}", score);
}

fn part2(input: &String) -> () {
    let score: u32 = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            let mut char_count: HashMap<char, u32> = HashMap::new();
            for line in group.iter() {
                let temp = line.chars().collect::<HashSet<char>>();
                for letter in temp.iter() {
                    *char_count.entry(letter.clone()).or_insert(0) += 1;
                }
            }
            let letter: &char = char_count.iter().find(|x| 3.eq(x.1)).unwrap().0;
            match letter {
                'a'..='z' => u32::from(*letter) - 96,
                'A'..='Z' => u32::from(*letter) - 38,
                _ => panic!("woops"),
            }
        })
        .sum();
    println!("{}", score);
}
