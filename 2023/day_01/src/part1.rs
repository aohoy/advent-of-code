use std::error::Error;

pub fn solve(_input: &str) -> Result<String, Box<dyn Error>> {
    let res = _input
        .lines()
        .map(|line| {
            let mut chars = line
                .chars()
                .filter_map(|x| x.to_digit(10));
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
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!("142", solve(input)?);
        Ok(())
    }
}
