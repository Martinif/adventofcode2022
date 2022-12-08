use adventofcode2022::load_input;

fn prepare_input(s: String) -> Vec<Vec<usize>> {
    s.lines()
        .map(|x| x.chars()
            .map(|tree| tree.to_digit(10).unwrap() as usize)
            .collect())
        .collect()
}

fn part1(forest: &Vec<Vec<usize>>) -> usize {
    let mut visible = 2 * forest.len() + 2 * forest[0].len() - 4; // border trees
    for i in 1..forest.len() - 1 {
        for j in 1..forest[0].len() - 1 {
            let height = forest[i][j];
            if forest[i][..j].iter().all(|t| t < &height) // left
                || forest[i][j + 1..].iter().all(|t| t < &height) // right
                || forest.iter().enumerate().all(|(idx, row)| idx >= i || row[j] < height) // top
                || forest.iter().enumerate().all(|(idx, row)| idx <= i || row[j] < height) { // bottom
                visible += 1;
            }
        }
    }
    visible
}

fn part2(forest: &Vec<Vec<usize>>) -> usize {
    let mut top_scenic_score = 0;
    for i in 0..forest.len() - 1 {
        for j in 0..forest[0].len() - 1 {
            let height = forest[i][j];
            let current_score =
                visible_trees(forest[i][..j].iter().rev().collect(), height) // left
                    * visible_trees(forest[i][j + 1..].iter().collect(), height) // right
                    * visible_trees(forest.iter().enumerate().filter(|(idx, _)| idx < &i).map(|(_, row)| &row[j]).rev().collect(), height) // top
                    * visible_trees(forest.iter().enumerate().filter(|(idx, _)| idx > &i).map(|(_, row)| &row[j]).collect(), height); // bottom
            if current_score > top_scenic_score {
                top_scenic_score = current_score;
            }
        }
    }
    top_scenic_score
}

fn visible_trees(t: Vec<&usize>, height: usize) -> usize {
    let mut visible = t.iter().take_while(|t| **t < &height).count();
    if visible != t.len() { visible += 1 }
    visible
}

fn main() {
    let data = load_input("input/08.txt");
    let forest = prepare_input(data);
    println!("First task: {:?}", part1(&forest));
    println!("Second task: {:?}", part2(&forest));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = load_input("input/08.test.txt");
        let forest = prepare_input(data);
        assert_eq!(part1(&forest), 21);
    }

    #[test]
    fn test_part2() {
        let data = load_input("input/08.test.txt");
        let forest = prepare_input(data);
        assert_eq!(part2(&forest), 8);
    }
}