fn parse(input: crate::Input) -> Vec<(u64, Vec<u64>)> {
    input
        .read_lines()
        .map(|line| {
            let (y, terms) = line.split_once(':').unwrap();
            let terms: Vec<_> = terms.trim().split(' ').map(|term| term.parse().unwrap()).collect();
            (y.parse().unwrap(), terms)
        })
        .collect()
}

pub fn part1(input: crate::Input) -> u64 {
    solve(input, false)
}

pub fn part2(input: crate::Input) -> u64 {
    solve(input, true)
}

fn solve(input: crate::Input, use_concatenation: bool) -> u64 {
    let all_results = parse(input).into_iter().map(|(y, terms)| {
        let seed = terms[0];
        #[rustfmt::skip]
        let results = terms.into_iter().skip(1).fold(vec![seed], |prev, term| {
            prev.into_iter()
                .flat_map(|value| [
                    use_concatenation.then(|| format!("{value}{term}").parse().unwrap()),
                    Some(value * term),
                    Some(value + term),
                ])
                .flatten()
                .collect()
        });
        (y, results)
    });

    all_results.filter_map(|(y, results)| results.contains(&y).then_some(y)).sum()
}
