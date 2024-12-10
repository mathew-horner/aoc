use std::cell::RefCell;
use std::collections::VecDeque;

type FileId = u32;

struct Block {
    data: VecDeque<FileId>,
    size: usize,
}

impl Block {
    fn filled(file_id: FileId, size: usize) -> Self {
        Self { data: vec![file_id; size].into(), size }
    }

    fn empty(size: usize) -> Self {
        Self { data: VecDeque::with_capacity(size), size }
    }

    fn move_to(&mut self, sink: &mut Block) -> bool {
        while !self.data.is_empty() && !sink.is_full() {
            let file_id = self.data.pop_front().unwrap();
            sink.data.push_back(file_id);
        }
        self.data.is_empty()
    }

    fn is_full(&self) -> bool {
        self.free_space() == 0
    }

    fn free_space(&self) -> usize {
        self.size - self.data.len()
    }
}

fn parse(input: crate::Input) -> Vec<RefCell<Block>> {
    let mut is_file = true;
    let mut file_id = 0;
    input
        .read_all()
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .map(|digit| {
            let block = if is_file {
                let block = Block::filled(file_id, digit as usize);
                file_id += 1;
                block
            } else {
                Block::empty(digit as usize)
            };
            is_file = !is_file;
            block
        })
        .map(RefCell::new)
        .collect()
}

fn checksum(blocks: &Vec<RefCell<Block>>) -> u64 {
    let mut position: usize = 0;
    let mut checksum = 0;
    for block in blocks.into_iter() {
        let block = block.borrow();
        for &file_id in &block.data {
            checksum += position as u64 * file_id as u64;
            position += 1;
        }
        position += block.size - block.data.len();
    }
    checksum
}

pub fn part1(input: crate::Input) -> u64 {
    let blocks = parse(input);
    let mut low = 1;
    let mut high = blocks.len() - 1;

    while high > low {
        if blocks[high].borrow_mut().move_to(&mut blocks[low].borrow_mut()) {
            high -= 1;
        } else {
            low += 1;
        }
    }

    checksum(&blocks)
}

pub fn part2(input: crate::Input) -> u64 {
    let blocks = parse(input);
    for (low, block) in blocks.iter().enumerate().rev() {
        for sink in &blocks[..low] {
            let mut sink = sink.borrow_mut();
            let mut block = block.borrow_mut();
            if sink.free_space() >= block.data.len() {
                block.move_to(&mut sink);
            }
        }
    }
    checksum(&blocks)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let input = crate::Input::memory("2333133121414131402");
        assert_eq!(part1(input), 1928);
    }

    #[test]
    fn part2_example() {
        let input = crate::Input::memory("2333133121414131402");
        assert_eq!(part2(input), 2858);
    }
}
