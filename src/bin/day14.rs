use std::collections::HashMap;
use itertools::Itertools;
use adventofcode2022::load_input;
use crate::Tile::{Rock, Sand};

#[derive(Debug, Clone)]
enum Tile {
    Sand,
    Rock,
}

fn prepare_input(s: String) -> HashMap<(i32, i32), Tile> {
    let mut map = HashMap::new();
    for line in s.lines() {
        for (start, end) in line.split(" -> ").tuple_windows::<(_, _)>() {
            let start_coords: (_, _) = start.split(",").map(|c| c.parse::<i32>().unwrap()).next_tuple().unwrap();
            let end_coords: (_, _) = end.split(",").map(|c| c.parse::<i32>().unwrap()).next_tuple().unwrap();
            if start_coords.0 == end_coords.0 {
                let mut ordered = vec![start_coords.1, end_coords.1];
                ordered.sort();
                for coord in ordered[0]..=ordered[1] {
                    map.insert((start_coords.0, coord), Rock);
                }
            } else {
                let mut ordered = vec![start_coords.0, end_coords.0];
                ordered.sort();
                for coord in ordered[0]..=ordered[1] {
                    map.insert((coord, start_coords.1), Rock);
                }
            }
        }
    }
    map
}

fn solver(mut map: HashMap<(i32, i32), Tile>, part2: bool) -> i32 {
    let mut sand_counter = 0;
    let lowest = map.keys().map(|(_, y)| y).max().unwrap().clone();
    loop {
        let mut sand = (500, 0);
        sand_counter += 1;
        while !part2 && sand.1 < lowest || part2 { // if we are in part 1 then loop until sand.1 < lowest, else loop indefinitely
            // sand settles on the floor
            if part2 && sand.1 == lowest + 1 {
                map.insert(sand, Sand);
                break;
            }
            // try to move one down
            if !map.contains_key(&(sand.0, sand.1 + 1)) {
                sand = (sand.0, sand.1 + 1);
            }
            // try to move diagonal left down
            else if !map.contains_key(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            }
            // try to move diagonal right down
            else if !map.contains_key(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            }
            // sand settles
            else {
                map.insert(sand, Sand);
                break;
            }
        }
        if !part2 && sand.1 == lowest {
            sand_counter -= 1; // the last sand fell off the pyramid
            break;
        }
        if part2 && sand == (500, 0) {
            break;
        }
    }
    sand_counter
}

fn part1(map: HashMap<(i32, i32), Tile>) -> i32 {
    solver(map, false)
}

fn part2(map: HashMap<(i32, i32), Tile>) -> i32 {
    solver(map, true)
}

fn main() {
    let data = load_input("input/14.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(input.clone()));
    println!("Second task: {:?}", part2(input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/14.test.txt");
        let input = prepare_input(data);
        println!("{:?}", input);
        assert_eq!(part1(input), 24);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/14.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(input), 93);
    }
}