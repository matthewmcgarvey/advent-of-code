use std::fs::read_to_string;

#[derive(Debug)]
struct Item {
    worry_level: u128,
}

impl Item {
    fn relief_update(&mut self) {
        self.worry_level /= 3;
    }
}

#[derive(Debug)]
struct WorryTest {
    divisible_by: u128,
    if_true: usize,
    if_false: usize,
}

impl WorryTest {
    fn apply(&self, item: &Item) -> usize {
        if item.worry_level % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
}

impl Operation {
    fn apply(&self, item: &mut Item) {
        match self {
            Self::Add(num) => item.worry_level += u128::from(*num),
            Self::Multiply(num) => item.worry_level *= u128::from(*num),
            Self::Square => item.worry_level *= item.worry_level,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    worry_test: WorryTest,
    inspection_count: usize,
}

impl Monkey {
    fn inspect_item(&mut self) {
        self.operation.apply(&mut self.items[0]);
        self.inspection_count += 1;
    }

    fn test_and_throw(&mut self, throws: &mut Vec<Throw>) {
        let to: usize = self.worry_test.apply(&self.items[0]);
        let item = self.items.remove(0);
        throws.push(Throw { item, to });
    }
}

struct Throw {
    item: Item,
    to: usize,
}

fn main() {
    let input = read_to_string("src/inputs/11.txt").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut monkeys: Vec<Monkey> = parse_monkeys(input);
    for j in 0..20 {
        let mut throws: Vec<Throw> = vec![];
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            let mut new_throws: Vec<Throw> = vec![];
            while !throws.is_empty() {
                let throw: Throw = throws.remove(0);
                if throw.to == i {
                    monkey.items.push(throw.item);
                } else {
                    new_throws.push(throw);
                }
            }
            throws = new_throws;

            let times = monkey.items.len();
            for _ in 0..times {
                monkey.inspect_item();
                monkey.items[0].relief_update();
                monkey.test_and_throw(&mut throws);
            }
        }

        // clean up remaining throws later monkeys threw to earlier ones
        while !throws.is_empty() {
            let throws: Throw = throws.remove(0);
            let monkey: &mut Monkey = &mut monkeys[throws.to];
            monkey.items.push(throws.item);
        }
    }

    let mut counts: Vec<usize> = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<usize>>();
    counts.sort();
    counts.reverse();
    println!(
        "MONKEY BUSINESS: {}",
        counts.iter().take(2).product::<usize>()
    );
}

fn part2(input: &String) {
    let mut monkeys: Vec<Monkey> = parse_monkeys(input);
    let max: u128 = monkeys
        .iter()
        .map(|monkey| monkey.worry_test.divisible_by)
        .product();
    for j in 0..10_000 {
        let mut throws: Vec<Throw> = vec![];
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            let mut new_throws: Vec<Throw> = vec![];
            while !throws.is_empty() {
                let throw: Throw = throws.remove(0);
                if throw.to == i {
                    monkey.items.push(throw.item);
                } else {
                    new_throws.push(throw);
                }
            }
            throws = new_throws;

            let times = monkey.items.len();
            for _ in 0..times {
                monkey.inspect_item();
                monkey.items[0].worry_level %= max;
                monkey.test_and_throw(&mut throws);
            }
        }

        // clean up remaining throws later monkeys threw to earlier ones
        while !throws.is_empty() {
            let throws: Throw = throws.remove(0);
            let monkey: &mut Monkey = &mut monkeys[throws.to];
            monkey.items.push(throws.item);
        }
    }

    let mut counts: Vec<usize> = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect::<Vec<usize>>();
    counts.sort();
    counts.reverse();
    println!(
        "MONKEY BUSINESS: {}",
        counts.iter().take(2).product::<usize>()
    );
}

fn parse_monkeys(input: &String) -> Vec<Monkey> {
    let mut monkeys = vec![];
    for raw_monkey in input.split("\n\n") {
        let mut lines = raw_monkey.lines();
        // monkey name, don't care as it increments from 0
        // and will match list index
        lines.next();
        let starting_items_line = lines.next().unwrap().trim();
        let starting_items = starting_items_line
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|num| num.parse::<u128>().unwrap())
            .map(|worry_level| Item { worry_level })
            .collect::<Vec<Item>>();

        let (left, right) = lines.next().unwrap().trim().rsplit_once(" ").unwrap();
        let op = left.chars().last().unwrap();
        let operation = match op {
            '+' => Operation::Add(right.parse().unwrap()),
            '*' => {
                if right == "old" {
                    Operation::Square
                } else {
                    Operation::Multiply(right.parse().unwrap())
                }
            }
            _ => panic!("woops"),
        };
        let test_line = lines.next().unwrap().trim();
        let divisible_by: u128 = test_line.split(" ").last().unwrap().parse().unwrap();

        let if_true: usize = lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let if_false: usize = lines
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let worry_test = WorryTest {
            divisible_by,
            if_true,
            if_false,
        };

        monkeys.push(Monkey {
            items: starting_items,
            operation,
            worry_test,
            inspection_count: 0,
        });
    }
    monkeys
}
