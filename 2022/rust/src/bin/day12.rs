use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
struct Position {
    location: (usize, usize),
    height: char,
    starting: bool,
    ending: bool,
}

impl Position {
    fn can_move_to(&self, other: &Self) -> bool {
        (u32::from(self.height) + 1) >= u32::from(other.height)
    }

    fn is_starting(&self) -> bool {
        self.height == 'a'
    }
}

struct Grid {
    positions: Vec<Position>,
}

impl Grid {
    fn starting_position(&self) -> &Position {
        self.positions
            .iter()
            .find(|position| position.starting)
            .unwrap()
    }

    fn ending_position(&self) -> &Position {
        self.positions
            .iter()
            .find(|position| position.ending)
            .unwrap()
    }

    fn surrounding_positions(&self, position: &Position) -> Vec<&Position> {
        let location = position.location;
        let mut surrounding_locations = vec![];
        // only include above if not first row
        if location.0 != 0 {
            surrounding_locations.push((location.0 - 1, location.1));
        }
        surrounding_locations.push((location.0 + 1, location.1));
        // only include left if not furthest left
        if location.1 != 0 {
            surrounding_locations.push((location.0, location.1 - 1));
        }
        surrounding_locations.push((location.0, location.1 + 1));

        let mut surrounding_positions = vec![];

        for grid_position in self.positions.iter() {
            if surrounding_locations.contains(&grid_position.location) {
                surrounding_positions.push(grid_position);
            }
        }

        return surrounding_positions;
    }

    fn available_moves(&self, position: &Position) -> Vec<&Position> {
        let mut surrounding_positions = self.surrounding_positions(position);
        surrounding_positions
            .retain(|surrounding_position| position.can_move_to(surrounding_position));
        return surrounding_positions;
    }

    fn x_len(&self) -> usize {
        self.positions
            .iter()
            .map(|pos| pos.location.0)
            .max()
            .unwrap()
            + 1
    }

    fn y_len(&self) -> usize {
        self.positions
            .iter()
            .map(|pos| pos.location.1)
            .max()
            .unwrap()
            + 1
    }
}

fn main() {
    let input = read_to_string("src/inputs/12.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let grid = parse_grid(input);
    let starting_position = grid.starting_position();
    let ending_position = grid.ending_position();
    let shortest_distance = shortest_distance(&grid, starting_position, ending_position).unwrap();
    println!("{}", shortest_distance);
}

fn part2(input: &String) {
    let grid = parse_grid(input);
    let ending_position = grid.ending_position();
    // do it backwards to avoid waiting hours
    let shortest_distance = shortest_distance2(&grid, ending_position, 'a').unwrap();
    println!("{}", shortest_distance);
}

fn shortest_distance(grid: &Grid, from: &Position, to: &Position) -> Option<u32> {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid.y_len()]; grid.x_len()];
    let mut queue = vec![(from, 0)];

    while !queue.is_empty() {
        let (current, distance) = queue.remove(0);
        if current == to {
            return Some(distance);
        }

        let available_moves = grid.available_moves(current);
        for available_move in available_moves {
            let location = available_move.location;
            if visited[location.0][location.1] {
                continue;
            }
            visited[location.0][location.1] = true;
            queue.push((available_move, distance + 1));
        }
    }

    return None;
}

fn shortest_distance2(grid: &Grid, from: &Position, to: char) -> Option<u32> {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; grid.y_len()]; grid.x_len()];
    let mut queue = vec![(from, 0)];

    while !queue.is_empty() {
        let (current, distance) = queue.remove(0);
        if current.height == to {
            return Some(distance);
        }

        let surrounding_positions = grid.surrounding_positions(current);
        for surr_position in surrounding_positions {
            if !surr_position.can_move_to(current) {
                continue;
            }
            let location = surr_position.location;
            if visited[location.0][location.1] {
                continue;
            }
            visited[location.0][location.1] = true;
            queue.push((surr_position, distance + 1));
        }
    }

    return None;
}

fn parse_grid(input: &String) -> Grid {
    let mut positions: Vec<Position> = vec![];
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            let height = match char {
                'S' => 'a',
                'E' => 'z',
                _ => char,
            };
            positions.push(Position {
                location: (x, y),
                height: height,
                starting: char == 'S',
                ending: char == 'E',
            });
        }
    }
    Grid { positions }
}
