use std::{collections::HashMap, iter};

use itertools::Itertools;

advent_of_code::solution!(12);

fn operational<'a>(
    chars: &'a [char],
    numbers: &'a [u32],
    count: u32,
    cache: &mut HashMap<(&'a [char], &'a [u32], u32), u64>,
) -> u64 {
    if numbers.first().is_some_and(|x| *x == count) {
        permutations(&chars[1..], &numbers[1..], 0, cache)
    } else if count == 0 {
        permutations(&chars[1..], numbers, 0, cache)
    } else {
        0
    }
}

fn damaged<'a>(
    chars: &'a [char],
    numbers: &'a [u32],
    count: u32,
    cache: &mut HashMap<(&'a [char], &'a [u32], u32), u64>,
) -> u64 {
    if numbers.first().map_or(true, |x| *x <= count) {
        0
    } else {
        permutations(&chars[1..], numbers, count + 1, cache)
    }
}

fn permutations<'a>(
    chars: &'a [char],
    numbers: &'a [u32],
    count: u32,
    cache: &mut HashMap<(&'a [char], &'a [u32], u32), u64>,
) -> u64 {
    if let Some(cached) = cache.get(&(chars, numbers, count)) {
        return *cached;
    }

    let num_of_permutations = match chars.first() {
        Some('.') => operational(chars, numbers, count, cache),
        Some('#') => damaged(chars, numbers, count, cache),
        Some('?') => {
            operational(chars, numbers, count, cache) + damaged(chars, numbers, count, cache)
        }
        None if numbers.is_empty() || numbers == [count] => 1,
        None => 0,
        Some(_) => panic!("wrong character"),
    };

    cache.insert((chars, numbers, count), num_of_permutations);
    num_of_permutations
}

fn solve_line(line: &str) -> Option<u64> {
    let (template, numbers) = line.split_once(' ').unwrap();

    let template: Vec<char> = template.chars().collect();
    let numbers: Vec<u32> = numbers.split(',').map(|x| x.parse().unwrap()).collect();
    let mut mem = HashMap::new();

    Some(permutations(&template, &numbers, 0, &mut mem))
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input.lines().map(|line| solve_line(line).unwrap()).sum();
    Some(res)
}

fn expand_line(input: &str) -> String {
    let (template, numbers) = input.split_once(' ').unwrap();
    let temlate = iter::repeat(template).take(5).join("?");
    let numbers = iter::repeat(numbers).take(5).join(",");
    format!("{temlate} {numbers}")
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|line| solve_line(&expand_line(line)).unwrap())
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expanded_line() {
        let input = ".??..??...?##. 1,1,3";
        let result = solve_line(&expand_line(input));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
