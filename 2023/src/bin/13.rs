use std::iter;

use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

advent_of_code::solution!(13);

fn diff_num(first: &[char], second: &[char]) -> u32 {
    iter::zip(first, second)
        .filter_map(|(f, s)| (f != s).then_some(()))
        .count() as u32
}

fn is_mirror(above: &[Vec<char>], below: &[Vec<char>], is_smudge: bool) -> bool {
    let mut diff = 0;
    for (a, b) in iter::zip(above.iter().rev(), below) {
        if is_smudge {
            diff += diff_num(a, b);
            if diff > 1 {
                return false;
            }
        } else if a != b {
            return false;
        }
    }
    !is_smudge || diff == 1
}

fn detect_mirror(field: &[Vec<char>], is_smudge: bool) -> Option<u32> {
    let mut prev = field.first().unwrap();

    field.iter().enumerate().skip(1).find_map(|(i, x)| {
        ((x == prev || is_smudge) && is_mirror(&field[0..i], &field[i..], is_smudge))
            .then_some(i as u32)
            .or_else(|| {
                prev = x;
                None
            })
    })
}

fn field(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(one_of("#.")))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Vec<char>>>> {
    separated_list1(tuple((line_ending, line_ending)), field)(input)
}

fn solve(input: &str, is_smudge: bool) -> Option<u32> {
    let (_, fields) = parse(input).unwrap();
    let res: u32 = fields
        .iter()
        .map(|x| {
            if let Some(f) = detect_mirror(x, is_smudge) {
                return f * 100;
            }

            let rotated = (0..x[0].len())
                .map(|i| x.iter().map(|row| row[i]).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            if let Some(f) = detect_mirror(&rotated, is_smudge) {
                return f;
            }
            panic!("no mirrors found")
        })
        .sum();
    Some(res)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
