use regex::{Match, Regex};

pub fn part1(input: crate::Input) -> u32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let input = input.read_all();
    let mut sum = 0;
    for captures in regex.captures_iter(&input) {
        let x: u32 = captures[1].parse().unwrap();
        let y: u32 = captures[2].parse().unwrap();
        sum += x * y;
    }
    sum
}

pub fn part2(input: crate::Input) -> u32 {
    let regex = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)").unwrap();
    let input = input.read_all();
    let mut sum = 0;
    let mut do_ = true;
    for captures in regex.captures_iter(&input) {
        if captures.get(1).as_ref().map(Match::as_str) == Some("mul") {
            if !do_ {
                continue;
            }
            let x: u32 = captures[2].parse().unwrap();
            let y: u32 = captures[3].parse().unwrap();
            sum += x * y;
        } else if captures.get(4).as_ref().map(Match::as_str) == Some("do") {
            do_ = true;
        } else if captures.get(5).as_ref().map(Match::as_str) == Some("don't") {
            do_ = false;
        } else {
            panic!("Regex must be wrong!");
        }
    }
    sum
}
