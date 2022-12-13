use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use once_cell::sync::Lazy;
use stable_eyre::{
    eyre::{bail, ensure, Context},
    Report,
};

/// The puzzle input.
pub const INPUT: &str = include_str!("input/day3");

/// The list of items for each rucksack is given as characters all on a single line.
/// A given rucksack always has the same number of items in each of its two compartments,
/// so the first half of the characters represent items in the first compartment,
/// while the second half of the characters represent items in the second compartment.
///
/// Suppose you have the following list of contents from six rucksacks:
///
/// ```not_rust
/// vJrwpWtwJgWrhcsFMMfFFhFp
/// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
/// PmmdzqPrVvPwwTWBwg
/// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
/// ttgJtRGJQctTZtZT
/// CrZsJsPPZsGzwwsLwLmpwMDw
/// ```
///
/// - The first rucksack contains the items `vJrwpWtwJgWrhcsFMMfFFhFp`,
///   which means its first compartment contains the items `vJrwpWtwJgWr`,
///   while the second compartment contains the items `hcsFMMfFFhFp`.
///   The only item type that appears in both compartments is lowercase p.
/// - The second rucksack's compartments contain `jqHRNqRjqzjGDLGL` and `rsFMfFZSrLrFZsSL`.
///   The only item type that appears in both compartments is uppercase L.
/// - The third rucksack's compartments contain `PmmdzqPrV` and `vPwwTWBwg`; the only common item type is uppercase P.
/// - The fourth rucksack's compartments only share item type v.
/// - The fifth rucksack's compartments only share item type t.
/// - The sixth rucksack's compartments only share item type s.
///
/// To help prioritize item rearrangement, every item type can be converted to a priority:
///
/// - Lowercase item types a through z have priorities 1 through 26.
/// - Uppercase item types A through Z have priorities 27 through 52.
///
/// In the above example, the priority of the item type that appears in both compartments of each rucksack is
/// 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.
///
/// Find the item type that appears in both compartments of each rucksack.
/// What is the sum of the priorities of those item types?
pub fn part1(input: &str) -> Result<usize, Report> {
    input
        .lines()
        .map(Rucksack::parse)
        .collect::<Result<Vec<_>, _>>()
        .map(score_rucksacks)
}

/// Every set of three lines in your list corresponds to a single group,
/// but each group can have a different badge item type.
/// So, in the above example, the first group's rucksacks are the first three lines:
///
/// ```not_rust
/// vJrwpWtwJgWrhcsFMMfFFhFp
/// jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
/// PmmdzqPrVvPwwTWBwg
/// ```
///
/// And the second group's rucksacks are the next three lines:
///
/// ```not_rust
/// wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
/// ttgJtRGJQctTZtZT
/// CrZsJsPPZsGzwwsLwLmpwMDw
/// ```
///
/// In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges.
/// In the second group, their badge item type must be Z.
///
/// Priorities for these items must still be found to organize the sticker attachment efforts:
/// here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.
///
/// Find the item type that corresponds to the badges of each three-Elf group.
pub fn part2(input: &str) -> Result<usize, Report> {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(Group::parse)
        .collect::<Result<Vec<_>, _>>()
        .and_then(score_groups)
}

fn score_rucksacks(sacks: Vec<Rucksack>) -> usize {
    sacks.into_iter().map(|rs| rs.priority).sum()
}

fn score_groups(groups: Vec<Group>) -> Result<usize, Report> {
    groups
        .iter()
        .map(Group::score)
        .collect::<Result<Vec<_>, _>>()
        .map(|scores| scores.into_iter().sum())
}

/// Every item type can be converted to a priority:
///
/// - Lowercase item types a through z have priorities 1 through 26.
/// - Uppercase item types A through Z have priorities 27 through 52.
fn calculate_priority(item_type: &char) -> Result<usize, Report> {
    let Some(priority) = PRIORITY.get(item_type) else { bail!("no priority for item type") };
    Ok(priority.to_owned())
}

/// A group is made up of multiple rucksacks.
struct Group {
    rucksacks: Vec<Rucksack>,
}

impl Group {
    fn new(rucksacks: Vec<Rucksack>) -> Self {
        Self { rucksacks }
    }

    fn parse<'a>(lines: impl IntoIterator<Item = &'a str>) -> Result<Self, Report> {
        lines
            .into_iter()
            .map(Rucksack::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(Self::new)
    }

    fn score(&self) -> Result<usize, Report> {
        let mut intersections = self
            .rucksacks
            .iter()
            .fold(HashSet::new(), |mut acc, sack| {
                let contents = sack.contents.chars().collect::<HashSet<_>>();
                let matching = |c: &char| contents.contains(c);
                if acc.is_empty() {
                    contents
                } else {
                    acc.retain(matching);
                    acc
                }
            })
            .into_iter();

        let Some(group_type) = intersections.next() else { bail!("no intersections found"); };
        ensure!(
            intersections.next().is_none(),
            "more than one intersection found"
        );

        calculate_priority(&group_type)
    }
}

/// A rucksack is made up of two compartments.
///
/// The item type that is shared between the two compartments is the item type of the rucksack.
/// The priority is then based upon that item type.
#[derive(Debug)]
struct Rucksack {
    contents: String,
    priority: usize,
}

impl Rucksack {
    /// A rucksack is made up of two compartments.
    ///
    /// The input string is split evenly in half, and the two halves are both compartments.
    fn parse(input: &str) -> Result<Self, Report> {
        let delimiter = input.len() / 2;
        let (first, second) = input.split_at(delimiter);
        let (first, second) = (Compartment::parse(first), Compartment::parse(second));

        let item_type = Self::calculate_item_type(&first, &second)
            .wrap_err_with(|| format!("calculate_item_type({first:?}, {second:?})"))?;
        let priority = calculate_priority(&item_type)
            .wrap_err_with(|| format!("calculate_priority({item_type:?})"))?;

        Ok(Self {
            contents: input.to_owned(),
            priority,
        })
    }

    /// The item type that is shared between the two compartments is the item type of the rucksack.
    fn calculate_item_type(first: &Compartment, second: &Compartment) -> Result<char, Report> {
        let mut intersections = first.contents.intersection(&second.contents);
        let Some(intersection) = intersections.next() else { bail!("no intersection found"); };
        ensure!(
            intersections.next().is_none(),
            "more than one intersection found"
        );
        Ok(intersection.to_owned())
    }
}

#[derive(Debug)]
struct Compartment {
    contents: HashSet<char>,
}

impl Compartment {
    fn parse(input: &str) -> Self {
        Self {
            contents: input.chars().collect(),
        }
    }
}

static PRIORITY: Lazy<HashMap<char, usize>> = Lazy::new(|| {
    let lower = 'a'..='z';
    let upper = 'A'..='Z';
    let flip = |(i, c): (usize, char)| (c, i);
    let idx_to_count = |(c, i): (char, usize)| (c, i + 1);
    HashMap::from_iter(lower.chain(upper).enumerate().map(flip).map(idx_to_count))
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_correct() {
        assert_eq!(PRIORITY[&'a'], 1);
        assert_eq!(PRIORITY[&'b'], 2);
        assert_eq!(PRIORITY[&'z'], 26);
        assert_eq!(PRIORITY[&'A'], 27);
        assert_eq!(PRIORITY[&'Z'], 52);
    }

    #[test]
    fn test_part1() -> Result<(), Report> {
        assert_eq!(part1(INPUT)?, 7875);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Report> {
        assert_eq!(part2(INPUT)?, 2479);
        Ok(())
    }
}
