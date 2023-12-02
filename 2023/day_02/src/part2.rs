use std::error::Error;

#[derive(Debug)]
struct RGB (i32, i32, i32);

// red 1
fn cube(_input: &str) -> RGB {
    let (num, color) = _input.trim().split_once(' ').unwrap();
    let num: i32 = num.parse().unwrap();
    match color {
        "red" => RGB(num, 0, 0),
        "green" => RGB(0, num, 0),
        "blue" => RGB(0, 0, num),
        _ => panic!("incorrect value"),
    }
}

// red 1,...
fn round(_input: &str) -> RGB {
    _input
        .split(',')
        .map(cube)
        .reduce(|acc, e| {
            RGB(acc.0 + e.0, acc.1 + e.1, acc.2 + e.2)
        })
        .unwrap()
}

// Game 1: ...
fn game(_input: &str) -> i32 {
    let (_, game_res) = _input.split_once(':').unwrap();
    let res = game_res
        .trim()
        .split(';')
        .map(round)
        .reduce(|mut acc, e| {
            if acc.0 < e.0 {acc.0 = e.0};
            if acc.1 < e.1 {acc.1 = e.1};
            if acc.2 < e.2 {acc.2 = e.2};
            acc
        })
        .unwrap();
    res.0 * res.1 * res.2
}

pub fn solve(_input: &str) -> Result<String, Box<dyn Error>> {
    let res: i32 = _input
        .lines()
        .map(game)
        .sum();
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", solve(input)?);
        Ok(())
    }
}
