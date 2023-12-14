use std::{collections::HashMap, fmt::Display};

use itertools::iproduct;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut field = Field {
        x: input.lines().count(),
        y: input.find('\n').unwrap(),
        field: input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate().filter_map(move |(j, c)| match c {
                    'O' => Some(((i, j), Rock::Movable)),
                    '#' => Some(((i, j), Rock::Immovable)),
                    _ => None,
                })
            })
            .collect(),
    };

    field.spin_to_north();
    Some(field.weight())
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Clone, PartialEq)]
enum Rock {
    Movable,
    Immovable,
}

struct Field {
    x: usize,
    y: usize,
    field: HashMap<(usize, usize), Rock>,
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.x {
            for j in 0..self.y {
                match self.field.get(&(i, j)) {
                    Some(Rock::Movable) => write!(f, "O")?,
                    Some(Rock::Immovable) => write!(f, "#")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Field {
    fn weight(&self) -> u32 {
        self.field
            .iter()
            .filter(|(_, rock)| **rock == Rock::Movable)
            .map(|(pos, _)| self.x - pos.0)
            .sum::<usize>() as u32
    }

    fn spin_to_north(&mut self) {
        let mut positions = vec![0usize; self.y];

        for (i, j) in iproduct!(0..self.x, 0..self.y) {
            match self.field.get(&(i, j)) {
                Some(Rock::Movable) => {
                    if i != positions[j] {
                        // println!("Remove {:?} and set it to {:?}", (i, j), (positions[j], j));
                        self.field.remove(&(i, j));
                        self.field.insert((positions[j], j), Rock::Movable);
                    }
                    positions[j] += 1;
                }
                Some(Rock::Immovable) => {
                    positions[j] = i + 1;
                }
                None => {}
            }
        }
    }

    fn spin_to_south(&mut self) {
        let mut positions = vec![self.x - 1; self.y];

        for (i, j) in iproduct!((0..self.x).rev(), 0..self.y) {
            match self.field.get(&(i, j)) {
                Some(Rock::Movable) => {
                    if i != positions[j] {
                        // println!("Remove {:?} and set it to {:?}", (i, j), (positions[j], j));
                        self.field.remove(&(i, j));
                        self.field.insert((positions[j], j), Rock::Movable);
                    }
                    (i > 0).then(|| positions[j] -= 1);
                }
                Some(Rock::Immovable) => {
                    (i > 0).then(|| positions[j] = i - 1);
                }
                None => {}
            }
        }
    }

    fn spin_to_west(&mut self) {
        let mut positions = vec![0; self.x];

        for (i, j) in iproduct!(0..self.x, 0..self.y) {
            match self.field.get(&(i, j)) {
                Some(Rock::Movable) => {
                    if j != positions[i] {
                        // println!("Remove {:?} and set it to {:?}", (i, j), (positions[j], j));
                        self.field.remove(&(i, j));
                        self.field.insert((i, positions[i]), Rock::Movable);
                    }
                    positions[i] += 1;
                }
                Some(Rock::Immovable) => {
                    positions[i] = j + 1;
                }
                None => {}
            }
        }
    }

    fn spin_to_east(&mut self) {
        let mut positions = vec![self.y - 1; self.x];

        for (i, j) in iproduct!(0..self.x, (0..self.y).rev()) {
            match self.field.get(&(i, j)) {
                Some(Rock::Movable) => {
                    if j != positions[i] {
                        // println!("Remove {:?} and set it to {:?}", (i, j), (positions[j], j));
                        self.field.remove(&(i, j));
                        self.field.insert((i, positions[i]), Rock::Movable);
                    }
                    (j > 0).then(|| positions[i] -= 1);
                }
                Some(Rock::Immovable) => {
                    (j > 0).then(|| positions[i] = j - 1);
                }
                None => {}
            }
        }
    }

    fn spin(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.spin_to_north(),
            Direction::West => self.spin_to_west(),
            Direction::South => self.spin_to_south(),
            Direction::East => self.spin_to_east(),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut field = Field {
        x: input.lines().count(),
        y: input.find('\n').unwrap(),
        field: input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate().filter_map(move |(j, c)| match c {
                    'O' => Some(((i, j), Rock::Movable)),
                    '#' => Some(((i, j), Rock::Immovable)),
                    _ => None,
                })
            })
            .collect(),
    };

    let rounds = 1_000_000_000;
    let mut cycle_history = Vec::new();
    let Some(loop_start) = (0..rounds).find_map(|_| {
        cycle_history.push(field.field.clone());
        field.spin(Direction::North);
        field.spin(Direction::West);
        field.spin(Direction::South);
        field.spin(Direction::East);
        cycle_history.iter().position(|prev| *prev == field.field)
    }) else {
        return Some(field.weight());
    };

    let loop_length = cycle_history.len() - loop_start;
    let solution_index = loop_start + ((rounds - loop_start) % loop_length);

    field.field = cycle_history[solution_index].clone();

    Some(field.weight())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
