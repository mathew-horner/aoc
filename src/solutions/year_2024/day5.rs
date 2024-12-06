use std::collections::HashSet;

type Page = u32;
type Pair = (Page, Page);
type Update = Vec<Page>;

fn parse(input: crate::Input) -> (Vec<Pair>, Vec<Update>) {
    let input = input.read_all();
    let (section1, section2) = input.split_once("\n\n").unwrap();
    let section1 = section1.trim();
    let section2 = section2.trim();

    let pairs: Vec<_> = section1
        .lines()
        .map(|line| {
            let (page1, page2) = line.split_once('|').unwrap();
            let page1: Page = page1.parse().unwrap();
            let page2: Page = page2.parse().unwrap();
            (page1, page2)
        })
        .collect();

    let updates: Vec<_> = section2
        .lines()
        .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect())
        .collect();

    (pairs, updates)
}

pub fn part1(input: crate::Input) -> u32 {
    let (pairs, updates) = parse(input);
    let mut sum = 0;

    for update in updates {
        if pairs.iter().all(|pair| check(&update, *pair)) {
            sum += middle_page(&update);
        }
    }

    sum
}

fn check(update: &Update, pair: Pair) -> bool {
    let mut idxs: (Option<usize>, Option<usize>) = (None, None);
    for (idx, &page) in update.into_iter().enumerate() {
        if page == pair.0 {
            idxs.0 = Some(idx);
        } else if page == pair.1 {
            idxs.1 = Some(idx);
        }
    }

    if let (Some(idx1), Some(idx2)) = idxs {
        return idx1 < idx2;
    }

    true
}

fn middle_page(update: &Update) -> Page {
    update[update.len() / 2]
}

pub fn part2(input: crate::Input) -> u32 {
    let (pairs, updates) = parse(input);
    let pairs: HashSet<_> = HashSet::from_iter(&pairs);
    let mut sum = 0;

    for update in updates {
        let mut sorted = update.clone();
        sorted.sort_by(|&a, &b| {
            if pairs.contains(&(a, b)) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        if sorted != update {
            sum += middle_page(&sorted);
        }
    }

    sum
}
