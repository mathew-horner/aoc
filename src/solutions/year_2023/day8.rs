use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new("([A-Z]{3}) = \\(([A-Z]{3}), ([A-Z]{3})\\)").unwrap();
}

struct Map<'a> {
    instructions: Vec<char>,
    map: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Map<'a> {
    fn parse(input: &'a str) -> Self {
        let mut lines = input.lines();
        let instructions: Vec<_> = lines.next().unwrap().chars().collect();
        // One blank line to throw away.
        _ = lines.next().unwrap();

        let map = lines
            .map(|line| {
                let (_, [key, left, right]) = LINE_REGEX.captures(line).unwrap().extract();
                (key, (left, right))
            })
            .collect();

        Self { instructions, map }
    }

    fn steps_to(&self, from: &str, to: impl Fn(&str) -> bool) -> u64 {
        let mut counter = 0;
        let mut curr = from;

        while !to(curr) {
            let idx = counter % self.instructions.len();
            let instruction = self.instructions[idx];
            let entry = self.map.get(curr).unwrap();
            curr = match instruction {
                'L' => entry.0,
                'R' => entry.1,
                _ => unreachable!(),
            };
            counter += 1;
        }

        counter as u64
    }
}

fn solve_part1(input: &str) -> u64 {
    Map::parse(input).steps_to("AAA", |curr| curr == "ZZZ")
}

fn solve_part2(input: &str) -> u64 {
    let map = Map::parse(input);
    lcm(map.map.keys().filter_map(|key| {
        if key.ends_with('A') {
            Some(map.steps_to(key, |curr| curr.ends_with('Z')))
        } else {
            None
        }
    }))
}

fn lcm(mut iter: impl Iterator<Item = u64>) -> u64 {
    let init = iter.next().unwrap();
    iter.fold(init, |acc, num| num::integer::lcm(acc, num))
}

pub fn solve(input: crate::Input) {
    let input = input.read_all();
    let answer1 = solve_part1(&input);
    println!("Answer #1: {answer1}");
    let answer2 = solve_part2(&input);
    println!("Answer #2: {answer2}");
}
