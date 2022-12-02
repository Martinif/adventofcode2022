use adventofcode2022::load_input;

// resulting points: (outcome) + (value of the shape we selected)
fn game2points_part1(game: &str) -> i32 {
    match game {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 0 + 3,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 0 + 2,
        "C Z" => 3 + 3,
        _ => unreachable!("Invalid input: {}", game)
    }
}

// resulting points: (outcome) + (value of the shape we selected)
fn game2points_part2(game: &str) -> i32 {
    match game {
        "A X" => 0 + 3,
        "A Y" => 3 + 1,
        "A Z" => 6 + 2,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 0 + 2,
        "C Y" => 3 + 3,
        "C Z" => 6 + 1,
        _ => unreachable!("Invalid input: {}", game)
    }
}

fn part1(input: &String) -> i32 {
    input.lines()
        .map(|l| game2points_part1(l))
        .sum()
}

fn part2(input: &String) -> i32 {
    input.lines()
        .map(|l| game2points_part2(l))
        .sum()
}

fn main() {
    let data = load_input("input/02.txt");
    println!("First task: {:?}", part1(&data));
    println!("Second task: {:?}", part2(&data));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/02.test.txt");
        assert_eq!(part1(&data), 15);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/02.test.txt");
        assert_eq!(part2(&data), 12);
    }
}