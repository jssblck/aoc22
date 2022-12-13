use stable_eyre::Report;

mod day1;
mod day2;
mod day3;

macro_rules! run_day {
    ($day:ident) => {{
        println!("== {} ==", stringify!($day));
        println!(" part1: {}", $day::part1($day::INPUT)?);
        println!(" part2: {}", $day::part2($day::INPUT)?);
        println!();
    }};
}

fn main() -> Result<(), Report> {
    println!("advent of code 2022");
    println!();

    run_day!(day1);
    run_day!(day2);
    run_day!(day3);

    Ok(())
}
