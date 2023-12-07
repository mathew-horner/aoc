use std::cmp::Ordering;
use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

lazy_static! {
    static ref UPGRADE_MAP: HashMap<HandClass, HandClass> = hashmap! {
        HandClass::HighCard => HandClass::OnePair,
        HandClass::OnePair => HandClass::ThreeOfAKind,
        HandClass::TwoPair => HandClass::FullHouse,
        HandClass::ThreeOfAKind => HandClass::FourOfAKind,
        HandClass::FourOfAKind => HandClass::FiveOfAKind,
    };
}

#[derive(Debug)]
struct HandWager {
    hand: Hand,
    wager: u32,
}

impl HandWager {
    fn parse(input: &str, j_as: Card) -> Self {
        let mut parts = input.split_whitespace();
        let hand = Hand::parse(parts.next().unwrap(), j_as);
        let wager = parts.next().unwrap().parse().unwrap();
        Self { hand, wager }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn parse(input: &str, j_as: Card) -> Self {
        let mut cards = [Card::Two; 5];
        for (i, ch) in input.chars().enumerate() {
            cards[i] = Card::parse(ch, j_as);
        }
        Self { cards }
    }

    fn classify(&self) -> HandClass {
        let mut counts = HashMap::new();
        for card in self.cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        let jacks = counts.remove(&Card::Joker).unwrap_or(0);
        let mut counts: Vec<_> = counts.values().collect();
        counts.sort();
        counts.reverse();

        match counts.as_slice() {
            &[5] => HandClass::FiveOfAKind,
            &[4, ..] => HandClass::FourOfAKind,
            &[3, 2] => HandClass::FullHouse,
            &[3, ..] => HandClass::ThreeOfAKind,
            &[2, 2, ..] => HandClass::TwoPair,
            &[2, ..] => HandClass::OnePair,
            &[..] => HandClass::HighCard,
        }
        .upgrade(jacks)
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord, strum::EnumString)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn parse(ch: char, j_as: Self) -> Self {
        match ch {
            'J' => j_as,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("invalid card character"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum HandClass {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandClass {
    fn upgrade(self, times: u32) -> Self {
        let mut curr = self;
        for _ in 0..times {
            curr = match UPGRADE_MAP.get(&curr) {
                Some(next) => *next,
                None => break,
            };
        }
        curr
    }
}

fn run(input: String, parse_line: impl Fn(&str) -> HandWager) -> u32 {
    let mut hand_wagers: Vec<_> = input.lines().map(parse_line).collect();

    hand_wagers.sort_by(|a, b| match a.hand.classify().cmp(&b.hand.classify()) {
        Ordering::Equal => {
            for idx in 0_usize..5 {
                match a.hand.cards[idx].cmp(&b.hand.cards[idx]) {
                    Ordering::Equal => {}
                    other => return other,
                };
            }
            unreachable!();
        }
        other => other,
    });

    hand_wagers
        .into_iter()
        .enumerate()
        .map(|(idx, hw)| hw.wager * (idx as u32 + 1))
        .sum()
}

pub fn solve(input: crate::Input) {
    let input = input.read_all();
    let answer1 = run(input.clone(), |line| HandWager::parse(line, Card::Jack));
    let answer2 = run(input.clone(), |line| HandWager::parse(line, Card::Joker));
    println!("Answer #1: {answer1}");
    println!("Answer #2: {answer2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_input() {
        let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#
        .trim();

        let answer2 = run(input.to_string(), |line| {
            HandWager::parse(line, Card::Joker)
        });
        assert_eq!(answer2, 5905);
    }
}
