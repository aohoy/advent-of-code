use day_04::part2::solve;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = include_str!("../../input2.txt");
    let result = solve(file)?;
    println!("{}", result);
    Ok(())
}
