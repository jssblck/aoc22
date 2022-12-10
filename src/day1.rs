use stable_eyre::{eyre::Context, Report};

/// The puzzle input.
pub const INPUT: &str = include_str!("input/day1");

/// Given the input, how many total calories is carried by the elf carrying the most calories?
///
/// Example input:
/// ```not_rust
/// 1000
/// 2000
/// 3000
///
/// 4000
///
/// 5000
/// 6000
///
/// 7000
/// 8000
/// 9000
///
/// 10000
/// ```
///
/// This list represents the Calories of the food carried by five Elves:
/// - The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
/// - The second Elf is carrying one food item with 4000 Calories.
/// - The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
/// - The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
/// - The fifth Elf is carrying one food item with 10000 Calories.
///
/// In case the Elves get hungry and need extra snacks, they need to know which Elf to ask:
/// they'd like to know how many Calories are being carried by the Elf carrying the most Calories.
///
/// In the example above, this is 24000 (carried by the fourth Elf).
pub fn part1(input: &str) -> Result<usize, Report> {
    group_stashes(input).map(|stashes| stashes.into_iter().max().unwrap_or_default())
}

/// By the time you calculate the answer to the Elves' question,
/// they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.
///
/// To avoid this unacceptable situation, the Elves would instead like to know the total Calories
/// carried by the top three Elves carrying the most Calories.
/// That way, even if one of those Elves runs out of snacks, they still have two backups.
///
/// In the example above, the top three Elves are the fourth Elf (with 24000 Calories),
/// then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories).
/// The sum of the Calories carried by these three elves is 45000.
///
/// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
pub fn part2(input: &str) -> Result<usize, Report> {
    group_stashes(input).map(|stashes| stashes.into_iter().multi_max(3).into_iter().sum())
}

fn parse_calories(line: &str) -> Result<usize, Report> {
    line.parse()
        .wrap_err_with(|| format!("parse input '{line}'"))
}

/// Given input in the form:
/// ```not_rust
/// <NUMBER>
/// <NUMBER>
/// <SPACE>
/// <NUMBER>
/// <NUMBER>
/// <NUMBER>
/// ```
///
/// This function sums each consecutive number, creating a new element in the vec when a space is encountered.
fn group_stashes(input: &str) -> Result<Vec<usize>, Report> {
    input
        .lines()
        .try_fold(Vec::new(), |mut elves, food| -> Result<_, Report> {
            if food.is_empty() {
                elves.push(0);
            } else {
                let calories = parse_calories(food)?;
                match elves.last_mut() {
                    Some(elf) => *elf += calories,
                    None => elves.push(calories),
                }
            }
            Ok(elves)
        })
}

trait MultiMaxer<T>
where
    T: Ord,
{
    /// Construct a vector which collects the top N values from the iterator.
    fn multi_max(self, count: usize) -> Vec<T>;
}

impl<T, I> MultiMaxer<T> for I
where
    T: Ord,
    I: Iterator<Item = T>,
{
    /// Construct a vector which collects the top N values from the iterator.
    fn multi_max(self, count: usize) -> Vec<T> {
        let mut maxes = Vec::with_capacity(count);

        for current_value in self {
            if maxes.len() < count {
                maxes.push(current_value);
            } else {
                for prev_max in maxes.iter_mut() {
                    if matches!(Ord::cmp(prev_max, &current_value), std::cmp::Ordering::Less) {
                        *prev_max = current_value;
                        break;
                    }
                }
            }

            // To keep the "searching for a value to replace" logic simpler,
            // just ensure that the lowest max value is the earliest item in the vec
            // each time we modify it.
            maxes.sort()
        }

        maxes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), Report> {
        assert_eq!(part1(INPUT)?, 69528);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Report> {
        assert_eq!(part2(INPUT)?, 206152);
        Ok(())
    }

    #[test]
    fn test_multi_max() {
        let inputs = vec![100, 200, 300, 400, 100, 500];
        let expected = vec![300, 400, 500];
        let maxes = inputs.into_iter().multi_max(3);
        assert_eq!(maxes, expected);

        let inputs = vec![100, 200, 300, 400, 100, 500];
        let expected = vec![100, 200, 300, 400, 500];
        let maxes = inputs.into_iter().multi_max(5);
        assert_eq!(maxes, expected);

        let inputs = vec![100, 200];
        let expected = vec![100, 200];
        let maxes = inputs.into_iter().multi_max(3);
        assert_eq!(maxes, expected);

        let inputs: Vec<usize> = vec![];
        let expected = vec![];
        let maxes = inputs.into_iter().multi_max(3);
        assert_eq!(maxes, expected);
    }

    #[test]
    fn test_group_stashes() -> Result<(), Report> {
        let input = r#"100
100
100

400

100
100"#;
        let expected = vec![300, 400, 200];
        assert_eq!(group_stashes(input)?, expected);
        Ok(())
    }
}
