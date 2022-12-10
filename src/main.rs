use stable_eyre::Report;

mod day1;

fn main() -> Result<(), Report> {
    println!("advent of code 2022");
    println!();

    println!("Day 1:");
    println!("- Part 1: {}", day1::part1(day1::INPUT)?);
    println!("- Part 2: {}", day1::part2(day1::INPUT)?);
    println!();

    Ok(())
}
