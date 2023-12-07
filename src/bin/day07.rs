use aoc2023::{read_input, InputType};
use counter::Counter;
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 7;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

#[derive(Debug, PartialEq, Hash, Eq)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
}

impl Card {
    fn get_strength(&self) -> u8 {
        match self {
            Card::A => 12,
            Card::K => 11,
            Card::Q => 10,
            Card::J => 9,
            Card::T => 8,
            Card::N9 => 7,
            Card::N8 => 6,
            Card::N7 => 5,
            Card::N6 => 4,
            Card::N5 => 3,
            Card::N4 => 2,
            Card::N3 => 1,
            Card::N2 => 0,
        }
    }
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::N9),
            '8' => Ok(Card::N8),
            '7' => Ok(Card::N7),
            '6' => Ok(Card::N6),
            '5' => Ok(Card::N5),
            '4' => Ok(Card::N4),
            '3' => Ok(Card::N3),
            '2' => Ok(Card::N2),
            _ => Err(()),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.get_strength().cmp(&other.get_strength()))
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn get_strength(&self) -> u8 {
        match self {
            HandType::FiveOfKind => 6,
            HandType::FourOfKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

impl TryFrom<&Vec<Card>> for HandType {
    type Error = ();

    fn try_from(cards: &Vec<Card>) -> Result<Self, Self::Error> {
        let counter = cards.iter().collect::<Counter<_>>();
        let values = counter.values().sorted().collect_vec();

        match values.as_slice() {
            &[5] => Ok(HandType::FiveOfKind),
            &[1, 4] => Ok(HandType::FourOfKind),
            &[1, 1, 3] => Ok(HandType::ThreeOfKind),
            &[2, 3] => Ok(HandType::FullHouse),
            &[1, 2, 2] => Ok(HandType::TwoPair),
            &[1, 1, 1, 2] => Ok(HandType::OnePair),
            &[1, 1, 1, 1, 1] => Ok(HandType::HighCard),
            _ => Err(()),
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.get_strength().cmp(&other.get_strength()))
    }
}

impl TryFrom<&str> for Hand {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.len() != 5 {
            return Err(());
        }

        let cards = value
            .chars()
            .map(|char| char.try_into().unwrap())
            .collect_vec();

        let hand_type = (&cards).try_into().unwrap();

        Ok(Hand { cards, hand_type })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(ordering) => match ordering {
                std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
                std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
                std::cmp::Ordering::Equal => {
                    for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
                        match card1.partial_cmp(card2) {
                            Some(card_ordering) => match card_ordering {
                                std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
                                std::cmp::Ordering::Equal => (),
                                std::cmp::Ordering::Greater => {
                                    return Some(std::cmp::Ordering::Greater)
                                }
                            },
                            None => unreachable!(),
                        }
                    }
                    return Some(std::cmp::Ordering::Equal);
                }
            },
            None => unreachable!(),
        }
    }
}

#[timed]
fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let (hand_part, bid_part) = line.trim().split(" ").collect_tuple().unwrap();
            let hand = Hand::try_from(hand_part).unwrap();
            let bid = bid_part.parse::<u64>().unwrap();
            (hand, bid)
        })
        .sorted_by(|(hand1, _), (hand2, _)| hand1.partial_cmp(hand2).unwrap())
        .enumerate()
        .map(|(index, (hand, bid))| {
            let rank = (index + 1) as u64;
            (hand, rank, bid)
        })
        .map(|(_, rank, bid)| rank * bid)
        .sum()
}

#[timed]
fn part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 6440;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_wj() {
        let expected = 251927063;
        let result = part1(&read_input(DAY, InputType::Other("WJ".to_string())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 5905;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_wj() {
        let expected = 255632664;
        let result = part2(&read_input(DAY, InputType::Other("WJ".to_string())).unwrap());
        assert_eq!(result, expected);
    }
}
