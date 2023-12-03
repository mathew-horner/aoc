use std::collections::HashSet;

use lazy_static::lazy_static;
use maplit::hashset;

lazy_static! {
    /// The set of symbols that indicate adjacent part numbers.
    static ref SYMBOLS: HashSet<char> = hashset! {
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{', '}',
        ';', ':', '\'', '"', ',', '<', '>', '/', '?', '\\', '|'
    };
}

/// A 2D engine schematic, parsed from the challenge input.
struct Schematic {
    table: Vec<Vec<Cell>>,
    part_numbers: Vec<PartNumber>,
}

impl Schematic {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut table = Vec::new();
        let mut parser = Parser::default();

        for (row, line) in lines.enumerate() {
            let mut table_row = Vec::new();
            for (col, ch) in line.chars().enumerate() {
                let location = Location::new(row, col);
                let cell = parser.process(ch, location);
                table_row.push(cell);
            }
            parser.pop_buffer();
            table.push(table_row);
        }

        Self {
            table,
            part_numbers: parser.part_numbers,
        }
    }

    fn rows(&self) -> usize {
        self.table.len()
    }

    fn cols(&self) -> usize {
        self.table[0].len()
    }

    /// If a digit exists at the given location, returns it.
    fn digit_at(&self, location: &Location) -> Option<Digit> {
        self.table[location.row][location.col].value.as_digit()
    }

    /// Performs an inspection of the engine schematic, yielding pertinent information such as the
    /// part numbers and gear ratios, per the challenge specification.
    fn inspect(&self) -> Inspection {
        let mut part_numbers = HashSet::new();
        let mut gear_ratios = Vec::new();

        let symbols: Vec<_> = self
            .table
            .iter()
            .flatten()
            .filter(|cell| matches!(cell.value, Value::Symbol))
            .collect();

        for symbol in symbols {
            let mut adjacent_part_numbers = HashSet::new();
            for adjacency in symbol.location.adjacent(self) {
                if let Some(digit) = self.digit_at(&adjacency) {
                    adjacent_part_numbers.insert(&self.part_numbers[digit.part_number_idx]);
                }
            }

            // Per the challenge description, any symbol with 2 adjacent part numbers is a gear.
            // As part of the inspection, we need to collect all the gear *ratios*, which is the
            // product of the two adjacent part numbers.
            if adjacent_part_numbers.len() == 2 {
                gear_ratios.push(
                    adjacent_part_numbers
                        .iter()
                        .map(|part_number| part_number.value)
                        .product(),
                );
            }

            part_numbers.extend(adjacent_part_numbers);
        }

        Inspection {
            gear_ratios,
            part_numbers: part_numbers
                .into_iter()
                .map(|part_number| part_number.value)
                .collect(),
        }
    }
}

/// Helps track state while parsing a [`Schematic`].
#[derive(Default)]
struct Parser {
    part_numbers: Vec<PartNumber>,
    buffer: Option<PartNumber>,
}

impl Parser {
    fn process(&mut self, ch: char, location: Location) -> Cell {
        let value = if let Some(digit) = ch.to_digit(10) {
            if let Some(curr) = self.buffer.as_mut() {
                curr.value = curr.value * 10 + digit;
            } else {
                self.buffer = Some(PartNumber::new(digit, location.clone()));
            };

            // This assumes that our code will later call Parser::pop_buffer and the part number
            // currently in the buffer will be at this index.
            Value::Digit(Digit::new(digit, self.part_numbers.len()))
        } else {
            self.pop_buffer();
            if ch == '.' {
                Value::Empty
            } else if SYMBOLS.contains(&ch) {
                Value::Symbol
            } else {
                panic!("invalid character found in input: {ch}");
            }
        };

        Cell::new(value, location)
    }

    /// If there is currently a part number in the buffer, pop it and push it to the set of part
    /// number results.
    fn pop_buffer(&mut self) {
        if let Some(part_number) = self.buffer.take() {
            self.part_numbers.push(part_number);
        }
    }
}

struct Cell {
    value: Value,
    location: Location,
}

impl Cell {
    fn new(value: Value, location: Location) -> Self {
        Self { value, location }
    }
}

enum Value {
    Digit(Digit),
    Symbol,
    Empty,
}

impl Value {
    /// If this value is a digit, returns it.
    fn as_digit(&self) -> Option<Digit> {
        if let Self::Digit(digit) = self {
            Some(digit.clone())
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct Digit {
    // NOTE: We technically don't need this for our implementation, but it can be helpful for debugging.
    #[allow(dead_code)]
    value: u32,
    /// A pointer to the [`PartNumber`] that this digit is a part of inside the
    /// `Schematic::part_numbers` vector.
    part_number_idx: usize,
}

impl Digit {
    fn new(value: u32, part_number_idx: usize) -> Self {
        Self {
            value,
            part_number_idx,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct PartNumber {
    value: u32,
    start: Location,
}

impl PartNumber {
    fn new(value: u32, start: Location) -> Self {
        Self { value, start }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Location {
    row: usize,
    col: usize,
}

impl Location {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Returns all the *valid* `Location`s that are adjacent to this location.
    fn adjacent(&self, schematic: &Schematic) -> Vec<Self> {
        [
            self.with_offset(schematic, -1, -1),
            self.with_offset(schematic, -1, 0),
            self.with_offset(schematic, -1, 1),
            self.with_offset(schematic, 0, -1),
            self.with_offset(schematic, 0, 1),
            self.with_offset(schematic, 1, -1),
            self.with_offset(schematic, 1, 0),
            self.with_offset(schematic, 1, 1),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    /// Returns a new `Location` instance with the given offsets applied.
    ///
    /// If the new location would be out of bounds, returns None.
    fn with_offset(&self, schematic: &Schematic, row: i32, col: i32) -> Option<Self> {
        let row = usize::try_from(self.row as i32 - row).ok()?;
        let col = usize::try_from(self.col as i32 - col).ok()?;
        if row >= schematic.rows() || col >= schematic.cols() {
            return None;
        }
        Some(Self { row, col })
    }
}

struct Inspection {
    part_numbers: Vec<u32>,
    gear_ratios: Vec<u32>,
}

fn run<'a>(lines: impl Iterator<Item = &'a str>) -> (u32, u32) {
    let schematic = Schematic::parse(lines);
    let inspection = schematic.inspect();
    (
        inspection.part_numbers.iter().sum(),
        inspection.gear_ratios.iter().sum(),
    )
}

pub fn solve(input: crate::Input) {
    let input = input.read_all();
    let (answer1, answer2) = run(input.lines());
    println!("Answer #1: {answer1}");
    println!("Answer #2: {answer2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"
12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56
"#
        .trim();

        let (answer1, answer2) = run(input.lines());
        assert_eq!(answer1, 413);
        assert_eq!(answer2, 6756);
    }
}
