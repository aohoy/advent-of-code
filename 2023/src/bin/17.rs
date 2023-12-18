use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    ops::Add,
};

use glam::{IVec2, UVec2};
use pathfinding::directed::dijkstra::{self, dijkstra};

advent_of_code::solution!(17);

#[derive(Debug, Clone)]
struct Node {
    heat: u32,
    weight: u32,
    pos: IVec2,
    tail: Box<VecDeque<IVec2>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        other.weight == self.weight
    }
}

impl Eq for Node {}

enum Direction {
    RIGHT,
    UP,
    DOWN,
}

fn find_path(blocks: &[Vec<Node>]) -> u32 {
    let end = IVec2::new(blocks.len() as i32 - 1, blocks[0].len() as i32 - 1);

    let res = dijkstra(
        &(IVec2::splat(0), VecDeque::from([IVec2::splat(0)])),
        |(pos, tail)| {
            [
                IVec2::new(1, 0),
                IVec2::new(-1, 0),
                IVec2::new(0, 1),
                IVec2::new(0, -1),
            ]
            .into_iter()
            .filter_map(|diff| {
                let new_pos = *pos + diff;

                if new_pos.x.clamp(0, end.x) != new_pos.x || new_pos.y.clamp(0, end.y) != new_pos.y
                {
                    return None;
                }

                if tail.len() > 1 && new_pos == tail[1] {
                    return None;
                }

                let mut new_tail = tail.clone();
                new_tail.push_front(new_pos);

                if new_tail.len() == 5 {
                    let dir = new_tail[1] - new_tail[0];
                    new_tail
                        .iter()
                        .enumerate()
                        .skip(2)
                        .find_map(|(i, &p)| (p - new_tail[i - 1] != dir).then_some(()))?;
                    new_tail.pop_back();
                }

                Some((new_pos, new_tail))
            })
            .map(|n| (n.clone(), blocks[n.0.x as usize][n.0.y as usize].heat))
            .collect::<Vec<_>>()
        },
        |(pos, _)| pos == &end,
    )
    .expect("No path found");

    for v in res.0 {
        println!("{:?}", v);
    }

    res.1
}

pub fn part_one(input: &str) -> Option<u32> {
    let blocks: Vec<Vec<Node>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| Node {
                    heat: c.to_digit(10).unwrap(),
                    weight: u32::MAX,
                    pos: IVec2::new(i as i32, j as i32),
                    tail: Box::default(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Some(find_path(&blocks))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    use glam::IVec2;

    use super::*;

    #[test]
    fn test_dijkstra() {
        let input = "21131
91195
99993";

        let res = part_one(input).unwrap();
        println!("{:?}", res);
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
