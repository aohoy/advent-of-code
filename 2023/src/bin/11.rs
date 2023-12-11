use itertools::Itertools;
use std::collections::BTreeSet;

advent_of_code::solution!(11);

fn fold_columns(
    set: &BTreeSet<usize>,
    expansion: u64,
) -> impl FnMut(Vec<u64>, usize) -> Vec<u64> + '_ {
    move |mut acc: Vec<u64>, el: usize| {
        let mut i = if el == 0 { 0u64 } else { acc[el - 1] };
        if set.contains(&el) {
            i += expansion - 1;
        }
        acc.push(i);
        acc
    }
}

fn solve(input: &str, expansion: u64) -> Option<u64> {
    let fields = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut empty_cols = BTreeSet::from_iter(0..fields.len());
    let mut empty_rows = BTreeSet::from_iter(0..fields[0].len());

    fields.iter().enumerate().for_each(|(i, col)| {
        col.iter().enumerate().for_each(|(j, &c)| {
            if c == '#' {
                empty_cols.remove(&i);
                empty_rows.remove(&j);
            }
        })
    });

    let cols = (0..fields.len()).fold(
        Vec::with_capacity(fields.len()),
        fold_columns(&empty_cols, expansion),
    );
    let rows = (0..fields[0].len()).fold(
        Vec::with_capacity(fields[0].len()),
        fold_columns(&empty_rows, expansion),
    );

    let galaxies = fields
        .iter()
        .enumerate()
        .flat_map(|(i, col)| {
            col.iter()
                .enumerate()
                .filter_map(|(j, &c)| {
                    if c == '#' {
                        Some((cols[i] + i as u64, j as u64 + rows[j]))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect::<Vec<_>>();

    println!("{cols:?}");
    println!("{rows:?}");
    println!("{galaxies:?}");

    let res = galaxies.iter().combinations(2).fold(0u64, |acc, el| {
        acc + el[0].0.abs_diff(el[1].0) + el[0].1.abs_diff(el[1].1)
    });
    Some(res)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1030));
    }
}
