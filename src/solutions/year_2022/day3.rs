use std::collections::HashSet;

fn priority(c: char) -> u32 {
    const UPPERCASE_OFFSET: u32 = 64;
    const LOWERCASE_OFFSET: u32 = 96;
    const ALPHABET_LENGTH: u32 = 26;

    if c >= 'A' && c <= 'Z' {
        return c as u32 - UPPERCASE_OFFSET + ALPHABET_LENGTH;
    }
    if c >= 'a' && c <= 'z' {
        return c as u32 - LOWERCASE_OFFSET;
    }

    panic!("We only know the priority of alphabetical characters");
}

pub fn solve(input: crate::Input) {
    let input = input.read_all();
    let lines: Vec<_> = input.lines().collect();

    let sum_of_priorities_p1: u32 = lines
        .iter()
        .map(|line| {
            let chars: Vec<_> = line.chars().collect();
            let halves: Vec<_> = chars.chunks(chars.len() / 2).collect();
            let first_half: HashSet<char> = HashSet::from_iter(halves[0].to_owned());
            let second_half: HashSet<char> = HashSet::from_iter(halves[1].to_owned());
            let item_in_both = *first_half.intersection(&second_half).next().unwrap();
            priority(item_in_both)
        })
        .sum();

    let sum_of_priorities_p2: u32 = lines
        .chunks(3)
        .map(|line_group| {
            let char_sets: Vec<_> = line_group
                .iter()
                .map(|s| *s)
                .map(str::chars)
                .map(HashSet::<char>::from_iter)
                .collect();

            let char_in_all = *char_sets[0]
                .intersection(
                    &char_sets[1]
                        .intersection(&char_sets[2])
                        .map(ToOwned::to_owned)
                        .collect::<HashSet<char>>(),
                )
                .next()
                .unwrap();

            priority(char_in_all)
        })
        .sum();

    println!("Answer #1: {}", sum_of_priorities_p1);
    println!("Answer #2: {}", sum_of_priorities_p2);
}
