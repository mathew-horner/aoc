use std::ops::RangeInclusive;

pub fn solve(input: crate::Input) {
    let input = input.read_all();
    let mut answer1 = 0;
    let mut answer2 = 0;

    for line in input.lines() {
        let (left, right) = parse_line(&line);
        let overlap = Overlap::compute(&left, &right);
        if overlap >= Overlap::Partial {
            answer2 += 1;
            if overlap == Overlap::Complete {
                answer1 += 1;
            }
        }
    }

    println!("Answer #1: {answer1}");
    println!("Answer #2: {answer2}");
}

fn parse_line(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let (left, right) = line.split_once(",").unwrap();
    (parse_range(left), parse_range(right))
}

fn parse_range(text: &str) -> RangeInclusive<u32> {
    let (left, right) = text.split_once("-").unwrap();
    let left: u32 = left.parse().unwrap();
    let right: u32 = right.parse().unwrap();
    left..=right
}

#[derive(Eq, PartialEq, PartialOrd, Ord, strum::FromRepr)]
#[repr(u8)]
enum Overlap {
    /// The two ranges do not overlap at all.
    None = 0,
    /// The two ranges share some space.
    Partial,
    /// One range completely contains the other.
    Complete,
}

impl Overlap {
    /// Determine what type of overlap exists between the two ranges.
    fn compute(left: &RangeInclusive<u32>, right: &RangeInclusive<u32>) -> Self {
        let from_left = count_in_range(left, right);
        let from_right = count_in_range(right, left);
        Self::from_repr(from_left.max(from_right)).unwrap()
    }
}

/// Returns the amount of points in `left` that are in the range of `right`.
///
/// The minimum return value for this function is 0, and the maximum is 2.
fn count_in_range(left: &RangeInclusive<u32>, right: &RangeInclusive<u32>) -> u8 {
    [right.contains(left.start()), right.contains(left.end())]
        .into_iter()
        .filter(|b| *b)
        .count() as u8
}
