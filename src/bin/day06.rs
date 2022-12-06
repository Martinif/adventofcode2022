use std::collections::VecDeque;

use adventofcode2022::load_input;

fn prepare_input(s: String) -> Vec<char> {
    s.chars().collect()
}

fn ndifferent(buffer: &VecDeque<char>, marker_len: usize) -> bool {
    if buffer.len() < marker_len { return false; }
    for i in 0..marker_len {
        for j in 0..marker_len {
            if buffer[i] == buffer[j] && i != j { return false; }
        }
    }
    true
}

fn solver(message: &Vec<char>, marker_len: usize) -> usize {
    let mut buffer = VecDeque::with_capacity(marker_len + 1);
    for (i, c) in message.iter().enumerate() {
        if ndifferent(&buffer, marker_len) { return i; }
        buffer.push_back(*c);
        if buffer.len() > marker_len { buffer.pop_front(); }
    }
    unreachable!()
}

fn part1(message: &Vec<char>) -> usize {
    solver(message, 4)
}

fn part2(message: &Vec<char>) -> usize {
    solver(message, 14)
}

fn main() {
    let data = load_input("input/06.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/06.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part1_additional() {
        let datas = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz",
                         "nppdvjthqldpwncqszvftbrmjlhg",
                         "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                         "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"];
        let inputs: Vec<Vec<char>> = datas.iter().map(|d| prepare_input(d.to_string())).collect();
        assert_eq!(inputs.iter().map(|i| part1(i)).collect::<Vec<usize>>(), vec![5, 6, 10, 11]);
    }


    #[test]
    fn test_part2() {
        let data = load_input("input/06.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 19);
    }

    #[test]
    fn test_part2_additional() {
        let datas = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz",
                         "nppdvjthqldpwncqszvftbrmjlhg",
                         "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                         "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"];
        let inputs: Vec<Vec<char>> = datas.iter().map(|d| prepare_input(d.to_string())).collect();
        assert_eq!(inputs.iter().map(|i| part2(i)).collect::<Vec<usize>>(), vec![23, 23, 29, 26]);
    }
}