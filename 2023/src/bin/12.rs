use std::{collections::HashMap, iter};

use itertools::Itertools;

advent_of_code::solution!(12);

fn combinations<'a>(
    chars: &'a [char],
    numbers: &'a [u64],
    mem: &mut HashMap<(&'a [u64], &'a [char]), u64>,
) -> u64 {
    if chars.is_empty() {
        if numbers.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    let next_combinations_same_instr = combinations(&chars[1..], numbers, mem);

    match chars[0] {
        '.' => next_combinations_same_instr,
        '#' => {
            if let Some(&curr_combinations) = mem.get(&(&numbers, &chars)) {
                return curr_combinations;
            }

            if numbers.is_empty() {
                return 0;
            }

            let wanted_spring_len = numbers[0] as usize;
            if chars.len() < wanted_spring_len || chars[0..wanted_spring_len].contains(&'.') {
                return 0;
            } else if chars.len() == wanted_spring_len {
                if numbers.len() == 1 {
                    return 1;
                }
                return 0;
            } else if chars[wanted_spring_len] == '#' {
                return 0;
            }

            let next_combinations_next_instr: u64 =
                combinations(&chars[(wanted_spring_len + 1)..], &numbers[1..], mem);

            mem.insert((&numbers, &chars), next_combinations_next_instr);

            next_combinations_next_instr
        }
        '?' => {
            if let Some(&curr_combinations) = mem.get(&(&numbers, &chars)) {
                return curr_combinations + next_combinations_same_instr;
            }

            if numbers.is_empty() {
                return next_combinations_same_instr;
            }

            let wanted_spring_len = numbers[0] as usize;
            if chars.len() < wanted_spring_len || chars[0..wanted_spring_len].contains(&'.') {
                return next_combinations_same_instr;
            } else if chars.len() == wanted_spring_len {
                if numbers.len() == 1 {
                    return 1 + next_combinations_same_instr;
                }

                return next_combinations_same_instr;
            } else if chars[wanted_spring_len] == '#' {
                return next_combinations_same_instr;
            }

            let next_combinations_next_instr: u64 =
                combinations(&chars[(wanted_spring_len + 1)..], &numbers[1..], mem);

            mem.insert((&numbers, &chars), next_combinations_next_instr);

            next_combinations_next_instr + next_combinations_same_instr
        }
        _ => panic!("Invalid spring"),
    }
}

fn solve_line(line: &str) -> Option<u64> {
    let (template, numbers) = line.split_once(' ').unwrap();

    let template: Vec<char> = template.chars().collect();
    let numbers: Vec<u64> = numbers.split(',').map(|x| x.parse().unwrap()).collect();
    let mut mem = HashMap::new();

    Some(combinations(&template, &numbers, &mut mem))
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
