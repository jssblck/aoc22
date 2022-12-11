use std::cmp::Ordering;

use duplicate::duplicate_item;
use stable_eyre::{
    eyre::{bail, ensure},
    Report,
};
use strum::{EnumIter, IntoEnumIterator};

/// The puzzle input.
pub const INPUT: &str = include_str!("input/day2");

/// one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The
/// first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second
/// column--" Suddenly, the Elf is called away to help with someone's tent.
///
/// The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for
/// Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
///
/// The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores
/// for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3
/// for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you
/// won).
///
/// Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get
/// if you were to follow the strategy guide.
///
/// For example, suppose you were given the following strategy guide:
/// ```not_rust
/// A Y
/// B X
/// C Z
/// ```
///
/// This strategy guide predicts and recommends the following:
///
/// - In the first round, your opponent will choose Rock (A), and you should choose Paper (Y).
///   This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
/// - In the second round, your opponent will choose Paper (B), and you should choose Rock (X).
///   This ends in a loss for you with a score of 1 (1 + 0).
/// - The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
///
/// In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
pub fn part1(input: &str) -> Result<usize, Report> {
    parse_rounds(input, parse_round).map(score_rounds)
}

/// "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the
/// round in a draw, and Z means you need to win. Good luck!"
///
/// The total score is still calculated in the same way, but now you need to figure out what shape to choose so the
/// round ends as indicated. The example above now goes like this:
///
/// - In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock.
///   This gives you a score of 1 + 3 = 4.
/// - In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
/// - In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
///
/// Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.
pub fn part2(input: &str) -> Result<usize, Report> {
    parse_rounds(input, parse_constraint)
        .and_then(reconstruct_rounds)
        .map(score_rounds)
}
fn parse_rounds<T, F>(input: &str, parser: F) -> Result<Vec<(OpponentMove, T)>, Report>
where
    F: Fn(&str) -> Result<(OpponentMove, T), Report>,
{
    input.lines().map(parser).collect()
}

fn score_rounds(rounds: Vec<(OpponentMove, PlayerMove)>) -> usize {
    rounds
        .into_iter()
        .map(|(opponent, player)| round_score(opponent, player))
        .sum()
}

fn reconstruct_rounds(
    rounds: Vec<(OpponentMove, PlayerConstraint)>,
) -> Result<Vec<(OpponentMove, PlayerMove)>, Report> {
    rounds
        .into_iter()
        .map(|(opponent, constraint)| {
            let player_move = desired_move(opponent, constraint)?;
            Ok((opponent, player_move))
        })
        .collect()
}

/// Calculate the move the player should make given the desired end state for the round.
fn desired_move(
    opponent: OpponentMove,
    constraint: PlayerConstraint,
) -> Result<PlayerMove, Report> {
    // To keep things simple, just brute force it.
    for possible_move in PlayerMove::iter() {
        if evaluate_round(opponent, possible_move) == constraint {
            return Ok(possible_move);
        }
    }

    bail!("no possible move found that satisfies player move constraint {constraint:?} for opponent move {opponent:?}");
}

/// Evaluate who won a round.
fn evaluate_round(opponent: OpponentMove, player: PlayerMove) -> Round {
    let Some(cmp) = PartialOrd::partial_cmp(&opponent, &player) else { unreachable!() };
    match cmp {
        Ordering::Less => Round::PlayerWin,
        Ordering::Equal => Round::Draw,
        Ordering::Greater => Round::PlayerLose,
    }
}

fn parse_round(input: &str) -> Result<(OpponentMove, PlayerMove), Report> {
    // Do it the hacky way, I don't feel like figuring out nom right now
    ensure!(
        input.len() == 3,
        "inputs must consist of 3 characters, got '{input}'"
    );

    // Just assume 3 chars, separated by space.
    let mut chars = input.chars();
    let opponent = OpponentMove::parse(chars.next())?;
    chars.next();
    let player = PlayerMove::parse(chars.next())?;

    Ok((opponent, player))
}

fn parse_constraint(input: &str) -> Result<(OpponentMove, PlayerConstraint), Report> {
    // Do it the hacky way, I don't feel like figuring out nom right now
    ensure!(
        input.len() == 3,
        "inputs must consist of 3 characters, got '{input}'"
    );

    // Just assume 3 chars, separated by space.
    let mut chars = input.chars();
    let opponent = OpponentMove::parse(chars.next())?;
    chars.next();
    let player = PlayerConstraint::parse(chars.next())?;

    Ok((opponent, player))
}

fn round_score(opponent: OpponentMove, player: PlayerMove) -> usize {
    evaluate_round(opponent, player).score() + player.score()
}

trait Score {
    fn score(&self) -> usize;
}

/// The result of a single round.
enum Round {
    PlayerLose,
    Draw,
    PlayerWin,
}

impl Score for Round {
    fn score(&self) -> usize {
        match self {
            Round::PlayerLose => 0,
            Round::Draw => 3,
            Round::PlayerWin => 6,
        }
    }
}

/// The moves an opponent may take.
#[derive(Debug, Copy, Clone)]
enum OpponentMove {
    Rock,
    Paper,
    Scissors,
}

impl OpponentMove {
    fn parse(input: Option<char>) -> Result<Self, Report> {
        match input {
            Some('A') => Ok(OpponentMove::Rock),
            Some('B') => Ok(OpponentMove::Paper),
            Some('C') => Ok(OpponentMove::Scissors),
            None => bail!("unexpected end of input"),
            _ => bail!("unexpected input"),
        }
    }
}

/// The moves the player may take.
#[derive(Copy, Clone, EnumIter)]
enum PlayerMove {
    Rock,
    Paper,
    Scissors,
}

impl Score for PlayerMove {
    fn score(&self) -> usize {
        match self {
            PlayerMove::Rock => 1,
            PlayerMove::Paper => 2,
            PlayerMove::Scissors => 3,
        }
    }
}

impl PlayerMove {
    fn parse(input: Option<char>) -> Result<Self, Report> {
        match input {
            Some('X') => Ok(PlayerMove::Rock),
            Some('Y') => Ok(PlayerMove::Paper),
            Some('Z') => Ok(PlayerMove::Scissors),
            None => bail!("unexpected end of input"),
            _ => bail!("unexpected input"),
        }
    }
}

/// `OpponentMove` and `PlayerMove` are semantically equivalent, let's make them comparable.
#[duplicate_item(
    local target;
    [ OpponentMove ] [ PlayerMove ];
    [ PlayerMove ] [ OpponentMove ];
)]
impl PartialEq<target> for local {
    fn eq(&self, other: &target) -> bool {
        match self {
            local::Rock => matches!(other, target::Rock),
            local::Paper => matches!(other, target::Paper),
            local::Scissors => matches!(other, target::Scissors),
        }
    }
}

/// `OpponentMove` and `PlayerMove` are semantically equivalent, let's make them orderable.
#[duplicate_item(
    local target;
    [ OpponentMove ] [ PlayerMove ];
    [ PlayerMove ] [ OpponentMove ];
)]
impl PartialOrd<target> for local {
    fn partial_cmp(&self, other: &target) -> Option<Ordering> {
        match self {
            local::Rock => match other {
                target::Rock => Some(Ordering::Equal),
                target::Paper => Some(Ordering::Less),
                target::Scissors => Some(Ordering::Greater),
            },
            local::Paper => match other {
                target::Rock => Some(Ordering::Greater),
                target::Paper => Some(Ordering::Equal),
                target::Scissors => Some(Ordering::Less),
            },
            local::Scissors => match other {
                target::Rock => Some(Ordering::Less),
                target::Paper => Some(Ordering::Greater),
                target::Scissors => Some(Ordering::Equal),
            },
        }
    }
}

/// The constraint on the move the player should take.
#[derive(Debug, Copy, Clone)]
enum PlayerConstraint {
    Draw,
    PlayerWin,
    PlayerLose,
}

impl PlayerConstraint {
    fn parse(input: Option<char>) -> Result<Self, Report> {
        match input {
            Some('X') => Ok(PlayerConstraint::PlayerLose),
            Some('Y') => Ok(PlayerConstraint::Draw),
            Some('Z') => Ok(PlayerConstraint::PlayerWin),
            None => bail!("unexpected end of input"),
            _ => bail!("unexpected input"),
        }
    }
}

/// `PlayerConstraint` and `Round` are semantically equivalent, let's make them comparable.
#[duplicate_item(
    local target;
    [ PlayerConstraint ] [ Round ];
    [ Round ] [ PlayerConstraint ];
)]
impl PartialEq<target> for local {
    fn eq(&self, other: &target) -> bool {
        match self {
            local::Draw => matches!(other, target::Draw),
            local::PlayerWin => matches!(other, target::PlayerWin),
            local::PlayerLose => matches!(other, target::PlayerLose),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), Report> {
        assert_eq!(part1(INPUT)?, 11386);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Report> {
        assert_eq!(part2(INPUT)?, 13600);
        Ok(())
    }
}
