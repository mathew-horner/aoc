use std::collections::BinaryHeap;

pub fn solve(input: crate::Input) {
    let input = input.read_all();
    let blocks = input.split("\n\n").map(str::trim);
    let mut heap: BinaryHeap<u32> = blocks
        .map(|block| {
            block
                .split("\n")
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    let top = heap.pop().unwrap();
    println!("Answer #1: {}", top);

    let top3 = top + heap.pop().unwrap() + heap.pop().unwrap();
    println!("Answer #2: {}", top3);
}
