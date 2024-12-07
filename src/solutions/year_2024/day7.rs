fn parse(input: crate::Input) -> Vec<(u64, Vec<u64>)> {
    input
        .read_lines()
        .map(|line| {
            let (y, terms) = line.split_once(':').unwrap();
            let y = y.parse().unwrap();
            let terms = terms.trim();
            let terms: Vec<_> = terms.split(' ').map(|term| term.parse().unwrap()).collect();
            (y, terms)
        })
        .collect()
}

pub fn part1(input: crate::Input) -> u64 {
    let equations = parse(input);
    let mut total = 0;
    for (y, terms) in equations {
        let mut terms = terms.into_iter();
        let mut values = vec![terms.next().unwrap()];
        for term in terms {
            let mut next = Vec::with_capacity(values.len() * 2);
            for value in values {
                next.push(value * term);
                next.push(value + term);
            }
            values = next;
        }

        if values.contains(&y) {
            total += y
        }
    }
    total
}

pub fn part2(input: crate::Input) -> u64 {
    let equations = parse(input);
    let mut total = 0;
    for (y, terms) in equations {
        let mut terms = terms.into_iter();
        let mut values = vec![terms.next().unwrap()];
        for term in terms {
            let mut next = Vec::with_capacity(values.len() * 2);
            for value in values {
                next.push(format!("{value}{term}").parse().unwrap());
                next.push(value * term);
                next.push(value + term);
            }
            values = next;
        }

        if values.contains(&y) {
            total += y
        }
    }
    total
}
