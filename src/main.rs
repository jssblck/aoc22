use stable_eyre::Report;

mod day1;
mod day2;

fn main() -> Result<(), Report> {
    println!("advent of code 2022");
    println!();

    println!("Day 1:");
    println!("- Part 1: {}", day1::part1(day1::INPUT)?);
    println!("- Part 2: {}", day1::part2(day1::INPUT)?);
    println!();

    println!("Day 2:");
    println!("- Part 1: {}", day2::part1(day2::INPUT)?);
    println!("- Part 2: {}", day2::part2(day2::INPUT)?);
    println!();

    Ok(())
}
