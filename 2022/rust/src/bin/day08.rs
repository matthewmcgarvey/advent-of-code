use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/inputs/08.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let map = parse_input(input);
    let points = points(&map);
    let count = points
        .iter()
        .filter(|point| is_visible(&map, point))
        .count();
    println!("{}", count);
}

fn part2(input: &String) {
    let map = parse_input(input);
    let points = points(&map);
    let best_score = points
        .iter()
        .map(|point| to_score(&map, point))
        .max()
        .unwrap();
    println!("{}", best_score);
}

fn to_score(map: &Vec<Vec<u32>>, point: &(usize, usize)) -> u32 {
    let point_height = map.get(point.1).unwrap().get(point.0).unwrap();
    let left = get_left(map, point);
    let right = get_right(map, point);
    let up = get_up(map, point);
    let down = get_down(map, point);
    let mut left_score = 0;
    for height in left.iter() {
        left_score += 1;
        if height >= point_height {
            break;
        }
    }
    let mut right_score = 0;
    for height in right.iter() {
        right_score += 1;
        if height >= point_height {
            break;
        }
    }
    let mut up_score = 0;
    for height in up.iter() {
        up_score += 1;
        if height >= point_height {
            break;
        }
    }
    let mut down_score = 0;
    for height in down.iter() {
        down_score += 1;
        if height >= point_height {
            break;
        }
    }
    left_score * right_score * up_score * down_score
}

fn is_visible(map: &Vec<Vec<u32>>, point: &(usize, usize)) -> bool {
    let point_height = map.get(point.1).unwrap().get(point.0).unwrap();
    // check left
    let left = get_left(map, point);
    if left.is_empty() || left.iter().all(|tree| tree < point_height) {
        return true;
    }
    // check right
    let right = get_right(map, point);
    if right.is_empty() || right.iter().all(|tree| tree < point_height) {
        return true;
    }
    // check up
    let up = get_up(map, point);
    if up.is_empty() || up.iter().all(|tree| tree < point_height) {
        return true;
    }
    // check down
    let down = get_down(map, point);
    if down.is_empty() || down.iter().all(|tree| tree < point_height) {
        return true;
    }

    return false;
}

fn get_left<'a>(map: &'a Vec<Vec<u32>>, point: &(usize, usize)) -> Vec<u32> {
    let row = &map[point.1];
    let mut temp = row[0..point.0].to_vec();
    temp.reverse();
    temp
}

fn get_right<'a>(map: &'a Vec<Vec<u32>>, point: &(usize, usize)) -> Vec<u32> {
    let row = &map[point.1];
    row[point.0 + 1..].to_vec()
}

fn get_up<'a>(map: &'a Vec<Vec<u32>>, point: &(usize, usize)) -> Vec<u32> {
    let col: Vec<u32> = map.iter().map(|row| row[point.0]).collect();
    let mut temp = col[0..point.1].to_vec();
    temp.reverse();
    temp
}

fn get_down<'a>(map: &'a Vec<Vec<u32>>, point: &(usize, usize)) -> Vec<u32> {
    let col: Vec<u32> = map.iter().map(|row| row[point.0]).collect();
    col[point.1 + 1..].to_vec()
}

fn points(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = vec![];
    for (x, row) in map.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            points.push((x, y));
        }
    }
    return points;
}

fn parse_input(input: &String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}
