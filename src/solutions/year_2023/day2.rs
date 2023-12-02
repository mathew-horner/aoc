use std::collections::HashMap;

const MAX_REDS: u32 = 12;
const MAX_GREENS: u32 = 13;
const MAX_BLUES: u32 = 14;

struct Game {
    id: u32,
    counts: Vec<CubeCounts>,
}

impl Game {
    /// Parses a game's data from a line from the challenge input.
    fn parse(line: &str) -> Self {
        let tokens: Vec<_> = line.split(':').map(str::trim).collect();
        let id = parse_game_id(tokens[0]);
        let counts = parse_game_cube_counts(tokens[1]);
        Self { id, counts }
    }

    /// Returns whether or not this game has a pull that violates one of the `MAX_` constraints.
    fn has_impossible_pull(&self) -> bool {
        self.counts.iter().any(|pull| {
            pull.num_reds > MAX_REDS || pull.num_greens > MAX_GREENS || pull.num_blues > MAX_BLUES
        })
    }

    /// Finds the fewest number of each block color necessary for the game to be possible.
    fn fewest_possible(&self) -> CubeCounts {
        let mut counts_iter = self.counts.iter();
        let first = counts_iter.next().unwrap();
        let mut fewest = CubeCounts {
            num_reds: first.num_reds,
            num_blues: first.num_blues,
            num_greens: first.num_greens,
        };

        for count in counts_iter {
            fewest.num_reds = fewest.num_reds.max(count.num_reds);
            fewest.num_blues = fewest.num_blues.max(count.num_blues);
            fewest.num_greens = fewest.num_greens.max(count.num_greens);
        }

        fewest
    }

    /// Finds the fewest number of each block color necessary and returns the product of those numbers.
    fn fewest_possible_product(&self) -> u32 {
        let fewest = self.fewest_possible();
        fewest.num_reds * fewest.num_blues * fewest.num_greens
    }
}

/// Parses the game's id out of the appropriate token.
///
/// Example: "Game 72"
fn parse_game_id(token: &str) -> u32 {
    token
        .split_whitespace()
        .last()
        .expect("incorrect number of subtokens in game label token found")
        .parse()
        .expect("failed to parse game id subtoken")
}

/// Parses the game's "cube counts" out of the appropriate token.
///
/// Example: "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
fn parse_game_cube_counts(token: &str) -> Vec<CubeCounts> {
    token
        .split(';')
        .map(str::trim)
        .map(CubeCounts::parse)
        .collect()
}

struct CubeCounts {
    num_reds: u32,
    num_greens: u32,
    num_blues: u32,
}

impl CubeCounts {
    /// Parses a single cube count for a game out of the appropriate token.
    ///
    /// Example: "3 blue, 4 red"
    fn parse(token: &str) -> Self {
        let mut counts: HashMap<&str, u32> = HashMap::new();
        let count_tokens: Vec<_> = token.split(',').map(str::trim).collect();

        for count_token in count_tokens {
            let mut subtokens = count_token.split_whitespace();
            let count = subtokens.next().unwrap().parse().unwrap();
            let color = subtokens.last().unwrap();
            counts.insert(color, count);
        }

        Self {
            num_reds: counts.remove("red").unwrap_or(0),
            num_greens: counts.remove("green").unwrap_or(0),
            num_blues: counts.remove("blue").unwrap_or(0),
        }
    }
}

pub fn solve(input: crate::Input) {
    let games: Vec<_> = input.read_lines().map(|s| Game::parse(&s)).collect();
    let answer1: u32 = games
        .iter()
        .filter(|game| !game.has_impossible_pull())
        .map(|game| game.id)
        .sum();
    let answer2: u32 = games.iter().map(Game::fewest_possible_product).sum();

    println!("Answer #1: {answer1}");
    println!("Answer #2: {answer2}");
}
