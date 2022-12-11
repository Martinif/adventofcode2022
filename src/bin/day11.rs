use std::cell::RefCell;
use std::collections::VecDeque;
use std::error;
use std::str::FromStr;
use itertools::Itertools;
use adventofcode2022::load_input;

#[derive(PartialEq, Debug, Clone)]
struct Monkey {
    items : RefCell<VecDeque<u64>>,
    operation: Box<fn(u64, Option<u64>) -> u64>,
    operation_payload : Option<u64>,
    test_divisor: u64,
    test_true: usize,
    test_false: usize,
    activity: RefCell<u64>
}

impl FromStr for Monkey {
    type Err = Box<dyn error::Error + 'static>;

    fn from_str<'a>(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().skip(1).collect::<Vec<_>>();
        let items : RefCell<VecDeque<u64>> = RefCell::new(lines[0][18..].split(", ").map(|x|x.parse().unwrap()).collect());
        let op_arg = lines[1][23..].split_once(" ").expect("err");
        let (operation, operation_payload) : (Box<fn(u64, Option<u64>) -> u64>, Option<u64>) = match op_arg {
            ("+", "old") => (Box::new(|x, _none| x+x), None),
            ("+", arg) => ( Box::new(|x, payload| x + payload.unwrap()), Some(arg.parse::<u64>()?)),
            ("*", "old") => (Box::new(|x, _none| x*x), None),
            ("*", arg) => ( Box::new(|x, payload| x * payload.unwrap()), Some(arg.parse::<u64>()?)),
            _ => {unreachable!("Unknown operation!")}
        };
        let test_divisor : u64 = lines[2][21..].parse()?;
        let test_true = lines[3][29..].parse()?;
        let test_false = lines[4][30..].parse()?;
        Ok(Monkey{items, operation, operation_payload, test_divisor, test_true, test_false, activity: RefCell::new(0) })
    }
}

fn reduce_worry(part2 : bool, worry_level : u64, common_modulo : u64) -> u64{
    if !part2 { worry_level/3}
    else { worry_level % common_modulo}
}

fn prepare_input(s: String) -> Vec<Monkey> {
    s.split("\n\n").map(|m|m.parse().unwrap()).collect()
}

fn part1(monkeys: Vec<Monkey>, rounds : u64) -> u64 {
    solver(monkeys, rounds, false)
}

fn part2(monkeys: Vec<Monkey>, rounds : u64) -> u64 {
    solver(monkeys,rounds,true)
}

fn solver(monkeys: Vec<Monkey>, rounds : u64, part2 : bool) -> u64 {
    let common_modulo = monkeys.iter().map(|monkey| monkey.test_divisor).product();
    for _round in 0..rounds {
        for monkey in &monkeys{
            while !monkey.items.borrow().is_empty() {
                let mut item = monkey.items.borrow_mut().pop_front().unwrap();
                *monkey.activity.borrow_mut() += 1;
                item = reduce_worry(part2, (monkey.operation)(item, monkey.operation_payload), common_modulo);
                if item % monkey.test_divisor == 0 {
                    monkeys[monkey.test_true].items.borrow_mut().push_back(item);
                } else {
                    monkeys[monkey.test_false].items.borrow_mut().push_back(item);
                }
            }
        }
    }
    monkeys
        .iter()
        .map(|monkey| *monkey.activity.borrow() as u64)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn main() {
    let data = load_input("input/11.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(input.clone(), 20));
    println!("Second task: {:?}", part2(input, 10000));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/11.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(input, 20), 10605);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/11.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(input, 10000), 2713310158);
    }
}