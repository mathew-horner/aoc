use std::ops::Range;
use std::str::pattern::Pattern;

use regex::Regex;

#[derive(Debug)]
struct Conversion {
    source: Range<u64>,
    destination: Range<u64>,
}

impl Conversion {
    fn new(source: Range<u64>, destination: Range<u64>) -> Self {
        Self {
            source,
            destination,
        }
    }

    fn parse(line: &str) -> Self {
        let mut tokens = line.split_whitespace();
        let destination: u64 = tokens.next().unwrap().parse().unwrap();
        let source: u64 = tokens.next().unwrap().parse().unwrap();
        let range: u64 = tokens.next().unwrap().parse().unwrap();
        Self::new(source..source + range, destination..destination + range)
    }

    fn map(&self, input: u64) -> Option<u64> {
        (self.source.contains(&input)).then(|| {
            let delta = input - self.source.start;
            self.destination.start + delta
        })
    }

    fn map_range(&self, input: &Range<u64>) -> Overlap {
        let overlap = Overlap::relative(&self.source, &input);
        let inner = overlap.inner.map(|inner| {
            let base = self.destination.start + inner.start;
            base..base + inner.count() as u64
        });
        Overlap {
            inner,
            oob_lo: overlap.oob_lo,
            oob_hi: overlap.oob_hi,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Overlap {
    inner: Option<Range<u64>>,
    oob_lo: Option<Range<u64>>,
    oob_hi: Option<Range<u64>>,
}

impl Overlap {
    fn absolute(source: &Range<u64>, candidate: &Range<u64>) -> Self {
        let mut overlap = Self::default();
        let mut inner = candidate.clone();

        if candidate.start < source.start {
            inner.start = source.start;
            let end = source.start.min(candidate.end);
            let oob_lo = candidate.start..end;
            if oob_lo.end > oob_lo.start {
                overlap.oob_lo = Some(oob_lo);
            }
        }

        if candidate.end > source.end {
            inner.end = source.end;
            let start = source.end.max(candidate.start);
            let oob_hi = start..candidate.end;
            if oob_hi.end > oob_hi.start {
                overlap.oob_hi = Some(oob_hi);
            }
        }

        overlap.inner = (inner.end > inner.start).then_some(inner);
        overlap
    }

    fn relative(source: &Range<u64>, candidate: &Range<u64>) -> Self {
        let absolute = Self::absolute(source, candidate);
        let inner = absolute
            .inner
            .map(|inner| inner.start - source.start..inner.end - source.start);

        Overlap {
            inner,
            // We just leave the OOB ranges alone, they won't be transformed anyways.
            oob_lo: absolute.oob_lo,
            oob_hi: absolute.oob_hi,
        }
    }
}

fn parse_blocks<'a>(input: &'a str) -> impl Iterator<Item = &'a str> {
    let block_header_regex = Regex::new(".*:\n?").unwrap();
    input
        .split("\n\n")
        .map(move |block| block_header_regex.strip_prefix_of(block))
        .map(Option::unwrap)
        .map(str::trim)
}

fn parse_header_part1(header: &str) -> Vec<u64> {
    header
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn parse_header_part2(header: &str) -> Vec<Range<u64>> {
    parse_header_part1(header)
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0];
            let length = chunk[1];
            start..start + length
        })
        .collect()
}

fn solve_part1(input: String) -> u64 {
    let mut blocks = parse_blocks(&input);
    let mut state: Vec<u64> = parse_header_part1(blocks.next().unwrap());

    for block in blocks {
        let conversions: Vec<_> = block.lines().map(Conversion::parse).collect();
        let prev_state = state.clone();
        for conversion in &conversions {
            for (idx, value) in state.iter_mut().enumerate() {
                if let Some(next) = conversion.map(prev_state[idx]) {
                    *value = next;
                }
            }
        }
    }

    state.sort();
    state[0]
}

fn solve_part2(input: String) -> u64 {
    let mut blocks = parse_blocks(&input);
    let mut state: Vec<Range<u64>> = parse_header_part2(blocks.next().unwrap());

    for block in blocks {
        let conversions: Vec<_> = block.lines().map(Conversion::parse).collect();
        let mut prev_state = state.clone();
        let mut next_state: Vec<_> = state.iter().map(|value| (value.clone(), false)).collect();

        for conversion in &conversions {
            let mut idx = 0;
            while idx < next_state.len() {
                if !next_state[idx].1 {
                    let overlap = conversion.map_range(&prev_state[idx]);
                    if let Some(inner) = overlap.inner {
                        next_state[idx] = (inner, true);
                        if let Some(oob_lo) = overlap.oob_lo {
                            prev_state.push(oob_lo.clone());
                            next_state.push((oob_lo, false));
                        }
                        if let Some(oob_hi) = overlap.oob_hi {
                            prev_state.push(oob_hi.clone());
                            next_state.push((oob_hi, false));
                        }
                    }
                }

                idx += 1;
            }
        }

        state = next_state.into_iter().map(|tuple| tuple.0).collect();
    }

    let mut state: Vec<_> = state.iter().map(|range| range.start).collect();
    state.sort();
    state[0]
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
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "#
        .trim();

        let answer1 = solve_part1(input.to_string());
        assert_eq!(answer1, 35);

        let answer2 = solve_part2(input.to_string());
        assert_eq!(answer2, 46);
    }
}
