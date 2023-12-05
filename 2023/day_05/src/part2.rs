use std::error::Error;

use nom::{
    IResult,
    sequence::{preceded, tuple, Tuple, terminated, separated_pair},
    character::complete::{self, line_ending, space0, space1},
    multi::separated_list1,
    bytes::complete::{tag, take_until},
};

use indicatif::ParallelProgressIterator;
use rayon::iter::{ParallelIterator, IntoParallelIterator};

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

fn parse_seeds(input: &str) -> IResult<&str, Vec<(SeedType, SeedType)>> {
    preceded(
        tuple((tag("seeds:"), space0)),
        terminated(
            separated_list1(space1, separated_pair(complete::u64, space1, complete::u64)),
            line_ending
        )
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<(SeedType, SeedType)>, Vec<Map>)> {
    (parse_seeds, parse_mappings).parse(input)
}

pub fn solve(_input: &str) -> Result<String, Box<dyn Error>> {
    let (_, (seeds, mappings)) = parse_input(_input).unwrap();
    
    let s = seeds.iter().flat_map(|x| x.0..(x.0+x.1)).collect::<Vec<SeedType>>();
    let res = s
        .into_par_iter()
        .progress()
        .map(|x| {
            mappings
                .iter()
                .fold(x, |acc, el| el.convert(acc) )
        })
        .collect::<Vec<SeedType>>();

    Ok(res.iter().min().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("46", solve(input)?);
        Ok(())
    }
}
