use std::collections::HashMap;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let results = input
        .lines()
        .take(2)
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let res = results[0]
        .iter()
        .zip(results[1].iter())
        .fold(1u32, |acc, (&t, &s)| {
            if acc == 0 {
                return 0;
            }

            let d = t.pow(2) - 4 * s;

            if d <= 0 {
                0
            } else {
                let d_sqr = f32::sqrt(d as f32);
                let nearest_sqr = d_sqr.floor() as i32;
                let squares = (
                    (nearest_sqr, (nearest_sqr).pow(2)),
                    (nearest_sqr + 1, (nearest_sqr + 1).pow(2)),
                );
                if d == squares.0 .1 || d == squares.1 .1 {
                    let x = if d == squares.0 .1 {
                        squares.0 .0
                    } else {
                        squares.1 .0
                    };
                    return acc * ((t + x) / 2 - (t - x) / 2 - 1) as u32;
                }

                let left_root = (t as f32 - d_sqr) / 2.0;
                let right_root = (t as f32 + d_sqr) / 2.0;

                acc * (right_root.floor() as u32 - left_root.ceil() as u32 + 1)
            }
        });
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let results = input
        .lines()
        .take(2)
        .map(|line| line.split_ascii_whitespace().skip(1).collect::<String>())
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let t = results[0];
    let s = results[1];

    let d = t.pow(2) - 4 * s;

    if d <= 0 {
        Some(0)
    } else {
        let d_sqr = f64::sqrt(d as f64);
        let nearest_sqr = d_sqr.floor() as i64;
        let squares = (
            (nearest_sqr, (nearest_sqr).pow(2)),
            (nearest_sqr + 1, (nearest_sqr + 1).pow(2)),
        );
        if d == squares.0 .1 || d == squares.1 .1 {
            let x = if d == squares.0 .1 {
                squares.0 .0
            } else {
                squares.1 .0
            };
            return Some(((t + x) / 2 - (t - x) / 2 - 1) as u64);
        }

        let left_root = (t as f64 - d_sqr) / 2.0;
        let right_root = (t as f64 + d_sqr) / 2.0;

        Some(right_root.floor() as u64 - left_root.ceil() as u64 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inp() {
        let _ = part_one(&advent_of_code::template::read_file("examples", DAY));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
