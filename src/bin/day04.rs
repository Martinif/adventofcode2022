use std::str::FromStr;
use std::string::ParseError;

use adventofcode2022::load_input;

#[derive(Debug)]
struct Elf {
    lower: i32,
    upper: i32,
}

#[derive(Debug)]
struct PairOfElfs(Elf, Elf);

impl PairOfElfs {
    fn isfullycontained(&self) -> bool {
        self.0.lower <= self.1.lower && self.0.upper >= self.1.upper ||
            self.1.lower <= self.0.lower && self.1.upper >= self.0.upper
    }

    fn overlap(&self) -> bool {
        !(self.0.upper < self.1.lower ||
            self.1.upper < self.0.lower)
    }
}

impl FromStr for PairOfElfs {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elfs = s.split(',').collect::<Vec<_>>();
        PairOfElfs::try_from(PairOfElfs(elfs[0].parse()?, elfs[1].parse()?))
    }
}

impl FromStr for Elf {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let limits: Vec<i32> = s.split('-').map(|n| n.parse().unwrap()).collect();
        Elf::try_from(Elf { lower: limits[0], upper: limits[1] })
    }
}

fn prepare_input(s: String) -> Vec<PairOfElfs> {
    s.lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

fn part1(input: &Vec<PairOfElfs>) -> i32 {
    input.iter().filter(|p| p.isfullycontained()).count() as i32
}

fn part2(input: &Vec<PairOfElfs>) -> i32 {
    input.iter().filter(|p| p.overlap()).count() as i32
}

fn main() {
    let data = load_input("input/04.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/04.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/04.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 4);
    }
}