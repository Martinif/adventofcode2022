use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use adventofcode2022::load_input;

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        Ok(Instruction {
            from: parts[3].parse()?,
            to: parts[5].parse()?,
            amount: parts[1].parse()?,
        })
    }
}

fn prepare_input(s: String) -> (HashMap<usize, Vec<char>>, Vec<Instruction>) {
    let (input_stacks, input_instructions) = s.split_once("\n\n").unwrap();
    // trim characters
    let stacks = input_stacks.lines()
        .map(|line| line.chars()
            .enumerate()
            .filter(|&(i, _)| i % 4 == 1)
            .map(|(_, v)| v)
            .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // transpose
    let mut stacks_transposed = vec![Vec::new(); stacks[0].len()];
    for i in 0..stacks.len() {
        for j in 0..stacks[0].len() {
            stacks_transposed[j].push(stacks[i][j]);
        }
    }

    // build hashmap while trimming whitespace ("air above the containers")
    let stacks_map: HashMap<usize, Vec<char>> = stacks_transposed.into_iter()
        .map(|stack| stack
            .into_iter()
            .filter(|c| *c != ' ')
            .rev()
            .collect::<Vec<char>>())
        .map(|stack| (
            stack[0].to_digit(10).expect("Assumption: Less than 10 stacks") as usize,
            stack[1..].to_vec())
        )
        .collect();

    // parse instructions
    let instructions: Vec<Instruction> = input_instructions.lines()
        .map(|line| line.parse().unwrap())
        .collect();
    (stacks_map, instructions)
}

fn top_elements(stacks: &HashMap<usize, Vec<char>>) -> String {
    let mut part1solution = String::with_capacity(stacks.len());
    for i in 1..stacks.len() + 1 {
        part1solution.push(*stacks
            .get(&i)
            .expect("Stacks not consecutively numbered")
            .last().expect("Empty stack"));
    }
    part1solution
}

fn solver(mut stacks: HashMap<usize, Vec<char>>, instructions: &Vec<Instruction>, part1: bool) -> String {
    for instruction in instructions {
        let entry = stacks.get_mut(&instruction.from).expect("From-Stack does not exist");
        let mut items = entry.split_off(entry.len() - instruction.amount);
        if part1 { items.reverse(); }
        stacks.get_mut(&instruction.to)
            .expect("To-Stack does not exist")
            .append(&mut items);
    }
    top_elements(&stacks)
}


fn part1(stacks: HashMap<usize, Vec<char>>, instructions: &Vec<Instruction>) -> String {
    solver(stacks, instructions, true)
}

fn part2(stacks: HashMap<usize, Vec<char>>, instructions: &Vec<Instruction>) -> String {
    solver(stacks, instructions, false)
}

fn main() {
    let data = load_input("input/05.txt");
    let (stacks, instructions) = prepare_input(data);
    println!("First task: {:?}", part1(stacks.clone(), &instructions));
    println!("Second task: {:?}", part2(stacks, &instructions));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/05.test.txt");
        let (stacks, instructions) = prepare_input(data);
        assert_eq!(part1(stacks, &instructions), "CMZ".to_string());
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/05.test.txt");
        let (stacks, instructions) = prepare_input(data);
        assert_eq!(part2(stacks, &instructions), "MCD".to_string());
    }
}