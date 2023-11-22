use std::collections::HashSet;

pub fn solve(input: crate::Input) {
    let input = input.read_all();
    let start_of_packet = find_first_marker(&input, 4).unwrap();
    let start_of_message = find_first_marker(&input, 14).unwrap();
    println!("Answer #1: {}", start_of_packet);
    println!("Answer #2: {}", start_of_message);
}

fn find_first_marker(input: &str, count: usize) -> Option<usize> {
    let chars: Vec<_> = input.chars().collect();
    chars
        .windows(count)
        .enumerate()
        .find(|(_, window)| HashSet::<_>::from_iter(window.iter()).len() == count)
        .map(|(i, _)| i + count)
}
