use std::collections::HashSet;

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Direction(i32, i32);

const UP: Direction = Direction(-1, 0);
const DOWN: Direction = Direction(1, 0);
const LEFT: Direction = Direction(0, -1);
const RIGHT: Direction = Direction(0, 1);
const ANY: Direction = Direction(0, 0);

enum Cell {
    Empty,
    Mirror,
    MirrorBack,
    SplitHor,
    SplitVert,
}

impl TryFrom<char> for Cell {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            '/' => Ok(Cell::Mirror),
            '\\' => Ok(Cell::MirrorBack),
            '|' => Ok(Cell::SplitVert),
            '-' => Ok(Cell::SplitHor),
            _ => Err(()),
        }
    }
}

enum ResultDirection {
    Unidirectional(Direction),
    Bidirectional(Direction, Direction),
}

impl Cell {
    fn next_direction(&self, from: Direction) -> ResultDirection {
        match self {
            Cell::Empty => ResultDirection::Unidirectional(from),
            Cell::Mirror => ResultDirection::Unidirectional(Direction(-from.1, -from.0)),
            Cell::MirrorBack => ResultDirection::Unidirectional(Direction(from.1, from.0)),
            Cell::SplitHor if from.0 == 0 => ResultDirection::Unidirectional(from),
            Cell::SplitHor => ResultDirection::Bidirectional(LEFT, RIGHT),
            Cell::SplitVert if from.1 == 0 => ResultDirection::Unidirectional(from),
            Cell::SplitVert => ResultDirection::Bidirectional(UP, DOWN),
        }
    }
}

fn walk(
    field: &[Vec<Cell>],
    mut pos: (i32, i32),
    mut dir: Direction,
    cache: &mut HashSet<(Direction, (i32, i32))>,
) -> u32 {
    let mut steps = 0;

    let (height, width) = (field.len() as i32 - 1, field[0].len() as i32 - 1);

    if pos.0.clamp(0, height) != pos.0 || pos.1.clamp(0, width) != pos.1 {
        return steps;
    }

    loop {
        // println!("Current pos: {:?}, dir: {:?}, steps: {}", pos, dir, steps);
        if cache.contains(&(dir, pos)) {
            return steps;
        }
        cache.insert((dir, pos));

        if !cache.contains(&(ANY, pos)) {
            steps += 1;
            cache.insert((ANY, pos));
        }

        let cell = &field[pos.0 as usize][pos.1 as usize];
        match cell.next_direction(dir) {
            ResultDirection::Unidirectional(new_dir) => {
                dir = new_dir;
                pos = (pos.0 + dir.0, pos.1 + dir.1);
                // println!("-- New pos: {:?}, dir: {:?}, steps: {}", pos, dir, steps);
                if pos.0.clamp(0, height) != pos.0 || pos.1.clamp(0, width) != pos.1 {
                    break;
                }

                if cache.contains(&(dir, pos)) {
                    break;
                }
            }
            ResultDirection::Bidirectional(new_dir_1, new_dir_2) => {
                // println!("Found bidirectional");
                steps += walk(field, pos, new_dir_1, cache) + walk(field, pos, new_dir_2, cache);
                break;
            }
        }
    }

    steps
}

pub fn part_one(input: &str) -> Option<u32> {
    let field: Vec<Vec<Cell>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Cell::try_from(c).expect("wrong input"))
                .collect::<Vec<Cell>>()
        })
        .collect();

    let mut cache = HashSet::new();
    Some(walk(&field, (0, 0), RIGHT, &mut cache))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
