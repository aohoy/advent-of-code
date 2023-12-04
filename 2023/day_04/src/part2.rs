use std::collections::HashSet;
use std::error::Error;

use nom::{
    IResult,
    sequence::{delimited, separated_pair, preceded},
    character::complete::{self, char, line_ending, space0}, multi::{separated_list1, fold_many1},
    bytes::complete::tag,
};

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

pub fn solve(_input: &str) -> Result<String, Box<dyn Error>> {
    let (_, parsed_cards) = cards(_input).unwrap();
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

    
    Ok(res.iter().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", solve(input)?);
        Ok(())
    }
}
