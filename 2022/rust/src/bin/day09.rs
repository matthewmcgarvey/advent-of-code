use std::{collections::HashSet, fs::read_to_string};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct HeadKnot {
    position: (i32, i32),
    tail: Vec<TailKnot>,
}

#[derive(Debug)]
struct TailKnot {
    position: (i32, i32),
    positions_visited: HashSet<(i32, i32)>,
}

impl HeadKnot {
    fn move_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.position.1 += 1,
            Direction::Down => self.position.1 -= 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        }
        let mut curr_pos = self.position;
        for knot in self.tail.iter_mut() {
            knot.maybe_move(&curr_pos);
            curr_pos = knot.position;
        }
    }
}

impl TailKnot {
    fn maybe_move(&mut self, lead_position: &(i32, i32)) {
        self.positions_visited.insert(self.position);
        // only move if lead is 2 or more away
        if self.distance_from(lead_position) < 2 {
            return;
        }

        if self.position.0 > lead_position.0 {
            self.position.0 -= 1;
        } else if self.position.0 < lead_position.0 {
            self.position.0 += 1;
        }

        if self.position.1 > lead_position.1 {
            self.position.1 -= 1;
        } else if self.position.1 < lead_position.1 {
            self.position.1 += 1;
        }
        self.positions_visited.insert(self.position);
    }

    fn distance_from(&self, lead_position: &(i32, i32)) -> u32 {
        let x_distance = lead_position.0.abs_diff(self.position.0);
        let y_distance = lead_position.1.abs_diff(self.position.1);
        std::cmp::max(x_distance, y_distance)
    }
}

fn main() {
    let input = read_to_string("src/inputs/09.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let directions = parse_input(input);
    let mut head = create_rope(2);
    for direction in directions.iter() {
        head.move_direction(direction);
    }

    let count = head.tail.last().unwrap().positions_visited.len();
    println!("{}", count);
}

fn part2(input: &String) {
    let directions = parse_input(input);
    let mut head = create_rope(10);
    for direction in directions.iter() {
        head.move_direction(direction);
    }

    let count = head.tail.last().unwrap().positions_visited.len();
    println!("{}", count);
}

fn create_rope(knots: u32) -> HeadKnot {
    let mut tail = vec![];
    for _ in 0..(knots - 1) {
        tail.push(TailKnot {
            position: (0, 0),
            positions_visited: HashSet::new(),
        });
    }

    HeadKnot {
        position: (0, 0),
        tail,
    }
}

fn parse_input(input: &String) -> Vec<Direction> {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .flat_map(|(raw_dir, times)| {
            let direction = match raw_dir {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => panic!("woops"),
            };
            let times: usize = times.parse().unwrap();
            vec![direction; times]
        })
        .collect::<Vec<Direction>>()
}
