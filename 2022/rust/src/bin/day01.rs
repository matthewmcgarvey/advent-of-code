use std::{fs::File, io::Read};

fn main() {
  let input = get_input();
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
  let mut counts: Vec<u32> = Vec::new();
  let mut current = 0;
  for line in input.lines() {
    if line.is_empty() {
      counts.push(current);
      current = 0;
      continue;
    }

    let calorie: u32 = line.parse().unwrap();
    current += calorie;
  }
  counts.sort();
  let total : u32 = counts.iter().rev().take(3).sum();
  println!("{:?}", total);
}

fn get_input() -> String {
  let mut input = String::new();
  File::open("src/inputs/01.txt")
    .expect("File for this day is missing")
    .read_to_string(&mut input)
    .expect("File for this day was not readable");
  return input;
}
