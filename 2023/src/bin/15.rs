use std::{collections::HashMap, hash};

use itertools::enumerate;
use nom::{
    character::complete::{self, alpha1, digit1, line_ending, one_of},
    combinator::{complete, opt},
    multi::{many1, separated_list1},
    sequence::{tuple, Tuple},
    IResult,
};

advent_of_code::solution!(15);

fn parse_tag(input: &str) -> IResult<&str, (&str, char, Option<u8>)> {
    tuple((alpha1, one_of("=-"), opt(complete::u8)))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, char, Option<u8>)>> {
    separated_list1(complete::char(','), parse_tag)(input)
}

fn hash(input: &str) -> u8 {
    input
        .chars()
        .fold(0u32, |acc, c| ((acc + c as u32) * 17) % 256) as u8
}

pub fn part_one(input: &str) -> Option<u32> {
    let sum: u32 = input.trim_end().split(',').map(|x| hash(x) as u32).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, tags) = parse(input).unwrap();
    let mut boxes: HashMap<u8, Vec<(&str, u8)>> = HashMap::new();
    tags.iter().for_each(|&(tag, op, num)| match op {
        '=' => {
            let b = boxes.entry(hash(tag)).or_default();
            b.iter_mut()
                .find_map(|el| (el.0 == tag).then(|| el.1 = num.unwrap()))
                .or_else(|| {
                    b.push((tag, num.unwrap()));
                    Some(())
                });
        }
        '-' => {
            boxes
                .entry(hash(tag))
                .and_modify(|b| b.retain(|el| el.0 != tag));
        }
        _ => {}
    });
    let res: u32 = boxes
        .iter()
        .map(|(k, v)| {
            v.iter().enumerate().fold(0u32, |acc, (i, el)| {
                (*k as u32 + 1) * (i as u32 + 1) * (el.1 as u32) + acc
            })
        })
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
