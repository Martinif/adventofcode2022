use adventofcode2022::load_input;

fn prepare_input(s: String) -> Vec<i32> {
    s.lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(_input: &Vec<i32>) -> i32 {
    0
}

fn part2(_input: &Vec<i32>) -> i32 {
    0
}

fn main() {
    let data = load_input("input/xx.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/xx.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/xx.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 0);
    }
}