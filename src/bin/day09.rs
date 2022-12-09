use std::collections::HashSet;

use adventofcode2022::load_input;

#[derive(PartialEq, Debug, Clone)]
enum Direction { Up, Down, Left, Right }

fn prepare_input(s: String) -> Vec<Direction> {
    let mut dir: Vec<Direction> = vec![];
    for line in s.lines() {
        match line.split_once(" ").unwrap() {
            ("R", n) => { dir.append(&mut vec![Direction::Right; n.parse().unwrap()]) }
            ("L", n) => { dir.append(&mut vec![Direction::Left; n.parse().unwrap()]) }
            ("U", n) => { dir.append(&mut vec![Direction::Up; n.parse().unwrap()]) }
            ("D", n) => { dir.append(&mut vec![Direction::Down; n.parse().unwrap()]) }
            (_, _) => unreachable!()
        }
    }
    dir
}

fn istouching(h: &(i32, i32), t: &(i32, i32)) -> bool {
    [h, &(h.0 + 1, h.1), &(h.0 + 1, h.1 + 1), &(h.0, h.1 + 1), &(h.0 - 1, h.1 + 1),
        &(h.0 - 1, h.1), &(h.0 - 1, h.1 - 1), &(h.0, h.1 - 1), &(h.0 + 1, h.1 - 1)].contains(&t)
}

fn check_and_move_tail(h: &(i32, i32), t: &mut (i32, i32)) -> () {
    if !istouching(h, t) {
        // move tail in one of 8 directions
        if h.1 == t.1 && h.0 == t.0 + 2 { // move right
            t.0 += 1;
        } else if h.1 > t.1 && h.0 > t.0 { // move right up
            t.0 += 1;
            t.1 += 1;
        } else if h.0 == t.0 && h.1 == t.1 + 2 { // move up
            t.1 += 1;
        } else if h.0 < t.0 && h.1 > t.1 {  // move left up
            t.0 -= 1;
            t.1 += 1;
        } else if h.1 == t.1 && h.0 == t.0 - 2 { // move left
            t.0 -= 1;
        } else if h.0 < t.0 && h.1 < t.1 {  // move left down
            t.0 -= 1;
            t.1 -= 1;
        } else if h.0 == t.0 && h.1 == t.1 - 2 { // move down
            t.1 -= 1;
        } else if h.0 > t.0 && h.1 < t.1 {  // move right down
            t.0 += 1;
            t.1 -= 1;
        } else { unreachable!("Error in logic handling tail move!") }
    }
}

fn move_head(direction: &Direction, head: &mut (i32, i32)) {
    match direction {
        Direction::Up => { head.1 += 1 }
        Direction::Down => { head.1 -= 1 }
        Direction::Left => { head.0 -= 1 }
        Direction::Right => { head.0 += 1 }
    }
}

fn part1(direction: &Vec<Direction>) -> usize {
    let mut head = (0, 0); // coordinates(x,y)
    let mut tail = (0, 0);
    let mut reached = HashSet::from([(0, 0)]);
    for d in direction {
        move_head(d, &mut head);
        check_and_move_tail(&head, &mut tail); // move tail
        reached.insert(tail);
    }
    reached.len()
}

fn part2(direction: &Vec<Direction>) -> usize {
    let mut knots = vec![(0, 0); 10];
    let mut reached = HashSet::from([(0, 0)]);
    for d in direction {
        move_head(d, &mut knots[0]);
        for i in 0..9 { // move the other parts of the rope, one after the other
            check_and_move_tail(&knots[i].clone(), &mut knots[i + 1]);
        }
        reached.insert(knots[9]);
    }
    reached.len()
}

fn main() {
    let data = load_input("input/09.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/09.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/09.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn test_part2_large() {
        let data = load_input("input/09_larger.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 36);
    }
}