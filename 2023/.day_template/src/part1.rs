use std::error::Error;

pub fn solve(_input: &str) -> Result<String, Box<dyn Error>> {
    todo!("Implement solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        todo!("Implement test");
        let input = "";
        assert_eq!("", solve(input)?);
        Ok(())
    }
}
