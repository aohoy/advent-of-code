advent_of_code::solution!(5);

use nom::{
    IResult,
    sequence::{preceded, tuple, Tuple, terminated, separated_pair},
    character::complete::{self, line_ending, space0, space1},
    multi::separated_list1,
    bytes::complete::{tag, take_until},
};

type SeedType = u64;

#[derive(Debug)]
struct Map {
    maps: Vec<(SeedType, SeedType, SeedType)>,
} 

impl Map {
    fn convert(&self, src: SeedType) -> SeedType {
        self.maps
            .iter()
            .find_map(|&el| {
                if el.1 <= src && src < el.1 + el.2 {
                    Some(el.0 + (src - el.1))
                } else {
                    None
                } 
            })
            .unwrap_or(src)
    }
}

fn parse_range(input: &str) -> IResult<&str, (SeedType, SeedType, SeedType)> {
    tuple((
        preceded(space0, complete::u64),
        preceded(space1, complete::u64),
        preceded(space1, complete::u64),
    ))(input)
}

fn parse_mapping(input: &str) -> IResult<&str, Map> {
    let (input, res) = preceded(
        tuple((take_until("map:"), tag("map:"), line_ending)),
        separated_list1(line_ending, parse_range)
    )(input)?;

    Ok((input, Map{ maps: res }))
}

fn parse_mappings(input: &str) -> IResult<&str, Vec<Map>> {
    preceded(
        line_ending,
        separated_list1(line_ending, parse_mapping)
    )(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<SeedType>> {
    preceded(
        tuple((tag("seeds:"), space0)),
        terminated(
            separated_list1(space1, complete::u64),
            line_ending
        )
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<SeedType>, Vec<Map>)> {
    (parse_seeds, parse_mappings).parse(input)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (seeds, mappings)) = parse_input(input).unwrap();
    
    let mut res = Vec::<SeedType>::with_capacity(seeds.len());
    for seed in seeds {
        let mut seed = seed;
        for map in &mappings {
            seed = map.convert(seed);
        }
        res.push(seed);
    };

    res.iter().map(|&el| el).min()
}

fn parse_seeds_two(input: &str) -> IResult<&str, Vec<(SeedType, SeedType)>> {
    preceded(
        tuple((tag("seeds:"), space0)),
        terminated(
            separated_list1(space1, separated_pair(complete::u64, space1, complete::u64)),
            line_ending
        )
    )(input)
}

fn parse_input_two(input: &str) -> IResult<&str, (Vec<(SeedType, SeedType)>, Vec<Map>)> {
    (parse_seeds_two, parse_mappings).parse(input)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (seeds, mappings)) = parse_input_two(input).unwrap();
    
    seeds
        .iter()
        .flat_map(|x| x.0..(x.0+x.1))
        .map(|x| {
            mappings
                .iter()
                .fold(x, |acc, el| el.convert(acc) )
        })
        .reduce(|acc, el| if acc < el { acc } else { el })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
