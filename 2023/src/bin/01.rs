advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input.lines()
        .map(|line| {
            let mut chars = line
                .chars()
                .filter_map(|x| x.to_digit(10));
            let first = chars.next().unwrap();
            let last = match chars.last() {
                Some(x) => x,
                None => first
            };
            first * 10 + last
        })
        .reduce(|acc, e| { acc + e })
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines()
        .map(|line| {
            let mut chars = line
                .char_indices()
                .filter_map(|(i, c)| {
                    let ln = &line[i..];
                    let res = if ln.starts_with("one")  { '1' }
                        else if ln.starts_with("two")   { '2' }
                        else if ln.starts_with("three") { '3' }
                        else if ln.starts_with("four")  { '4' }
                        else if ln.starts_with("five")  { '5' }
                        else if ln.starts_with("six")   { '6' }
                        else if ln.starts_with("seven") { '7' }
                        else if ln.starts_with("eight") { '8' }
                        else if ln.starts_with("nine")  { '9' }
                        else { c };
                    res.to_digit(10)
                });
            let first = chars.next().unwrap();
            let last = match chars.last() {
                Some(x) => x,
                None => first
            };
            first * 10 + last
        })
        .reduce(|acc, e| { acc + e })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
