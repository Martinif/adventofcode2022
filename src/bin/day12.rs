use std::collections::HashMap;
use itertools::Itertools;

use adventofcode2022::load_input;

fn prepare_input(s: String) -> Vec<Vec<char>> {
    s.lines()
        .map(|x| x.chars().collect())
        .collect()
}

fn build_graph(
    map: &Vec<Vec<char>>,
    startletter: char,
    endletters: Vec<char>,
    node_connected: fn((usize, usize), (usize, usize), &Vec<Vec<char>>) -> bool)
    -> (HashMap<(usize, usize), Vec<(usize, usize)>>, (usize, usize), (usize, usize)) {
    // graph as adjacency list, start node for dijkstra, end node (applicable only in part1)
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = Default::default();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut waitlist: Vec<(usize, usize)> = vec![];
    // find start
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == startletter {
                start = (i, j);
                break;
            }
        }
    }
    // build graph
    waitlist.push(start);
    while !waitlist.is_empty() {
        let node = waitlist.pop().unwrap();
        if graph.get(&node).is_none() {
            // collect neighbouring nodes
            let mut neighbours: Vec<(usize, usize)> = vec![];
            if node.0 != 0 && node_connected(node, (node.0 - 1, node.1), &map) { neighbours.push((node.0 - 1, node.1)) } // up
            if node.1 != 0 && node_connected(node, (node.0, node.1 - 1), &map) { neighbours.push((node.0, node.1 - 1)) } // left
            if node.0 != map.len() - 1 && node_connected(node, (node.0 + 1, node.1), &map) { neighbours.push((node.0 + 1, node.1)) } // down
            if node.1 != map[0].len() - 1 && node_connected(node, (node.0, node.1 + 1), &map) { neighbours.push((node.0, node.1 + 1)) } // right
            // push those neighbours to waitlist, that are not in the waitlist or the graph already
            let mut neighbours_to_add: Vec<(usize, usize)> = neighbours.clone().into_iter().filter(|n| !waitlist.contains(n) && !graph.keys().contains(n)).collect();
            if endletters.iter().map(|l| l != &map[node.0][node.1]).reduce(|a, b| a || b).unwrap() { waitlist.append(&mut neighbours_to_add) } else { end = node }
            // push node to graph
            graph.insert(node, neighbours);
        }
    }
    (graph, start, end)
}

fn nodes_connected(node1: (usize, usize), node2: (usize, usize), map: &Vec<Vec<char>>) -> bool {
    map[node1.0][node1.1] as u32 + 1 >= map[node2.0][node2.1] as u32 && map[node2.0][node2.1] != 'E' ||
        map[node2.0][node2.1] == 'a' ||
        map[node1.0][node1.1] == 'z' && map[node2.0][node2.1] == 'E'
}

fn nodes_connected_inverse(node1: (usize, usize), node2: (usize, usize), map: &Vec<Vec<char>>) -> bool {
    map[node1.0][node1.1] == 'E' && map[node2.0][node2.1] == 'z' ||
        map[node1.0][node1.1] != 'E' && map[node1.0][node1.1] as u32 <= map[node2.0][node2.1] as u32 + 1 ||
        map[node1.0][node1.1] != 'E' && map[node1.0][node1.1] == 'b' && map[node2.0][node2.1] == 'S'
}


fn dijkstra_end_coordinates(graph: &HashMap<(usize, usize), Vec<(usize, usize)>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut pathlengths: HashMap<(usize, usize), u32> = Default::default();
    pathlengths.insert(start, 0);
    let mut waitlist: Vec<(usize, usize)> = vec![start];
    while !waitlist.is_empty() {
        waitlist.sort_by(|a, b| pathlengths[b].cmp(&pathlengths[a]));
        let node = waitlist.pop().unwrap();
        for neighbour in &graph[&node] {
            if neighbour == &end {
                return pathlengths[&node] + 1;
            }
            if !pathlengths.contains_key(&neighbour) || pathlengths[&neighbour] > pathlengths[&node] + 1 {
                pathlengths.insert(*neighbour, pathlengths[&node] + 1);
                if !waitlist.contains(neighbour) { waitlist.push(*neighbour) }
            }
        }
    }
    unreachable!()
}

fn dijkstra_end_letters(graph: &HashMap<(usize, usize), Vec<(usize, usize)>>, start: (usize, usize), map: Vec<Vec<char>>) -> u32 {
    let mut pathlengths: HashMap<(usize, usize), u32> = Default::default();
    pathlengths.insert(start, 0);
    let mut waitlist: Vec<(usize, usize)> = vec![start];
    while !waitlist.is_empty() {
        waitlist.sort_by(|a, b| pathlengths[b].cmp(&pathlengths[a]));
        let node = waitlist.pop().unwrap();
        for neighbour in &graph[&node] {
            if map[neighbour.0][neighbour.1] == 'a' || map[neighbour.0][neighbour.1] == 'S' {
                return pathlengths[&node] + 1;
            }
            if !pathlengths.contains_key(&neighbour) || pathlengths[&neighbour] > pathlengths[&node] + 1 {
                pathlengths.insert(*neighbour, pathlengths[&node] + 1);
                if !waitlist.contains(neighbour) { waitlist.push(*neighbour) }
            }
        }
    }
    unreachable!()
}

fn part1(graph: &HashMap<(usize, usize), Vec<(usize, usize)>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    dijkstra_end_coordinates(graph, start, end)
}

fn part2(graph: &HashMap<(usize, usize), Vec<(usize, usize)>>, start: (usize, usize), map: Vec<Vec<char>>) -> u32 {
    dijkstra_end_letters(graph, start, map)
}

fn main() {
    let data = load_input("input/12.txt");
    let map = prepare_input(data);
    let input = build_graph(&map, 'S', vec!['E'], nodes_connected);
    println!("First task: {:?}", part1(&input.0, input.1, input.2));
    let input2 = build_graph(&map, 'E', vec!['a', 'S'], nodes_connected_inverse);
    println!("First task: {:?}", part2(&input2.0, input2.1, map));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/12.test.txt");
        let map = prepare_input(data);
        let input = build_graph(&map, 'S', vec!['E'], nodes_connected);
        assert_eq!(part1(&input.0, input.1, input.2), 31);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/12.test.txt");
        let map = prepare_input(data);
        let input = build_graph(&map, 'E', vec!['a', 'S'], nodes_connected_inverse);
        assert_eq!(part2(&input.0, input.1, map), 29);
    }
}