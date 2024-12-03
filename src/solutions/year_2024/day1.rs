use std::collections::HashMap;

pub fn part1(input: crate::Input) -> i32 {
    let (mut l, mut r) = parse(input);
    l.sort();
    r.sort();

    let mut d = 0;
    for (x, y) in l.into_iter().zip(r) {
        d += (y as i32 - x as i32).abs();
    }

    d
}

pub fn part2(input: crate::Input) -> u32 {
    let (l, r) = parse(input);
    let mut counts: HashMap<_, _> = l.into_iter().map(|n| (n, 0)).collect();

    for n in r {
        if let Some(count) = counts.get_mut(&n) {
            *count += 1;
        }
    }

    let mut s = 0;
    for (n, count) in counts {
        s += n * count;
    }

    s
}

fn parse(input: crate::Input) -> (Vec<u32>, Vec<u32>) {
    let mut l = Vec::new();
    let mut r = Vec::new();

    for line in input.read_lines() {
        let (x, y) = line.split_once(' ').unwrap();
        let x: u32 = x.trim().parse().unwrap();
        let y: u32 = y.trim().parse().unwrap();
        l.push(x);
        r.push(y);
    }

    assert_eq!(l.len(), r.len());

    (l, r)
}
