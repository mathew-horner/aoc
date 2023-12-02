use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

lazy_static! {
    /// Maps all spelled out digits to their numeric representation.
    static ref SPELLED_DIGIT_MAP: HashMap<String, u32> = hashmap! {
        "zero".into() => 0,
        "one".into() => 1,
        "two".into() => 2,
        "three".into() => 3,
        "four".into() => 4,
        "five".into() => 5,
        "six".into() => 6,
        "seven".into() => 7,
        "eight".into() => 8,
        "nine".into() => 9,
    };
}

pub fn solve(input: crate::Input) {
    let mut all_numbers = Vec::new();
    let mut all_numbers_and_spelled = Vec::new();

    for line in input.read_lines() {
        let mut numbers = Vec::new();
        let mut spelled = Vec::new();

        for (idx, ch) in line.chars().enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                numbers.push((idx, digit));
            }
        }

        for (name, digit) in SPELLED_DIGIT_MAP.iter() {
            for (idx, _) in line.match_indices(name) {
                spelled.push((idx, *digit));
            }
        }

        numbers.sort_by_key(|t| t.0);
        spelled.sort_by_key(|t| t.0);

        let numbers_and_spelled = merge_sorted_digit_arrays(&numbers, &spelled);
        let numbers: Vec<_> = numbers.into_iter().map(|t| t.1).collect();

        all_numbers.push(compose_digits(numbers));
        all_numbers_and_spelled.push(compose_digits(numbers_and_spelled));
    }

    let sum_numbers: u32 = all_numbers.iter().sum();
    let sum_numbers_and_spelled: u32 = all_numbers_and_spelled.iter().sum();
    println!("Answer #1: {sum_numbers}");
    println!("Answer #2: {sum_numbers_and_spelled}");
}

/// Takes the digits and returns the concatenation of the first and last one.
fn compose_digits(digits: impl IntoIterator<Item = u32>) -> u32 {
    let mut digits = digits.into_iter();
    let first = digits
        .next()
        .expect("line should have had at least one digit");

    let second = digits.last().unwrap_or(first);
    (first * 10) + second
}

/// Takes the two arrays of indicies and digits for the line and merges them into one Vec that is
/// sorted by original index.
///
/// The index is stripped away in the output as it is no longer necessary.
fn merge_sorted_digit_arrays(array1: &[(usize, u32)], array2: &[(usize, u32)]) -> Vec<u32> {
    let mut merged = Vec::with_capacity(array1.len() + array2.len());
    let mut p1 = 0;
    let mut p2 = 0;

    while p1 < array1.len() && p2 < array2.len() {
        if array1[p1].0 < array2[p2].0 {
            merged.push(array1[p1].1);
            p1 += 1;
        } else {
            merged.push(array2[p2].1);
            p2 += 1;
        }
    }

    while p1 < array1.len() {
        merged.push(array1[p1].1);
        p1 += 1;
    }

    while p2 < array2.len() {
        merged.push(array2[p2].1);
        p2 += 1;
    }

    merged
}
