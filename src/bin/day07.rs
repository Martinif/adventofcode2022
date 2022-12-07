use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use adventofcode2022::load_input;

/*
Data structure selection:
There are multiple ways to implement trees in Rust, tree of the most popular are
 * Rc<RefCell<Node>> (used here and shown in)
   https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
 * with explicit static lifetimes
 * as a Vec with indices as node indexes as shown in
   https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6
 */

#[derive(PartialEq, Debug, Clone)]
struct Node {
    size: i32,
    name: String,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.printh(0))
    }
}

impl Node {
    pub fn new(size: i32, name: String, parent: Option<Rc<RefCell<Node>>>) -> Node {
        return Node {
            size,
            name,
            parent,
            children: vec![],
        };
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<Node>>) {
        self.children.push(new_node);
    }

    fn printh(&self, depth: usize) -> String {
        let mut s: String = "   ".repeat(depth).to_string() + &self.name + " : " + &self.size.to_string() + "\n";
        if !self.children.is_empty() {
            let mut children = "".to_string();
            for child in &self.children {
                children = children + "" + &child.borrow().printh(depth + 1)
            }
            s = s + &children;
        }
        s
    }

    pub fn calculate_sizes(&mut self) {
        if self.children.is_empty() { // file sizes do not change
            return;
        } else {
            let _: () = self.children.iter().map(|ch| Rc::clone(ch).borrow_mut().calculate_sizes()).collect();
            self.size = self.children.iter().map(|ch| Rc::clone(ch).borrow().size).sum();
        }
    }

    pub fn traverse_part1(&self) -> i32 {
        let mut relevant = 0;
        if self.children.is_empty() {  // we only consider directories
            return 0;
        } else {
            if self.size <= 100000 {
                relevant += self.size
            }
            relevant + self.children.iter().map(|ch| Rc::clone(ch).borrow().traverse_part1()).sum::<i32>()
        }
    }

    pub fn traverse_part2(&self) -> i32 {
        let min_free = self.size - 40000000; // how much space has to be freed at least
        let candidate = self.size;
        self.traverse_part2h(min_free, candidate)
    }

    pub fn traverse_part2h(&self, min_free: i32, mut candidate: i32) -> i32 {
        for ch in &self.children {
            let ch_borrowed = ch.borrow();
            if !ch_borrowed.children.is_empty() { // we only consider directories
                if ch_borrowed.size > min_free && ch_borrowed.size < candidate {
                    candidate = ch_borrowed.size;
                }
                candidate = ch_borrowed.traverse_part2h(min_free, candidate)
            }
        }
        candidate
    }
}

fn prepare_input(s: String) -> Rc<RefCell<Node>> {
    let t = Rc::new(RefCell::new(Node::new(0, "/".to_string(), None)));
    let mut current = Rc::clone(&t);
    let lines: Vec<_> = s.lines().filter(|l| l != &"$ ls").collect();
    for line in &lines[1..] {
        if line.starts_with("dir") {             // add directory
            let name = line.split_once(" ").unwrap().1.to_string();
            let node = Rc::new(RefCell::new(Node::new(0, name, Some(Rc::clone(&current)))));
            current.borrow_mut().children.push(Rc::clone(&node));
        } else if line == &"$ cd .." {               // change to parent directory
            let current_clone = Rc::clone(&current);
            current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
        } else if line.starts_with("$ cd ") {    // change to child directory
            let dirname = line.split_at(5).1.to_string();
            let current_clone = Rc::clone(&current);
            current = Rc::clone(current_clone.borrow().children.iter()
                .filter(|n| (n.borrow().name == dirname))
                .next()
                .unwrap());
        } else {                                     // add directory
            let (size, name) = line.split_once(" ").unwrap();
            let node = Rc::new(RefCell::new(Node::new(size.parse().unwrap(), name.to_string(), Some(Rc::clone(&current)))));
            current.borrow_mut().add_child(Rc::clone(&node));
        }
    }
    t.borrow_mut().calculate_sizes(); //recalculate sizes of directories
    t
}

fn part1(input: &Rc<RefCell<Node>>) -> i32 {
    input.borrow().traverse_part1()
}

fn part2(input: &Rc<RefCell<Node>>) -> i32 {
    input.borrow().traverse_part2()
}

fn main() {
    let data = load_input("input/07.txt");
    let input = prepare_input(data);
    // println!("Tree: \n{}", input.borrow());
    println!("First task: {:?}", part1(&input));
    println!("Second task: {:?}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/07.test.txt");
        let input = prepare_input(data);
        assert_eq!(part1(&input), 95437);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/07.test.txt");
        let input = prepare_input(data);
        assert_eq!(part2(&input), 24933642);
    }
}