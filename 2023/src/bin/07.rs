advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char) -> Option<Card> {
        match c {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::J),
            'Q' => Some(Card::Q),
            'K' => Some(Card::K),
            'A' => Some(Card::A),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Hand {
    High(Card, Card, Card, Card, Card),
    Pair(Card, Card, Card, Card, Card),
    TwoPairs(Card, Card, Card, Card, Card),
    Three(Card, Card, Card, Card, Card),
    FullHouse(Card, Card, Card, Card, Card),
    Four(Card, Card, Card, Card, Card),
    Five(Card, Card, Card, Card, Card),
}

fn get_hand(cards: &str) -> Hand {
    let mut deck = cards
        .chars()
        .map(Card::from_char)
        .map(|c| c.unwrap())
        .take(5)
        .collect::<Vec<_>>();

    let (a0, b0, c0, d0, e0) = (deck[0], deck[1], deck[2], deck[3], deck[4]);
    deck.sort();
    let (a, b, c, d, e) = (deck[0], deck[1], deck[2], deck[3], deck[4]);

    if a == b && b == c && c == d && d == e {
        Hand::Five(a0, b0, c0, d0, e0)
    } else if (a == b && b == c && c == d) || (b == c && c == d && d == e) {
        Hand::Four(a0, b0, c0, d0, e0)
    } else if (a == b && b == c && d == e) || (a == b && c == d && d == e) {
        Hand::FullHouse(a0, b0, c0, d0, e0)
    } else if (a == b && b == c) || (b == c && c == d) || (c == d && d == e) {
        Hand::Three(a0, b0, c0, d0, e0)
    } else if (a == b && c == d) || (a == b && d == e) || (b == c && d == e) {
        Hand::TwoPairs(a0, b0, c0, d0, e0)
    } else if a == b || b == c || c == d || d == e {
        Hand::Pair(a0, b0, c0, d0, e0)
    } else {
        Hand::High(a0, b0, c0, d0, e0)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut res = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(d, rank)| (get_hand(d), d, rank.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();
    res.sort_by(|a, b| a.0.cmp(&b.0));
    let res = res.iter().enumerate().fold(0u32, |acc, el| {
        println!("{el:?}");
        acc + (el.0 as u32 + 1) * el.1 .2
    });
    Some(res)
}

impl Card {
    fn from_char_part_two(c: char) -> Option<Card> {
        match c {
            'J' => Some(Card::Joker),
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'Q' => Some(Card::Q),
            'K' => Some(Card::K),
            'A' => Some(Card::A),
            _ => None,
        }
    }
}
fn get_hand_part_two(cards: &str) -> Hand {
    let mut deck = cards
        .chars()
        .map(Card::from_char_part_two)
        .map(|c| c.unwrap())
        .take(5)
        .collect::<Vec<_>>();

    let (a0, b0, c0, d0, e0) = (deck[0], deck[1], deck[2], deck[3], deck[4]);
    deck.sort();
    let (a, b, c, d, e) = (deck[0], deck[1], deck[2], deck[3], deck[4]);

    if a == b && b == c && c == d && d == e {
        Hand::Five(a0, b0, c0, d0, e0)
    } else if (a == b && b == c && c == d) || (b == c && c == d && d == e) {
        if a == Card::Joker {
            Hand::Five(a0, b0, c0, d0, e0)
        } else {
            Hand::Four(a0, b0, c0, d0, e0)
        }
    } else if (a == b && b == c && d == e) || (a == b && c == d && d == e) {
        if a == Card::Joker {
            Hand::Five(a0, b0, c0, d0, e0)
        } else {
            Hand::FullHouse(a0, b0, c0, d0, e0)
        }
    } else if (a == b && b == c) || (b == c && c == d) || (c == d && d == e) {
        if a == Card::Joker {
            Hand::Four(a0, b0, c0, d0, e0)
        } else {
            Hand::Three(a0, b0, c0, d0, e0)
        }
    } else if (a == b && c == d) || (a == b && d == e) {
        if a == Card::Joker {
            Hand::Four(a0, b0, c0, d0, e0)
        } else {
            Hand::TwoPairs(a0, b0, c0, d0, e0)
        }
    } else if b == c && d == e {
        if a == Card::Joker {
            Hand::FullHouse(a0, b0, c0, d0, e0)
        } else {
            Hand::TwoPairs(a0, b0, c0, d0, e0)
        }
    } else if a == b || b == c || c == d || d == e {
        if a == Card::Joker {
            Hand::Three(a0, b0, c0, d0, e0)
        } else {
            Hand::Pair(a0, b0, c0, d0, e0)
        }
    } else if a == Card::Joker {
        Hand::Pair(a0, b0, c0, d0, e0)
    } else {
        Hand::High(a0, b0, c0, d0, e0)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(d, rank)| (get_hand_part_two(d), d, rank.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();
    res.sort_by(|a, b| a.0.cmp(&b.0));
    let res = res.iter().enumerate().fold(0u32, |acc, el| {
        println!("{el:?}");
        acc + (el.0 as u32 + 1) * el.1 .2
    });
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card() {
        let input = "32T3K 765
T55J5 684
KKJJT 220
KK677 28
QQQJA 483";
        let res = part_one(input);
        println!("{:?}", res);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
