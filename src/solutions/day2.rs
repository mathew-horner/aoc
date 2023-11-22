use std::str::FromStr;

use anyhow::anyhow;

#[derive(Clone, Eq, PartialEq)]
enum RpsMove {
    Rock,
    Paper,
    Scissors,
}

enum RpsOutcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for RpsMove {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            value => return Err(anyhow!("Invalid rock paper scissors move: {}", value)),
        })
    }
}

impl RpsMove {
    fn superior(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn inferior(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn score_versus(&self, theirs: &Self) -> u32 {
        let mut score = match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        };

        if self == theirs {
            score += 3;
        } else if theirs.superior() == *self {
            score += 6;
        }

        score
    }

    fn opponent_score_with_outcome(&self, outcome: &RpsOutcome) -> u32 {
        let move_ = match outcome {
            RpsOutcome::Win => self.superior(),
            RpsOutcome::Lose => self.inferior(),
            RpsOutcome::Draw => (*self).clone(),
        };

        move_.score_versus(self)
    }
}

impl FromStr for RpsOutcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            value => return Err(anyhow!("Invalid rock paper scissors outcome: {}", value)),
        })
    }
}

pub fn solve(input: crate::Input) {
    let input = input.read_all();
    let (total_score_winning, total_score_desired_outcome): (u32, u32) = input
        .lines()
        .map(|line| {
            let tokens: Vec<_> = line.split(" ").collect();
            let move_theirs: RpsMove = tokens[0].parse().unwrap();

            // Calculate the score when the second column is the winning move (Part 1).
            let move_ours: RpsMove = tokens[1].parse().unwrap();
            let score_winning = move_ours.score_versus(&move_theirs);

            // Calculate the score when the second column is the desired outcome (Part 2).
            let desired_outcome: RpsOutcome = tokens[1].parse().unwrap();
            let score_desired_outcome = move_theirs.opponent_score_with_outcome(&desired_outcome);

            (score_winning, score_desired_outcome)
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();

    println!("Answer #1: {}", total_score_winning);
    println!("Answer #2: {}", total_score_desired_outcome);
}
