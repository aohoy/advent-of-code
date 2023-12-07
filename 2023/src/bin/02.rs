advent_of_code::solution!(2);

// red 1
fn bad_cube(input: &str) -> Option<()> {
    let (num, color) = input.trim().split_once(' ').unwrap();
    let num: u32 = num.parse().unwrap();
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
fn bad_round(input: &str) -> Option<()> {
    input.split(',').find_map(bad_cube)
}

// Game 1: ...
fn game(input: &str) -> Option<u32> {
    let (game_id, game_res) = input.split_once(':').unwrap();
    let id: u32 = game_id.split_ascii_whitespace().last().unwrap().parse().unwrap();
    let res = game_res
        .trim()
        .split(';')
        .find_map(bad_round);
    match res {
        Some(_) => None,
        None => Some(id),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines()
        .filter_map(game)
        .reduce(|acc, el| acc + el)
}

#[derive(Debug)]
struct RGB (u32, u32, u32);

// red 1
fn cube(input: &str) -> RGB {
    let (num, color) = input.trim().split_once(' ').unwrap();
    let num: u32 = num.parse().unwrap();
    match color {
        "red" => RGB(num, 0, 0),
        "green" => RGB(0, num, 0),
        "blue" => RGB(0, 0, num),
        _ => panic!("incorrect value"),
    }
}

// red 1,...
fn round(input: &str) -> RGB {
    input
        .split(',')
        .map(cube)
        .reduce(|acc, e| {
            RGB(acc.0 + e.0, acc.1 + e.1, acc.2 + e.2)
        })
        .unwrap()
}

// Game 1: ...
fn game_two(input: &str) -> u32 {
    let (_, game_res) = input.split_once(':').unwrap();
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

pub fn part_two(input: &str) -> Option<u32> {
    input.lines()
        .map(game_two)
        .reduce(|acc, el| acc + el)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
