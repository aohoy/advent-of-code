use std::error::Error;

// red 1
fn bad_cube(_input: &str) -> Option<()> {
    let (num, color) = _input.trim().split_once(' ').unwrap();
    let num: i32 = num.parse().unwrap();
    let max = match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => panic!("incorrect value"),
    };
    match num > max {
        true => Some(()),
        false => None,
    }
}

// red 1, ...
fn bad_round(_input: &str) -> Option<()> {
    _input.split(',').find_map(bad_cube)
}

// Game 1: ...
fn game(_input: &str) -> Option<i32> {
    let (game_id, game_res) = _input.split_once(':').unwrap();
    let id: i32 = game_id.split_ascii_whitespace().last().unwrap().parse().unwrap();
    let res = game_res
        .trim()
        .split(';')
        .find_map(bad_round);
    match res {
        Some(_) => None,
        None => Some(id),
    }
}

pub fn solve(_input: &str) -> Result<String, Box<dyn Error>> {
    let res: i32 = _input
        .lines()
        .filter_map(game)
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
        assert_eq!("8", solve(input)?);
        Ok(())
    }
}
