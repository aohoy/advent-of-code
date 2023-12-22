use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending, one_of, space0},
    combinator::{complete, map_res, opt},
    multi::{fold_many1, many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple, Tuple},
    IResult,
};
use pathfinding::{grid, num_traits::Pow};

advent_of_code::solution!(21);

const DOWN: IVec2 = IVec2::new(1, 0);
const UP: IVec2 = IVec2::new(-1, 0);
const LEFT: IVec2 = IVec2::new(0, -1);
const RIGHT: IVec2 = IVec2::new(0, 1);

enum Cell {
    Garden,
    Position,
    Rock,
}

impl TryFrom<char> for Cell {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Cell::Rock),
            '.' => Ok(Cell::Garden),
            'S' => Ok(Cell::Position),
            _ => Err(()),
        }
    }
}

// impl From<char> for Cell {
//     fn from(value: char) -> Self {
//         match value {
//             '#' => Cell::Rock,
//             '.' => Cell::Garden,
//             'S' => Cell::Position,
//             _ => panic!("Invalid cell"),
//         }
//     }
// }

fn parse(input: &str) -> IResult<&str, Vec<Vec<Cell>>> {
    separated_list1(line_ending, many1(map_res(one_of(".#S"), Cell::try_from)))(input)
}

fn walk(grid: &[Vec<Cell>], start: IVec2, steps_count: i32) -> i32 {
    let size = (grid.len() as i32 - 1, grid[0].len() as i32 - 1);
    let mut current_positions = HashSet::from([start]);

    for step in 1..=steps_count {
        let mut new_positions = HashSet::new();

        for pos in current_positions {
            for dir in [DOWN, UP, LEFT, RIGHT] {
                let new_pos = pos + dir;
                if new_pos.x.clamp(0, size.0) != new_pos.x
                    || new_pos.y.clamp(0, size.1) != new_pos.y
                    || matches!(grid[new_pos.x as usize][new_pos.y as usize], Cell::Rock)
                {
                    continue;
                }
                // println!("{} -> {}", pos, new_pos);
                new_positions.insert(new_pos);
            }
        }

        current_positions = new_positions;
    }
    current_positions.len() as i32
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, grid) = parse(input).ok()?;

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(x, row)| {
            row.iter()
                .position(|cell| matches!(cell, Cell::Position))
                .map(|y| IVec2::new(x as i32, y as i32))
        })
        .expect("No start position");

    Some(walk(&grid, start, 6) as u32)
}

fn walk_infinitely(grid: &[Vec<Cell>], start: IVec2, steps_count: i32) -> u64 {
    let size = IVec2::new(grid.len() as i32, grid[0].len() as i32);
    let half = size.x / 2;
    let mut current_positions = HashSet::from([start]);
    let mut results = Vec::new();

    for step in 1.. {
        let mut new_positions = HashSet::new();

        for pos in current_positions {
            for dir in [DOWN, UP, LEFT, RIGHT] {
                let new_pos = (pos + dir).rem_euclid(size);
                if matches!(grid[new_pos.x as usize][new_pos.y as usize], Cell::Rock) {
                    continue;
                }
                // println!("{} -> {}", pos, new_pos);
                new_positions.insert(new_pos);
            }
        }

        current_positions = new_positions;

        if step == half + size.x * results.len() as i32 {
            println!("Add: {}", current_positions.len());
            results.push(current_positions.len() as i32);

            if results.len() == 3 {
                let (d0, d1, d2) = (
                    results[0] as u64,
                    (results[1] - results[0]) as u64,
                    (results[2] - results[1]) as u64,
                );

                let div = (steps_count / size.x) as u64;
                return d0 + d1 * div + (div * (div - 1) / 2) * (d2 - d1);
            }
        }
    }
    0
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, grid) = parse(input).ok()?;

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(x, row)| {
            row.iter()
                .position(|cell| matches!(cell, Cell::Position))
                .map(|y| IVec2::new(x as i32, y as i32))
        })
        .expect("No start position");

    let steps_count = 26_501_365;
    Some(walk_infinitely(&grid, start, steps_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
