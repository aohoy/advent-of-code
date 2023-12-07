use std::collections::HashSet;

use nom::{
    IResult,
    sequence::{delimited, separated_pair, preceded},
    character::complete::{self, char, line_ending, space0}, multi::{separated_list1, fold_many1},
    bytes::complete::tag,
};

advent_of_code::solution!(4);

fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        delimited(space0, complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, el| {
            acc.insert(el);
            acc
        }
    )(input)
}

fn card_name(input: &str) -> IResult<&str, u32> {
    preceded(tag("Card"), preceded(space0, complete::u32))(input)
}

fn card_data(input: &str) -> IResult<&str, (HashSet<u32>, HashSet<u32>)> {
    separated_pair(set, char('|'), set)(input)
}

fn card(input: &str) -> IResult<&str, (u32, (HashSet<u32>, HashSet<u32>))> {
    separated_pair(card_name, char(':'), card_data)(input)
}

fn cards(input: &str) -> IResult<&str, Vec<(u32, (HashSet<u32>, HashSet<u32>))>> {
    separated_list1(line_ending, card)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, parsed_cards) = cards(input).unwrap();
    parsed_cards
        .iter()
        .filter_map(|(_, (win, numbers))| {
            let num: u32 = win.intersection(numbers).collect::<HashSet<_>>().len() as u32;
            match num > 0 {
                true => Some(2u32.pow(num - 1)),
                false => None
            }
        })
        .reduce(|acc, el| acc + el)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, parsed_cards) = cards(input).unwrap();
    let seq = parsed_cards
        .iter()
        .map(|(_, (win, numbers))| {
            win.intersection(numbers).collect::<HashSet<_>>().len() as u32
        })
        .collect::<Vec<u32>>();

    let mut res = vec![1u32; seq.len()];
    for i in 0..seq.len() {
        let (start, end) = (i+1, std::cmp::min(i+(seq[i] as usize)+1, res.len()));
        for j in start..end {
            res[j] += res[i]
        }
    }

    
    res
        .iter()
        .map(|&el| el)
        .reduce(|acc, el| acc + el)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
