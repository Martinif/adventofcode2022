use array_tool::vec::Intersect;
use itertools::Itertools;
use adventofcode2022::load_input;

fn prepare_input(s: String) -> Vec<Vec<char>> {
    s.lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn char2prio(c: &char) -> i32 {
    if c.is_lowercase() { *c as i32 - 96 } else { *c as i32 - 38 }
}

fn get_line_prio(line: &Vec<char>) -> i32 {
    let packsize = line.len() / 2;
    let mut unique: char = ' ';
    for c in &line[packsize..] {
        if line[0..packsize].to_vec().iter().any(|ch| ch == c) {
            unique = *c;
            break; // we know that there is exactly one unique item
        }
    }
    char2prio(&unique)
}

fn get_group_prio(group: &[Vec<char>]) -> i32 {
    let (p1, p2, p3): (Vec<char>, Vec<char>, Vec<char>) = group.
        into_iter()
        .map(|f| f.to_owned())
        .collect_tuple()
        .unwrap();
    let badge = p1.intersect(p2).intersect(p3)[0];
    char2prio(&badge)
}

fn part1(input: &Vec<Vec<char>>) -> i32 {
    input.iter()
        .map(get_line_prio)
        .sum()
}

fn part2(input: &Vec<Vec<char>>) -> i32 {
    input.chunks(3)
        .map(get_group_prio)
        .sum()
}

fn main() {
    let data = load_input("input/03.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/03.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 157);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/03.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 70);
    }
}