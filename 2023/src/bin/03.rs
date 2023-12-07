use std::collections::{BTreeMap, BTreeSet};

advent_of_code::solution!(3);

#[derive(Debug)]
enum Value {
    Dot,
    Symbol,
    Star,
    Num(u32),
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut prev_num: u32 = 0;
            line.chars()
                .enumerate()
                .map(move |(x, ch)| {
                    (
                        (y as i32, x as i32),
                        match ch {
                            '0'..='9' => {prev_num = prev_num * 10 + ch.to_digit(10).unwrap(); Value::Num(prev_num)},
                            '.' => {prev_num = 0; Value::Dot},
                            _ => {prev_num = 0; Value::Symbol},
                        }
                    )
                })
                
        })
        .collect::<BTreeMap<(i32,i32), Value>>();
    let mut visited = BTreeSet::<(i32,i32)>::new();
    let (_, line) = input.lines().enumerate().last().unwrap();
    let size_x: i32 = line.len().try_into().unwrap();

    map
        .iter()
        .flat_map(|((y, x), val)| {
            let pos = [
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1),           (0, 1),
                (1, -1),  (1, 0),  (1, 1),
            ];
            let mut res: Vec<u32> = vec![];
            if let Value::Symbol = val {
                res = pos.iter().filter_map(|(yp, xp)| {
                    match map.get(&(y + yp, x + xp)).unwrap() {
                        Value::Num(_) => {
                            let fx = (x+xp..size_x)
                                .take_while(|&xf| {
                                    match map.get(&(y + yp, xf)) {
                                        Some(Value::Num(_)) => true,
                                        _ => false,
                                    }
                                })
                                .last()
                                .and_then(|xf| if !visited.contains(&(y + yp, xf)) {visited.insert((y+yp, xf)); Some(xf)} else {None} );
                            match fx {
                                Some(fx) => match map.get(&(y + yp, fx)).unwrap() {
                                    Value::Num(fnum) => Some(*fnum),
                                    _ => None,
                                },
                                None => None,
                            }
                            
                        },
                        _ => None,
                    }
                })
                .collect::<Vec<u32>>();
            };
            res
        })
        .reduce(|acc, el| acc + el)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut prev_num: u32 = 0;
            line.chars()
                .enumerate()
                .map(move |(x, ch)| {
                    (
                        (y as i32, x as i32),
                        match ch {
                            '0'..='9' => {prev_num = prev_num * 10 + ch.to_digit(10).unwrap(); Value::Num(prev_num)},
                            '.' => {prev_num = 0; Value::Dot},
                            '*' => {prev_num = 0; Value::Star},
                            _ => {prev_num = 0; Value::Symbol},
                        }
                    )
                })
                
        })
        .collect::<BTreeMap<(i32,i32), Value>>();
    let mut visited = BTreeSet::<(i32,i32)>::new();
    let (_, line) = input.lines().enumerate().last().unwrap();
    let size_x: i32 = line.len().try_into().unwrap();
    map
        .iter()
        .flat_map(|((y, x), val)| {
            let pos = [
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1),           (0, 1),
                (1, -1),  (1, 0),  (1, 1),
            ];
            let mut res: Vec<u32> = vec![];
            if let Value::Star = val {
                res = pos.iter().filter_map(|(yp, xp)| {
                    match map.get(&(y + yp, x + xp)).unwrap() {
                        Value::Num(_) => {
                            let fx = (x+xp..size_x)
                                .take_while(|&xf| {
                                    match map.get(&(y + yp, xf)) {
                                        Some(Value::Num(_)) => true,
                                        _ => false,
                                    }
                                })
                                .last()
                                .and_then(|xf| if !visited.contains(&(y + yp, xf)) {visited.insert((y+yp, xf)); Some(xf)} else {None} );
                            match fx {
                                Some(fx) => match map.get(&(y + yp, fx)).unwrap() {
                                    Value::Num(fnum) => Some(*fnum),
                                    _ => None,
                                },
                                None => None,
                            }
                            
                        },
                        _ => None,
                    }
                })
                .collect::<Vec<u32>>();
                if res.len() != 2 {
                    res = vec![];
                } else {
                    res = vec![res.iter().product()]
                }
            };
            res
        })
        .reduce(|acc, el| acc + el)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
