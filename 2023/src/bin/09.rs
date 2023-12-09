advent_of_code::solution!(9);

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

// m * a / b = (m/gcd(m, b) * a) / (b / gcd(m, b))
fn multiply(m: i64, a: i64, b: i64) -> i64 {
    if m == 0 || b == 0 {
        return 0;
    }
    let g = gcd(m, b);
    (m / g * a) / (b / g)
}

fn binomial(n: i64, k: i64) -> i64 {
    if n == 0 || k == 0 {
        return 1;
    }

    // coefficients are simmetric
    if k > n - k {
        return binomial(n, n - k);
    }

    let mut res = 1;
    let mut i = 1;
    let mut n = n;

    loop {
        if i > k {
            break;
        }
        res = multiply(res, n, i);
        i += 1;
        n -= 1;
    }
    res
}

fn next_number(numbers: &[i64]) -> i64 {
    let n = numbers.len() as i64;
    let mut mul = -1;
    numbers
        .iter()
        .rev()
        .enumerate()
        .map(|(num, &el)| {
            mul *= -1;
            mul * binomial(n, (num + 1) as i64) * el
        })
        .sum::<i64>()
}

pub fn part_one(input: &str) -> Option<i64> {
    let res = input
        .lines()
        .map(|line| {
            let numbers = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            next_number(&numbers)
        })
        .sum::<i64>();
    Some(res)
}

pub fn part_two(input: &str) -> Option<i64> {
    let res = input
        .lines()
        .map(|line| {
            let numbers = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .rev()
                .collect::<Vec<_>>();
            next_number(&numbers)
        })
        .sum::<i64>();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(10, 5), 252);
        assert_eq!(binomial(7, 3), 35);
        assert_eq!(binomial(7, 4), 35);
        assert_eq!(binomial(2, 1), 2);
        assert_eq!(binomial(0, 0), 1);
        assert_eq!(binomial(3, 0), 1);
    }

    #[test]
    fn test_next_number() {
        assert_eq!(next_number(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(next_number(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(next_number(&[10, 13, 16, 21, 30, 45]), 68);
        assert_eq!(next_number(&[45, 30, 21, 16, 13, 10]), 5);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
