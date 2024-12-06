use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use rayon::prelude::*;

type Position = (i32, i32);

struct Input {
    obstacles: HashSet<Position>,
    guard: Position,
    map_height: usize,
    map_width: usize,
}

impl Input {
    fn parse(input: crate::Input) -> Self {
        let lines: Vec<_> = input.read_lines().collect();
        let map_height = lines.len();
        let map_width = lines[0].len();
        let mut obstacles = HashSet::new();
        let mut guard: Option<Position> = None;

        for (row, line) in lines.into_iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        obstacles.insert((row as i32, col as i32));
                    }
                    '^' => guard = Some((row as i32, col as i32)),
                    _ => {}
                };
            }
        }

        Self {
            obstacles,
            guard: guard.unwrap(),
            map_height,
            map_width,
        }
    }
}

pub fn part1(input: crate::Input) -> usize {
    walk(&Input::parse(input)).len()
}

fn walk(input: &Input) -> HashSet<Position> {
    let mut walked = HashSet::new();
    let mut guard = input.guard;

    // N = 0, E = 1, S = 2, W = 3
    let mut direction = 0;

    while guard.0 >= 0
        && guard.0 < input.map_height as i32
        && guard.1 >= 0
        && guard.1 < input.map_width as i32
    {
        walked.insert(guard);
        guard = update_pos(guard, &mut direction, &input.obstacles);
    }

    walked
}

fn update_pos(pos: Position, direction: &mut usize, obstacles: &HashSet<Position>) -> Position {
    loop {
        let pos = match direction {
            0 => (pos.0 - 1, pos.1),
            1 => (pos.0, pos.1 + 1),
            2 => (pos.0 + 1, pos.1),
            3 => (pos.0, pos.1 - 1),
            _ => unreachable!(),
        };

        if obstacles.contains(&pos) {
            *direction = (*direction + 1) % 4;
        } else {
            break pos;
        }
    }
}

pub fn part2(input: crate::Input) -> usize {
    let input = Input::parse(input);
    let path = walk(&input);
    let sum = Arc::new(AtomicUsize::new(0));
    path.par_iter().for_each({
        let sum = sum.clone();
        move |&(row, col)| {
            let row = row as i32;
            let col = col as i32;
            let mut obstacles = input.obstacles.clone();
            if (row, col) == input.guard || !obstacles.insert((row, col)) {
                return;
            }

            let mut guard = input.guard;
            let mut direction = 0;
            let mut seen = HashSet::new();

            while guard.0 >= 0
                && guard.0 < input.map_height as i32
                && guard.1 >= 0
                && guard.1 < input.map_width as i32
            {
                if !seen.insert((guard, direction)) {
                    sum.fetch_add(1, Ordering::SeqCst);
                    break;
                }
                guard = update_pos(guard, &mut direction, &obstacles);
            }
        }
    });
    sum.load(Ordering::SeqCst)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2_example() {
        let input = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#
        .trim();

        let input = crate::Input::memory(input);
        let answer = part2(input);
        assert_eq!(answer, 6);
    }
}
