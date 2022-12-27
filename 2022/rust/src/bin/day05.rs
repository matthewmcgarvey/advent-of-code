use std::{collections::VecDeque, fs::read_to_string};

#[derive(Debug)]
struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    let input = read_to_string("src/inputs/05.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) -> () {
    let temp = input.split_once("\n\n").unwrap();
    let mut state = compute_state(temp.0);
    let commands = compute_commands(temp.1);
    let result: String = commands
        .iter()
        .fold(&mut state, apply_command)
        .iter_mut()
        .flat_map(|stack| stack.pop_back())
        .collect();
    println!("{:?}", result);
}

fn part2(input: &String) -> () {
    let temp = input.split_once("\n\n").unwrap();
    let mut state = compute_state(temp.0);
    let commands = compute_commands(temp.1);
    let result: String = commands
        .iter()
        .fold(&mut state, apply_command2)
        .iter_mut()
        .flat_map(|stack| stack.pop_back())
        .collect();
    println!("{:?}", result);
}

fn apply_command<'a>(
    state: &'a mut Vec<VecDeque<char>>,
    command: &Command,
) -> &'a mut Vec<VecDeque<char>> {
    for _ in 0..command.amount {
        let item = state[command.from].pop_back().unwrap();
        state[command.to].push_back(item);
    }
    state
}

fn apply_command2<'a>(
    state: &'a mut Vec<VecDeque<char>>,
    command: &Command,
) -> &'a mut Vec<VecDeque<char>> {
    let new_stack_size = std::cmp::max(state[command.from].len() - command.amount, 0);
    let mut popped_stack = state[command.from].split_off(new_stack_size);

    state[command.to].append(&mut popped_stack);
    state
}

fn compute_commands(str: &str) -> Vec<Command> {
    str.lines()
        .map(|line| {
            let mut iter = line.split(" ");
            // move...
            iter.next();
            let amount: usize = iter.next().unwrap().parse().unwrap();
            // from...
            iter.next();
            let from: usize = iter.next().unwrap().parse().unwrap();
            // to...
            iter.next();
            let to: usize = iter.next().unwrap().parse().unwrap();
            Command {
                amount,
                from: from - 1,
                to: to - 1,
            }
        })
        .collect()
}

fn compute_state(str: &str) -> Vec<VecDeque<char>> {
    let mut state: Vec<VecDeque<char>> = Vec::new();
    for line in str.lines() {
        let mut counter = 0;
        let mut counter2 = 1;
        for (i, char) in line.chars().enumerate() {
            if i != counter2 {
                continue;
            }

            if state.len() == counter {
                state.push(VecDeque::new());
            }
            if char.is_alphabetic() {
                state[counter].push_front(char);
            }
            counter2 += 4;
            counter += 1;
        }
    }

    state
}
