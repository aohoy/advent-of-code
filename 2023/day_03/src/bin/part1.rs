use day_03::part1::solve;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("../../input1.txt");
    let result = solve(file)?;
    println!("{}", result);
    Ok(())
}
