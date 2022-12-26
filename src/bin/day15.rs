use std::collections::{HashMap, HashSet};
use adventofcode2022::load_input;

fn dist (p1 : (i32, i32), p2 : (i32, i32)) -> i32 {
    (p1.0-p2.0).abs() + (p1.1 - p2.1).abs()
}

#[derive(Debug, Clone, PartialEq)]
struct Range {
    start : i32,
    end : i32
}

impl Range {
    // only consider overlapping from one side
    fn isoverlapping(&self, range : &Range) -> bool {
        self.end <= range.end && self.end >= range.start
    }

    fn contains(&self, range : &Range) -> bool {
        self.start<= range.start && self.end>= range.end
    }

}

fn prepare_input(s: String, row : i32) -> (HashMap<(i32,i32), i32>, i32) {
    let mut map = HashMap::new();
    let mut beacons : HashSet<(i32, i32)> = HashSet::new();
    for line in s.lines() {
        let parts :Vec<_>= line.split(" ").collect();
        let s_x: i32 = parts[2].clone().strip_suffix(",").unwrap()[2..].parse().unwrap();
        let s_y: i32 = parts[3].clone().strip_suffix(":").unwrap()[2..].parse().unwrap();
        let b_x: i32 = parts[8].clone().strip_suffix(",").unwrap()[2..].parse().unwrap();
        let b_y: i32 = parts[9].clone()[2..].parse().unwrap();
        map.insert((s_x, s_y), dist((s_x,s_y),(b_x,b_y)));
        if b_y == row {
            beacons.insert((b_x,b_y));
        }
    }
    (map, beacons.len() as i32)
}

fn part1(map: &HashMap<(i32,i32), i32>, beacon_count : i32, row : i32) -> i32 {
    let mut checked = HashSet::new();
    for (sensor, d) in map {
        if dist(*sensor, (sensor.0, row)) > *d { continue; }
        else {
            let restdistance = d-dist(*sensor, (sensor.0, row));
            let end1 = sensor.0+restdistance;
            let end2 = sensor.0-restdistance;
            for pos in end2..=end1 {
                checked.insert(pos);
            }
        }
    }
    checked.len() as i32 - beacon_count
}

fn part2(map: &HashMap<(i32,i32), i32>, limit: i32) -> u64 {
    let full_range: Range = Range{start: 0, end : limit};
    for y in 0..=limit {
        let mut checked = Vec::new();
        for (sensor, d) in map {
            if dist(*sensor, (sensor.0, y)) > *d { continue; }
            else {
                let restdistance = d-dist(*sensor, (sensor.0, y));
                let end1 = sensor.0+restdistance;
                let end2 = sensor.0-restdistance;
                checked.push(Range{start : end2, end : end1})
            }
        }
        // connect ranges
        let mut update_range = true;
        while checked.len()> 1 && update_range{
            update_range = false;
            let mut newchecked = checked.clone();
            for i in 0..checked.len() {
                for j in 0..checked.len() {
                    if i == j {continue}
                    if checked[i].isoverlapping(&checked[j]) {
                        let rangeconnected = Range{start: checked[i].start.min(checked[j].start), end: checked[j].end.max(checked[i].end)};
                        newchecked.retain(|x| *x != checked[i] && *x != checked[j]);
                        newchecked.push(rangeconnected);
                        update_range = true;
                        break;
                    }
                }
                if update_range { break; }
            }
            if update_range {checked = newchecked;}
        }

        if !checked.iter().any(|r|r.contains(&full_range)) {
            // we assume there is exactly one possible location for the distress beacon
            // filter ranges which are not in the possible locations
            checked.retain(|r|r.end>0 && r.start < limit);
            return (checked.iter().map(|r|r.end).min().unwrap() as u64 +1)  * 4000000 + y as u64;
        }
    }
    unreachable!()
}


fn main() {
    let data = load_input("input/15.txt");
    let (map, beaconcount) = prepare_input(data, 2000000);
    println!("First task: {:?}", part1(&map, beaconcount, 2000000));
    println!("Second task: {:?}", part2(&map, 4000000));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/15.test.txt");
        let (map, beacon_count) = prepare_input(data, 10);
        assert_eq!(part1(&map, beacon_count, 10), 26);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/15.test.txt");
        let (map, _beacon_count) = prepare_input(data, 10);
        assert_eq!(part2(&map, 20), 56000011);
    }
}