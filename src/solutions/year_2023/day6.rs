use itertools::Itertools;

struct Race {
    time: u64,
    dist_record: u64,
}

impl Race {
    fn parse_as_multiple<'a>(input: String) -> Vec<Self> {
        let mut lines = input.lines();
        let times_line = lines.next().unwrap();
        let times = parse_line_as_multiple(&times_line);
        let distances_line = lines.next().unwrap();
        let distances = parse_line_as_multiple(&distances_line);
        times
            .zip(distances)
            .map(|(time, dist_record)| Self { time, dist_record })
            .collect()
    }

    fn parse_as_one(input: String) -> Self {
        let mut lines = input.lines();
        let time_line = lines.next().unwrap();
        let time = parse_line_as_one(&time_line);
        let distance_line = lines.next().unwrap();
        let distance = parse_line_as_one(&distance_line);
        Self {
            time,
            dist_record: distance,
        }
    }

    fn ways_to_beat_record(&self) -> u64 {
        let mut count = 0;
        for i in 1..self.time {
            let dist = (self.time - i) * i;
            if dist > self.dist_record {
                count += 1;
            }
        }
        count
    }
}

fn parse_line_as_multiple<'a>(line: &'a str) -> impl Iterator<Item = u64> + 'a {
    line.split_whitespace()
        .skip(1)
        .map(str::parse)
        .map(Result::unwrap)
}

fn parse_line_as_one(line: &str) -> u64 {
    line.split_whitespace().skip(1).join("").parse().unwrap()
}

fn solve_part1(input: String) -> u64 {
    Race::parse_as_multiple(input)
        .iter()
        .map(Race::ways_to_beat_record)
        .product()
}

fn solve_part2(input: String) -> u64 {
    Race::parse_as_one(input).ways_to_beat_record()
}

pub fn solve(input: crate::Input) {
    let input = input.read_all();
    let answer1 = solve_part1(input.clone());
    let answer2 = solve_part2(input);
    println!("Answer #1: {answer1}");
    println!("Answer #2: {answer2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_input() {
        let input = r#"
Time:      7  15   30
Distance:  9  40  200
"#
        .trim();

        let answer1 = solve_part1(input.to_owned());
        let answer2 = solve_part2(input.to_owned());
        assert_eq!(answer1, 288);
        assert_eq!(answer2, 71503);
    }
}
