fn parse(input: crate::Input) -> Vec<Vec<char>> {
    input
        .read_lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part1(input: crate::Input) -> usize {
    const WORD: &str = "XMAS";

    let wordsearch = parse(input);
    let mut sum = 0;

    for r in 0..wordsearch.len() {
        for c in 0..wordsearch[0].len() {
            for direction in [
                Direction::Horizontal,
                Direction::Vertical,
                Direction::ForwardDiagonal,
                Direction::BackwardDiagonal,
            ] {
                if let Some(word) = get_four_letter_word(&wordsearch, r, c, direction) {
                    if word == WORD || &word.chars().rev().collect::<String>() == WORD {
                        sum += 1;
                    }
                }
            }
        }
    }

    sum
}

enum Direction {
    Horizontal,
    Vertical,
    ForwardDiagonal,
    BackwardDiagonal,
}

fn get_four_letter_word(
    wordsearch: &Vec<Vec<char>>,
    r: usize,
    c: usize,
    direction: Direction,
) -> Option<String> {
    let mut word = String::new();
    match direction {
        Direction::Horizontal => {
            for i in 0..4 {
                word.push(*wordsearch.get(r)?.get(c + i)?);
            }
        }
        Direction::Vertical => {
            for i in 0..4 {
                word.push(*wordsearch.get(r + i)?.get(c)?);
            }
        }
        Direction::ForwardDiagonal => {
            for i in 0..4 {
                word.push(*wordsearch.get(r + i)?.get(c + i)?);
            }
        }
        Direction::BackwardDiagonal => {
            for i in 0..4 {
                word.push(
                    *wordsearch
                        .get(r + i)?
                        .get(usize::try_from(c as i32 - i as i32).ok()?)?,
                );
            }
        }
    }
    Some(word)
}

pub fn part2(input: crate::Input) -> usize {
    let wordsearch = parse(input);
    let mut sum = 0;
    for row in 0..=wordsearch.len() - 3 {
        for col in 0..=wordsearch[0].len() - 3 {
            if window_contains_x_mas(&wordsearch, row, col) {
                sum += 1;
            }
        }
    }
    sum
}

fn window_contains_x_mas(wordsearch: &Vec<Vec<char>>, root_row: usize, root_col: usize) -> bool {
    const WORD: &str = "MAS";

    let mut fw_diagonal_word = String::new();
    let mut bw_diagonal_word = String::new();

    for x in 0..3 {
        let row = root_row + x;
        let fw_diagonal_col = root_col + x;
        let bw_diagonal_col = root_col + 2 - x;
        fw_diagonal_word.push(wordsearch[row][fw_diagonal_col]);
        bw_diagonal_word.push(wordsearch[row][bw_diagonal_col]);
    }

    (fw_diagonal_word == WORD || &fw_diagonal_word.chars().rev().collect::<String>() == WORD)
        && (bw_diagonal_word == WORD || &bw_diagonal_word.chars().rev().collect::<String>() == WORD)
}
