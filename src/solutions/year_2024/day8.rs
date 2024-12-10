use std::collections::{HashMap, HashSet, VecDeque};

type SafePos = (u32, u32);
type UnsafePos = (i32, i32);

#[derive(Debug)]
struct Input {
    nodes: HashMap<char, Vec<SafePos>>,
    grid_height: usize,
    grid_width: usize,
}

impl Input {
    fn parse(input: crate::Input) -> Self {
        let grid: Vec<Vec<char>> = input.read_lines().map(|line| line.chars().collect()).collect();
        let grid_height = grid.len();
        let grid_width = grid[0].len();
        let mut nodes = HashMap::new();
        for (i, chars) in grid.into_iter().enumerate() {
            for (j, char_) in chars.into_iter().enumerate().filter(|(_, char_)| *char_ != '.') {
                nodes.entry(char_).or_insert_with(|| Vec::new()).push((i as u32, j as u32));
            }
        }
        Self { nodes, grid_height, grid_width }
    }
}

pub fn part1(input: crate::Input) -> usize {
    let input = Input::parse(input);
    let mut antinode_spots = HashSet::new();
    for (_, positions) in &input.nodes {
        for (idx, &pos1) in positions.iter().enumerate() {
            for &pos2 in positions.iter().skip(idx + 1) {
                let (a1, a2) = antinodes(pos1, pos2);
                antinode_spots.insert(a1);
                antinode_spots.insert(a2);
            }
        }
    }
    antinode_spots
        .into_iter()
        .filter(|&(x, y)| x >= 0 && x < input.grid_height as i32 && y >= 0 && y < input.grid_width as i32)
        .count()
}

fn antinodes(node1: SafePos, node2: SafePos) -> (UnsafePos, UnsafePos) {
    let node1 = (node1.0 as i32, node1.1 as i32);
    let node2 = (node2.0 as i32, node2.1 as i32);
    let dx = node2.0 - node1.0;
    let dy = node2.1 - node1.1;
    let antinode1 = (node1.0 - dx, node1.1 - dy);
    let antinode2 = (node2.0 + dx, node2.1 + dy);
    (antinode1, antinode2)
}

pub fn part2(input: crate::Input) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let input = crate::Input::memory(
            r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#
            .trim(),
        );
        assert_eq!(part1(input), 14);
    }

    #[test]
    fn part2_example() {
        let input = crate::Input::memory(
            r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#
            .trim(),
        );
        assert_eq!(part2(input), 34);
    }
}
