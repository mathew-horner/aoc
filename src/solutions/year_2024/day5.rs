use std::collections::{HashMap, VecDeque};

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
    let mut sum = 0;

    for update in updates {
        let sorted = sort_update(&update, &pairs);
        if sorted != update {
            sum += middle_page(&sorted);
        }
    }

    sum
}

fn sort_update(update: &Update, pairs: &Vec<Pair>) -> Update {
    let mut graph = Graph::build(&update, &pairs);
    let mut sorted = Vec::with_capacity(update.len());
    let mut buffer: VecDeque<Page> = VecDeque::from_iter(
        update
            .iter()
            .filter(|page| !graph.has_incoming(**page))
            .cloned(),
    );

    while let Some(page) = buffer.pop_back() {
        sorted.push(page);
        for other_page in graph.outgoing(page) {
            graph.remove_edge(page, other_page);
            if !graph.has_incoming(other_page) {
                buffer.push_front(other_page);
            }
        }
    }

    sorted
}

struct Graph<'a> {
    update: &'a Update,
    index_map: HashMap<Page, usize>,
    matrix: Vec<Vec<bool>>,
}

impl<'a> Graph<'a> {
    fn build(update: &'a Update, pairs: &Vec<Pair>) -> Self {
        let index_map: HashMap<Page, usize> = update
            .into_iter()
            .cloned()
            .enumerate()
            .map(|(idx, page)| (page, idx))
            .collect();

        let mut matrix = vec![vec![false; update.len()]; update.len()];
        for (page1, page2) in pairs {
            if let (Some(idx1), Some(idx2)) = (index_map.get(page1), index_map.get(page2)) {
                matrix[*idx1][*idx2] = true;
            }
        }

        Self {
            update,
            index_map,
            matrix,
        }
    }

    fn outgoing(&self, page: Page) -> Vec<Page> {
        let mut output = Vec::new();
        let row = *self.index_map.get(&page).unwrap();
        for col in 0..self.vertex_count() {
            if self.matrix[row][col] {
                output.push(self.update[col]);
            }
        }
        output
    }

    fn remove_edge(&mut self, page1: Page, page2: Page) {
        let idx1 = *self.index_map.get(&page1).unwrap();
        let idx2 = *self.index_map.get(&page2).unwrap();
        self.matrix[idx1][idx2] = false;
    }

    fn has_incoming(&self, page: Page) -> bool {
        let col = *self.index_map.get(&page).unwrap();
        for row in 0..self.vertex_count() {
            if self.matrix[row][col] {
                return true;
            }
        }
        false
    }

    fn vertex_count(&self) -> usize {
        self.update.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_sort() {
        let pairs = vec![
            (47, 53),
            (97, 13),
            (97, 61),
            (97, 47),
            (75, 29),
            (61, 13),
            (75, 53),
            (29, 13),
            (97, 29),
            (53, 29),
            (61, 53),
            (97, 53),
            (61, 29),
            (47, 13),
            (75, 47),
            (97, 75),
            (47, 61),
            (75, 61),
            (47, 29),
            (75, 13),
            (53, 13),
        ];

        assert_eq!(
            sort_update(&vec![75, 97, 47, 61, 53], &pairs),
            vec![97, 75, 47, 61, 53]
        );
        assert_eq!(
            sort_update(&vec![97, 13, 75, 29, 47], &pairs),
            vec![97, 75, 47, 29, 13]
        );
        assert_eq!(sort_update(&vec![61, 13, 29], &pairs), vec![61, 29, 13]);
    }
}
