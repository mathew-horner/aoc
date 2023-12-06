use itertools::Itertools;

struct Race {
    time: u64,
    dist_record: u64,
}

impl Race {
    fn parse_as_multiple<'a>(input: String) -> Vec<Self> {
        let mut parsed = input.lines().map(parse_line_as_multiple);
        let times = parsed.next().unwrap();
        let distances = parsed.next().unwrap();
        times
            .zip(distances)
            .map(|(time, dist_record)| Self { time, dist_record })
            .collect()
    }

    fn parse_as_one(input: String) -> Self {
        let mut parsed = input.lines().map(parse_line_as_one);
        let time = parsed.next().unwrap();
        let distance = parsed.next().unwrap();
        Self {
            time,
            dist_record: distance,
        }
    }

    fn ways_to_beat_record(&self) -> u64 {
        (1..self.time).fold(0, |mut acc, epoch| {
            if (self.time - epoch) * epoch > self.dist_record {
                acc += 1
            }
            acc
        })
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
