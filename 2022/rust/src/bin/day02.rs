use std::fs::read_to_string;

#[derive(Copy, Clone, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

struct Game {
    left: Play,
    right: Play,
}

impl Play {
    fn from1(char: char) -> Self {
        match char {
            'A' | 'X' => Play::Rock,
            'B' | 'Y' => Play::Paper,
            'C' | 'Z' => Play::Scissors,
            _ => panic!("Didn't expect to receive"),
        }
    }

    fn from2(char: char) -> Self {
        match char {
            'A' => Play::Rock,
            'B' => Play::Paper,
            'C' => Play::Scissors,
            _ => panic!("Didn't expect to receive"),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn beats(&self) -> Play {
        match self {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        }
    }

    fn loses_to(&self) -> Play {
        match self {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        }
    }
}

impl Game {
    fn from1(raw: &str) -> Self {
        let left = raw.chars().nth(0).unwrap();
        let right = raw.chars().nth(2).unwrap();
        Game {
            left: Play::from1(left),
            right: Play::from1(right),
        }
    }

    fn from2(raw: &str) -> Self {
        let left: Play = Play::from2(raw.chars().nth(0).unwrap());
        let raw_right = raw.chars().nth(2).unwrap();
        let right = match raw_right {
            'X' => left.beats(),
            'Y' => left,
            'Z' => left.loses_to(),
            _ => panic!("blah"),
        };
        Game { left, right }
    }

    fn wins(&self) -> bool {
        self.left == self.right.beats()
    }

    fn ties(&self) -> bool {
        self.left == self.right
    }

    fn score(&self) -> u32 {
        let points = if self.wins() {
            6
        } else if self.ties() {
            3
        } else {
            0
        };
        points + self.right.score()
    }
}

fn main() {
    let input = read_to_string("src/inputs/02.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) -> () {
    let final_score: u32 = input
        .lines()
        .map(|line| Game::from1(line))
        .map(|game| game.score())
        .sum();
    println!("WOAH {final_score}");
}

fn part2(input: &String) -> () {
    let final_score: u32 = input
        .lines()
        .map(|line| Game::from2(line))
        .map(|game| game.score())
        .sum();
    println!("WOAH {final_score}");
}
