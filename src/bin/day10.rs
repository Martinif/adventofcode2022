use std::cmp::max;

use adventofcode2022::load_input;

fn prepare_input(s: String) -> Vec<i32> {
    s.lines()
        .map(|x| match x {
            "noop" => vec![0],
            x => { vec![0, x.split_once(" ").unwrap().1.parse().unwrap()] }
        })
        .flatten()
        .collect()
}

fn part1(instructions: &Vec<i32>) -> i32 {
    let mut signal_strength: i32 = 0;
    let mut x_register: i32 = 1;
    for i in [20, 60, 100, 140, 180, 220] {
        x_register = x_register + instructions[max(0, i - 40 - 1) as usize..i as usize - 1].iter().sum::<i32>();
        signal_strength += i * x_register;
    }
    signal_strength
}


fn part2(instructions: &Vec<i32>) -> String {
    let mut pixels = String::new();
    let mut x_register: i32 = 1;
    for position in 0..240 {
        let location = position % 40;
        if x_register.abs_diff(location) <= 1 { pixels.push('#') } else { pixels.push('.') }
        if location == 39 { pixels.push('\n') }
        x_register += instructions[position as usize];
    }
    pixels
}

fn main() {
    let data = load_input("input/10.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: \n{}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/10.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 13140);
    }

    #[test]
    fn test_part1_small() {
        let data = load_input("input/10_small.test.txt");
        let cycles = prepare_input(data);
        assert_eq!(1 + cycles[..1].iter().sum::<i32>(), 1); // after cycle 1
        assert_eq!(1 + cycles[..2].iter().sum::<i32>(), 1); // after cycle 2
        assert_eq!(1 + cycles[..3].iter().sum::<i32>(), 4); // after cycle 3
        assert_eq!(1 + cycles[..4].iter().sum::<i32>(), 4); // after cycle 4
        assert_eq!(1 + cycles[..5].iter().sum::<i32>(), -1); // after cycle 5
    }


    #[test]
    fn test_part2() {
        let data = load_input("input/10.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input),
                   "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....\n".to_string());
    }
}