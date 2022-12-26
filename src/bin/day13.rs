use std::cmp::{min, Ordering};
use std::collections::VecDeque;
use adventofcode2022::load_input;
use crate::Entry::{List, Number};

use serde_json::Value;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Entry {
    Number(i32),
    List{l: VecDeque<Entry>}
}

fn parse(v : Value) -> Entry {
    match v {
        Value::Number(n) => {return Number(n.as_i64().unwrap() as i32)}
        Value::Array(arr) => {
            List {l: arr
                .into_iter()
                .map(|m|parse(m))
                .collect::<VecDeque<Entry>>()
            }
        }
        _ => unreachable!()
    }
}

fn prepare_input(s: String) -> Vec<(Entry, Entry)> {
    let pairs : Vec<(&str, &str)> = s.split("\n\n").map(|pair|pair.split_once('\n').unwrap()).collect();
    let mut pairs_entry: Vec<(Entry, Entry)> = vec![];
    for (p0,p1) in pairs {
        pairs_entry.push((parse(serde_json::from_str(p0).unwrap()), parse(serde_json::from_str(p1).unwrap())))
    }
    pairs_entry
}

fn isrightorder(entry0 : Entry, entry1 : Entry) -> Option<bool> {
    match (entry0, entry1) {
        (Number(n1), Number(n2)) if n1 < n2 => { return Some(true)}
        (Number(n1), Number(n2)) if n1 == n2 => {return None}
        (Number(n1), Number(n2)) if n1 > n2 => { return Some(false)}
        (List {l:l1 }, List {l:l2 }) if l1.is_empty() && !l2.is_empty() => { return Some(true)}
        (List {l:l1 }, List {l:l2 }) if !l1.is_empty() && l2.is_empty() => { return Some(false)}
        (List {l: mut l1 }, List {l: mut l2 }) => {
            let minlen = min(l1.len(), l2.len());
            for _ in 0..minlen {
                let elem1 = l1.pop_front().unwrap();
                let elem2 = l2.pop_front().unwrap();
                if let Some(cmp_result) = isrightorder(elem1, elem2) {return Some(cmp_result);}
            }
            if !(l1.is_empty() && l2.is_empty()) {return isrightorder(List {l:l1}, List {l:l2});}
            else { return None }
        }
        (List {l:l1 }, Number(n2)) => { return isrightorder(List {l:l1}, List {l: VecDeque::from(vec![Number(n2)])})}
        (Number(n1), List {l:l2}) => { return isrightorder(List {l: VecDeque::from(vec![Number(n1)])}, List {l: l2})}
        (e0, e1) => {unreachable!("Unreachable {:?} {:?}", e0, e1)}
    }
}


fn part1(pairs: &Vec<(Entry, Entry)> ) -> i32 {
    pairs.iter()
        .enumerate()
        .filter(|(_, pair)|isrightorder(pair.0.clone(), pair.1.clone()).unwrap())
        .fold(0,| acc, (idx,_)| acc + idx +1)
    as i32
}

fn part2(pairs: &Vec<(Entry, Entry)>) -> i32 {
    let divider = vec![
        List {l: VecDeque::from([List {l: VecDeque::from([Number(2)])}])},
        List {l: VecDeque::from([List {l: VecDeque::from([Number(6)])}])},
    ];
    let mut flattened : Vec<_> = pairs.iter().map(|(e1,e2)|vec![e1,e2]).flatten().collect();
    for entry in divider.iter() {flattened.push(entry)}

    flattened.sort_by(|e0, e1| {
        if isrightorder((*e0).clone(), (*e1).clone()).unwrap() {
            Ordering::Less
        } else { Ordering::Greater }
    });

    flattened.iter().enumerate().filter(|(_ , entry)| divider.contains(entry)).map(|(idx, _)| idx as i32 +1 ).product()
}

fn main() {
    let data = load_input("input/13.txt");
    let input = prepare_input(data);
    println!("First task: {:?}", part1(&input.clone()));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/13.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/13.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 140);
    }
}