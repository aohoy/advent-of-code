use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
};

use glam::I64Vec2;

advent_of_code::solution!(18);

const DOWN: I64Vec2 = I64Vec2::new(1, 0);
const UP: I64Vec2 = I64Vec2::new(-1, 0);
const LEFT: I64Vec2 = I64Vec2::new(0, -1);
const RIGHT: I64Vec2 = I64Vec2::new(0, 1);

fn shovels_area(instructions: &[(I64Vec2, i64)]) -> i64 {
    let (_, area, perim) =
        instructions
            .iter()
            .fold((I64Vec2::ZERO, 0, 0), |(pos, area, perim), (dir, steps)| {
                let new_pos = pos + *dir * *steps;
                let new_area = area + (pos.x * new_pos.y - new_pos.x * pos.y);
                let new_perim = (new_pos.x - pos.x).abs() + (new_pos.y - pos.y).abs() + perim;
                (new_pos, new_area, new_perim)
            });
    (area.abs() + perim.abs()) / 2 + 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let (dir, line) = line.split_once(' ').expect("no space");
            let (steps, _) = line.split_once(' ').expect("no steps");
            let steps = steps.parse::<i64>().expect("not a number");

            let dir = match dir {
                "D" => DOWN,
                "U" => UP,
                "L" => LEFT,
                "R" => RIGHT,
                _ => panic!("unknown direction"),
            };
            (dir, steps)
        })
        .collect();

    Some(shovels_area(&instructions) as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(' ').expect("no space");
            let (_, instr) = line.split_once(' ').expect("no steps");
            let (steps, dir) = instr
                .strip_prefix('(')
                .unwrap()
                .strip_prefix('#')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_at(5);
            let steps = i64::from_str_radix(steps, 16).expect("not a number");

            let dir = match dir {
                "1" => DOWN,
                "3" => UP,
                "2" => LEFT,
                "0" => RIGHT,
                _ => panic!("unknown direction"),
            };
            (dir, steps)
        })
        .collect();

    Some(shovels_area(&instructions) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
