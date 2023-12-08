use std::collections::{HashMap, HashSet};

use nom::{
    character::complete::{self, alpha1, alphanumeric1, line_ending, one_of, space0},
    combinator::{eof, opt},
    multi::{fold_many1, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(8);

fn next<'a>(node: (&'a str, &'a str), dir: char) -> &'a str {
    match dir {
        'L' => node.0,
        'R' => node.1,
        _ => unimplemented!("impossible to get here"),
    }
}

fn parse_entry(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alphanumeric1,
        delimited(space0, complete::char('='), space0),
        delimited(
            complete::char('('),
            separated_pair(
                alphanumeric1,
                delimited(space0, complete::char(','), space0),
                alphanumeric1,
            ),
            complete::char(')'),
        ),
    )(input)
}

fn parse_graph(input: &str) -> IResult<&str, HashMap<&str, (&str, (&str, &str))>> {
    fold_many1(
        terminated(parse_entry, opt(line_ending)),
        HashMap::new,
        |mut acc: HashMap<_, _>, el| {
            acc.insert(el.0, el);
            acc
        },
    )(input)
}

fn parse_directions(input: &str) -> IResult<&str, Vec<char>> {
    terminated(many1(one_of("RL")), line_ending)(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<char>, HashMap<&str, (&str, (&str, &str))>)> {
    separated_pair(parse_directions, line_ending, parse_graph)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (directions, graph)) = parse(input).unwrap();
    let mut i = 0;
    let mut pos = graph.get("AAA")?;
    for _ in 0..100 {
        for d in directions.iter() {
            i += 1;
            let n = next(pos.1, *d);
            if n == "ZZZ" {
                return Some(i);
            };
            pos = graph.get(n)?;
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, (directions, graph)) = parse(input).unwrap();
    let pos = graph
        .keys()
        .filter(|&x| x.ends_with('A'))
        .map(|&x| graph.get(x).unwrap())
        .collect::<Vec<_>>();

    let res = pos
        .iter()
        .map(|&node| {
            let mut current = *node;

            directions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(idx, d)| {
                    let opt = graph.get(current.0).expect("no node");
                    let next = next(opt.1, *d);
                    if next.ends_with('Z') {
                        Some(idx + 1)
                    } else {
                        current = graph.get(next).expect("no node").to_owned();
                        None
                    }
                })
                .expect("should be a cycle")
        })
        .collect::<Vec<usize>>();

    Some(lcm(&res))
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        println!("{}", gcd_of_two_numbers(3, 4));
    }

    #[test]
    fn test_parse_graph() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = parse(input);
        println!("{:?}", result);
    }

    #[test]
    fn test_parse_entry() {
        let input = "AAA = (BBB, BBB)";
        let result = parse_entry(input);
        println!("{:?}", result);
    }

    #[test]
    fn test_cycle() {
        let input = "LR

11A = (11C, XXX)
11B = (XXX, 11Z)
11C = (XXX, 11D)
11D = (11B, XXX)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let result = part_two(input);
        println!("{result:?}");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
