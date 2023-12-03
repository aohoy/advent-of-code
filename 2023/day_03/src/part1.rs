use std::{error::Error, collections::{BTreeMap, BTreeSet}};

#[derive(Debug)]
enum Value {
    Dot,
    Symbol,
    Num(u32),
}

pub fn solve(_input: &str) -> Result<String, Box<dyn Error>> {
    let map = _input
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
    let (_, line) = _input.lines().enumerate().last().unwrap();
    let size_x: i32 = line.len().try_into().unwrap();
    let res = map
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
        .sum::<u32>();
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", solve(input)?);
        Ok(())
    }
}
