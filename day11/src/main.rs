use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    testnum: usize,
    monkey_true: usize,
    monkey_false: usize,
    n_inspections: usize,
}

impl Monkey {
    fn from_input(monkey: &'static str) -> Self {
        let find_part = |marker: &str| {
            let start = &monkey[monkey.find(marker).unwrap() + marker.len()..];
            let end = start.find('\n').unwrap();
            &start[..end]
        };
        let items = find_part("Starting items: ")
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let operation = match find_part("Operation: new = old ").split_once(' ').unwrap() {
            ("+", val) => Operation::Add(val.parse().unwrap()),
            ("*", "old") => Operation::Square,
            ("*", val) => Operation::Multiply(val.parse().unwrap()),
            _ => unreachable!(),
        };
        let testnum = find_part("Test: divisible by ").parse().unwrap();
        let monkey_true = find_part("If true: throw to monkey ").parse().unwrap();
        let monkey_false = find_part("If false: throw to monkey ").parse().unwrap();

        Monkey {
            items,
            operation,
            testnum,
            monkey_true,
            monkey_false,
            n_inspections: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Operation {
    fn exec(&self, val: usize) -> usize {
        match self {
            Operation::Add(x) => val + x,
            Operation::Multiply(x) => val * x,
            Operation::Square => val * val,
        }
    }
}

fn simulate_rounds(mut monkeys: Vec<Monkey>, product: usize, rounds: usize, divide: bool) -> usize {
    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            while let Some(v) = monkeys[idx].items.pop_front() {
                monkeys[idx].n_inspections += 1;
                let mut item = monkeys[idx].operation.exec(v) % product;
                if divide {
                    item /= 3;
                }
                let next = if item % monkeys[idx].testnum == 0 {
                    monkeys[idx].monkey_true
                } else {
                    monkeys[idx].monkey_false
                };
                monkeys[next].items.push_back(item);
            }
        }
    }

    let mut inspected: Vec<_> = monkeys.iter().map(|x| x.n_inspections).collect();
    inspected.sort_by(|a, b| b.cmp(a));
    inspected[0] * inspected[1]
}

fn main() {
    let input = include_str!("../input.txt");
    let monkeys: Vec<Monkey> = input
        .split("Monkey")
        .skip_while(|s| s.is_empty())
        .map(Monkey::from_input)
        .collect();

    let product: usize = monkeys.iter().map(|monkey| monkey.testnum).product();

    let p1 = simulate_rounds(monkeys.clone(), product, 20, true);
    let p2 = simulate_rounds(monkeys, product, 10_000, false);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
