use std::collections::{HashMap, HashSet};
use std::ops::Range;

struct Card {
    winning: HashSet<u32>,
    you_have: Vec<u32>,
}

impl Card {
    fn parse(line: &str) -> Self {
        // Strip off the leading "Card X:" text, we don't need it.
        let both_token = line.split(':').last().unwrap().trim();
        let tokens: Vec<_> = both_token.split('|').collect();
        let winning = HashSet::from_iter(parse_numbers(tokens[0]));
        let you_have = parse_numbers(tokens[1]);
        Self { winning, you_have }
    }

    fn count_matching(&self) -> usize {
        self.you_have
            .iter()
            .filter(|card| self.winning.contains(card))
            .count()
    }

    fn score(&self) -> u32 {
        let count = self.count_matching();
        if count == 0 {
            return 0;
        }
        2_u32.pow(count as u32 - 1)
    }
}

fn parse_numbers(text: &str) -> Vec<u32> {
    text.split_whitespace()
        .map(|token| token.parse().unwrap())
        .collect()
}

fn count_with_2_rules(
    cards: &[Card],
    won: Range<usize>,
    cache: &mut HashMap<Range<usize>, u32>,
) -> u32 {
    if let Some(sum) = cache.get(&won) {
        return *sum;
    }

    let mut sum = 0;
    for idx in won.clone() {
        let score = cards[idx].count_matching();
        let lo = idx + 1;
        let hi = (lo + score as usize).min(cards.len());
        sum += count_with_2_rules(cards, lo..hi, cache) + 1;
    }

    cache.insert(won, sum);
    sum
}

fn run<'a>(lines: impl Iterator<Item = &'a str>) -> (u32, u32) {
    let cards: Vec<_> = lines.map(Card::parse).collect();
    let answer1: u32 = cards.iter().map(Card::score).sum();
    let answer2: u32 = count_with_2_rules(&cards, 0..cards.len(), &mut HashMap::new());
    (answer1, answer2)
}

pub fn solve(input: crate::Input) {
    let input = input.read_all();
    let (answer1, answer2) = run(input.lines());
    println!("Answer #1: {answer1}");
    println!("Answer #2: {answer2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_input() {
        let input: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#
        .trim();

        let (answer1, answer2) = run(input.lines());
        assert_eq!(answer1, 13);
        assert_eq!(answer2, 30);
    }
}
