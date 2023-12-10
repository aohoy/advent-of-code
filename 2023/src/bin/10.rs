use core::num;
use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(10);

// .....
// .F-7.
// .|.|.
// .L-J.
// .....

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Debug)]
enum Cell {
    LeftRight, // -
    LeftDown,  // 7
    UpDown,    // |
    UpLeft,    // J
    RightUp,   // L
    DownRight, // F
    Dot,       // .
    Animal,    // S
}

impl Cell {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '-' => Some(Cell::LeftRight),
            '|' => Some(Cell::UpDown),
            '7' => Some(Cell::LeftDown),
            'F' => Some(Cell::DownRight),
            'J' => Some(Cell::UpLeft),
            'L' => Some(Cell::RightUp),
            '.' => Some(Cell::Dot),
            'S' => Some(Cell::Animal),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Cell::LeftRight => '-',
            Cell::UpDown => '|',
            Cell::LeftDown => '7',
            Cell::DownRight => 'F',
            Cell::UpLeft => 'J',
            Cell::RightUp => 'L',
            Cell::Dot => '.',
            Cell::Animal => 'S',
        }
    }

    fn direction(&self, from: &Direction) -> Option<Direction> {
        // println!("-- {:?} {:?}", self, from);
        match (from, self) {
            (Direction::Right, Cell::LeftRight) => Some(Direction::Right),
            (Direction::Left, Cell::LeftRight) => Some(Direction::Left),

            (Direction::Up, Cell::UpDown) => Some(Direction::Up),
            (Direction::Down, Cell::UpDown) => Some(Direction::Down),

            (Direction::Right, Cell::LeftDown) => Some(Direction::Down),
            (Direction::Up, Cell::LeftDown) => Some(Direction::Left),

            (Direction::Left, Cell::DownRight) => Some(Direction::Down),
            (Direction::Up, Cell::DownRight) => Some(Direction::Right),

            (Direction::Down, Cell::UpLeft) => Some(Direction::Left),
            (Direction::Right, Cell::UpLeft) => Some(Direction::Up),

            (Direction::Left, Cell::RightUp) => Some(Direction::Up),
            (Direction::Down, Cell::RightUp) => Some(Direction::Right),

            _ => None,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut current_loc = (0, 0);
    let mut map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        current_loc = (i, j)
                    };
                    Cell::from_char(c).unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_i = map.len() - 1;
    let max_j = map[0].len() - 1;

    // find first direction
    let mut dir = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    .find_map(|&d| match (current_loc, d) {
        ((0, _), Direction::Up) => None,
        ((i, _), Direction::Down) if i == max_i => None,
        ((_, 0), Direction::Left) => None,
        ((_, j), Direction::Right) if j == max_j => None,
        _ => {
            let offset = d.to_offset();
            let cell = &map[(current_loc.0 as i32 + offset.0) as usize]
                [(current_loc.1 as i32 + offset.1) as usize];
            println!("- {:?} -> {:?}", cell, d);
            cell.direction(&d).map(|_| d)
        }
    })
    .unwrap();

    println!("{:?}", current_loc);
    println!("{:?}", dir);
    input
        .lines()
        .for_each(|line| println!("{:?}", line.chars().collect::<Vec<_>>()));

    for steps in 0..((max_i + 1) * (max_j + 1)) {
        let dir_offset = dir.to_offset();
        println!("{:?} -- {:?}", current_loc, dir_offset);
        current_loc = (
            (current_loc.0 as i32 + dir_offset.0) as usize,
            (current_loc.1 as i32 + dir_offset.1) as usize,
        );
        let current = &map[current_loc.0][current_loc.1];
        if let Cell::Animal = current {
            return Some((steps as u32 + 1) / 2);
        }
        dir = current.direction(&dir).unwrap();
        println!("{:?} -> {:?}", current.to_char(), dir);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut current_loc = (0, 0);
    let mut map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        current_loc = (i, j)
                    };
                    Cell::from_char(c).unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_i = map.len() - 1;
    let max_j = map[0].len() - 1;

    // find first direction
    let dirs = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    .filter_map(|&d| match (current_loc, d) {
        ((0, _), Direction::Up) => None,
        ((i, _), Direction::Down) if i == max_i => None,
        ((_, 0), Direction::Left) => None,
        ((_, j), Direction::Right) if j == max_j => None,
        _ => {
            let offset = d.to_offset();
            let cell = &map[(current_loc.0 as i32 + offset.0) as usize]
                [(current_loc.1 as i32 + offset.1) as usize];
            println!("- {:?} -> {:?}", cell, d);
            cell.direction(&d).map(|_| d)
        }
    })
    .collect_tuple::<(Direction, Direction)>()
    .unwrap();

    let mut dir = dirs.0;

    map[current_loc.0][current_loc.1] = match dirs {
        (Direction::Up, Direction::Down) => Cell::UpDown,
        (Direction::Left, Direction::Right) => Cell::UpDown,
        (Direction::Up, Direction::Left) => Cell::UpLeft,
        (Direction::Up, Direction::Right) => Cell::RightUp,
        (Direction::Down, Direction::Left) => Cell::LeftDown,
        (Direction::Down, Direction::Right) => Cell::DownRight,
        _ => panic!("no direction"),
    };

    map.iter()
        .for_each(|line| println!("{:?}", line.iter().map(|x| x.to_char()).collect::<Vec<_>>()));

    let mut loop_cells = HashSet::from([current_loc]);

    let initial_loc = current_loc;

    for _ in 0..((max_i + 1) * (max_j + 1)) {
        let dir_offset = dir.to_offset();
        current_loc = (
            (current_loc.0 as i32 + dir_offset.0) as usize,
            (current_loc.1 as i32 + dir_offset.1) as usize,
        );
        if current_loc == initial_loc {
            break;
        }
        loop_cells.insert(current_loc);
        let current = &map[current_loc.0][current_loc.1];
        dir = current.direction(&dir).unwrap();
    }

    let mut inside = 0;

    for (i, line) in map.iter().enumerate() {
        let mut num_of_walls = 0;
        let mut prev_corner = None;
        for (j, c) in line.iter().enumerate() {
            match c {
                _ if !loop_cells.contains(&(i, j)) => {
                    if num_of_walls % 2 != 0 {
                        inside += 1;
                    }
                }
                Cell::LeftRight => (),
                Cell::RightUp | Cell::DownRight => prev_corner = Some(c),
                Cell::LeftDown => {
                    match prev_corner {
                        Some(Cell::RightUp) => num_of_walls += 1,
                        _ => num_of_walls += 2,
                    }
                    prev_corner = None;
                }
                Cell::UpLeft => {
                    match prev_corner {
                        Some(Cell::DownRight) => num_of_walls += 1,
                        _ => num_of_walls += 2,
                    }
                    prev_corner = None;
                }
                Cell::UpDown => num_of_walls += 1,
                _ => panic!("no direction"),
            };
        }
    }

    Some(inside)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    // OF----7F7F7F7F-7OOOO
    // O|F--7||||||||FJOOOO
    // O||OFJ||||||||L7OOOO
    // FJL7L7LJLJ||LJIL-7OO
    // L--JOL7IIILJS7F-7L7O
    // OOOOF-JIIF7FJ|L7L7L7
    // OOOOL7IF7||L7|IL7L7|
    // OOOOO|FJLJ|FJ|F7|OLJ
    // OOOOFJL-7O||O||||OOO
    // OOOOL---JOLJOLJLJOOO

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
