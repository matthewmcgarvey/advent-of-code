use std::fs::read_to_string;

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i32),
    Store,
}

struct CPU {
    register: i32,
    temp: i32,
}

impl CPU {
    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => return,
            Instruction::Add(amount) => self.temp = self.register + amount,
            Instruction::Store => self.register = self.temp,
        }
    }
}

fn main() {
    let input = read_to_string("src/inputs/10.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let instructions = parse_instructions(input);
    let mut cpu = CPU {
        register: 1,
        temp: 0,
    };
    let mut samples: Vec<i32> = vec![];
    let mut sample_at = 20;
    for (i, instruction) in instructions.iter().enumerate() {
        let cycle = (i + 1) as i32;
        if cycle == sample_at {
            samples.push(cycle * cpu.register);
            sample_at += 40;
        }
        cpu.apply(instruction);
    }
    println!("{}", samples.iter().sum::<i32>());
}

fn part2(input: &String) {
    let instructions = parse_instructions(input);
    let mut cpu = CPU {
        register: 1,
        temp: 0,
    };
    let mut crt: Vec<Vec<bool>> = vec![vec![false; 40]; 6];
    let mut current_row = 0;
    for (i, instruction) in instructions.iter().enumerate() {
        let crt_position = i % 40;
        let pixel_range = (cpu.register - 1)..=(cpu.register + 1);
        let row = &mut crt[current_row];
        if pixel_range.contains(&(crt_position as i32)) {
            row[crt_position] = true;
        }
        if crt_position == 39 {
            current_row += 1;
        }
        cpu.apply(instruction);
    }
    for row in crt {
        for item in row.iter() {
            let temp = if *item { "#" } else { "." };
            print!("{}", temp);
        }
        print!("\n");
    }
}

fn parse_instructions(input: &String) -> Vec<Instruction> {
    input
        .lines()
        .flat_map(|line| {
            if line == "noop" {
                vec![Instruction::Noop]
            } else {
                let amount: i32 = line.split_once(" ").unwrap().1.parse().unwrap();
                vec![Instruction::Add(amount), Instruction::Store]
            }
        })
        .collect::<Vec<Instruction>>()
}
