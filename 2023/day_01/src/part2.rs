use std::error::Error;

pub fn solve(_input: &str) -> Result<String, Box<dyn Error>> {
    let res = _input
        .lines()
        .map(|line| {
            let mut chars = line
                .char_indices()
                .filter_map(|(i, c)| {
                    let ln = &line[i..];
                    let res = if ln.starts_with("one")  { '1' }
                        else if ln.starts_with("two")   { '2' }
                        else if ln.starts_with("three") { '3' }
                        else if ln.starts_with("four")  { '4' }
                        else if ln.starts_with("five")  { '5' }
                        else if ln.starts_with("six")   { '6' }
                        else if ln.starts_with("seven") { '7' }
                        else if ln.starts_with("eight") { '8' }
                        else if ln.starts_with("nine")  { '9' }
                        else { c };
                    res.to_digit(10)
                });
            let first = chars.next().unwrap();
            let last = match chars.last() {
                Some(x) => x,
                None => first
            };
            first * 10 + last
        })
        .reduce(|acc, e| { acc + e })
        .unwrap();
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!("281", solve(input)?);
        Ok(())
    }
}
