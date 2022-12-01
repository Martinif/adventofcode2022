use adventofcode2022::load_input;

fn prepare_input(s: String) -> Vec<i32> {
    let mut elfs : Vec<i32> = s.split("\n\n")
        .map(|elf| elf.lines().map(|l|l.parse::<i32>().unwrap()).sum())
        .collect();

    elfs.sort_by(|a, b| b.cmp(a));
    elfs
}

fn part1(input : &Vec<i32>) -> i32 {
    input[0]
}

fn part2(input : &Vec<i32>) -> i32 {
    input[0..3].iter().sum()
}

fn main() {
    let data = load_input("input/01.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/01.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/01.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 45000);
    }
}