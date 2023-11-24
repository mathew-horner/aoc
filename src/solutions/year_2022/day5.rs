use itertools::Itertools;

pub fn solve(input: crate::Input) {
    let input = input.read_all();
    let blocks: Vec<_> = input.split("\n\n").collect();
    let mut stacks1: Stacks = Stacks::parse_block(blocks[0]);
    let mut stacks2 = stacks1.clone();
    let commands = Command::parse_block(blocks[1]);
    for command in commands {
        stacks1.execute_command(&command, false);
        stacks2.execute_command(&command, true);
    }
    let answer1: String = stacks1.peek_all().collect();
    let answer2: String = stacks2.peek_all().collect();
    println!("Answer #1: {answer1}");
    println!("Answer #2: {answer2}");
}

#[derive(Clone)]
struct Stacks {
    inner: Vec<Vec<char>>,
}

impl Stacks {
    fn new(count: usize) -> Self {
        Self {
            inner: (0..count).into_iter().map(|_| Vec::new()).collect(),
        }
    }

    fn parse_block(s: &str) -> Self {
        let mut lines: Vec<_> = s.lines().collect();
        let mut stacks = Self::preallocate(&mut lines);
        lines.reverse();
        for line in lines {
            for (col, chunk) in line.chars().chunks(4).into_iter().enumerate() {
                let chunk: Vec<_> = chunk.collect();
                if chunk[1] != ' ' {
                    stacks.inner[col].push(chunk[1]);
                }
            }
        }
        stacks
    }

    /// Create a new instance with the inner vector pre-allocated with the correct number of stacks
    /// as determined by the challenge input, also pop that line from the input as it won't be
    /// needed anymore.
    fn preallocate(lines: &mut Vec<&str>) -> Self {
        let last_line = lines.pop().unwrap();
        let numbers: Vec<_> = last_line.split_whitespace().collect();
        Self::new(numbers.len())
    }

    fn peek_all(&self) -> impl Iterator<Item = char> + '_ {
        self.inner.iter().map(|stack| {
            // NOTE: This assumes that each stack is non-empty, which is true for our input but
            // makes this implementation a bit more brittle.
            *stack.last().unwrap()
        })
    }

    /// While executing a command, each box is moved individually. This results in the order of
    /// crates to flip when moved to the destination stack unless `preserve_order` is `true`.
    fn execute_command(&mut self, command: &Command, preserve_order: bool) {
        let mut boxes = Vec::new();
        for _ in 0..command.count {
            // NOTE: This assumes a valid command.
            let box_ = self.inner[command.from].pop().unwrap();
            boxes.push(box_);
        }
        if preserve_order {
            boxes.reverse();
        }
        self.inner[command.to].extend(boxes);
    }
}

/// Describes a movement of crates from one stack to another.
struct Command {
    from: usize,
    to: usize,
    count: usize,
}

impl Command {
    fn parse_line(line: &str) -> Self {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let count: usize = tokens[1].parse().unwrap();
        let from = tokens[3].parse::<usize>().unwrap() - 1;
        let to = tokens[5].parse::<usize>().unwrap() - 1;
        Self { from, to, count }
    }

    fn parse_block(s: &str) -> Vec<Self> {
        s.lines().map(Self::parse_line).collect()
    }
}
