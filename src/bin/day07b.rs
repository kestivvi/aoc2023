use aoc2023::{read_input, InputType};
use counter::Counter;
use itertools::Itertools;
use timed::timed;

const DAY: u8 = 7;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part2: {}", part2(&real_input));
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
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
            Card::A => 13,
            Card::K => 12,
            Card::Q => 11,
            Card::J => 0,
            Card::T => 10,
            Card::N9 => 9,
            Card::N8 => 8,
            Card::N7 => 7,
            Card::N6 => 6,
            Card::N5 => 5,
            Card::N4 => 4,
            Card::N3 => 3,
            Card::N2 => 2,
        }
    }

    fn vec() -> Vec<Card> {
        vec![
            Card::A,
            Card::K,
            Card::Q,
            Card::J,
            Card::T,
            Card::N9,
            Card::N8,
            Card::N7,
            Card::N6,
            Card::N5,
            Card::N4,
            Card::N3,
            Card::N2,
        ]
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

#[derive(Debug, PartialEq, Ord, Eq)]
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
        let best_hand = Card::vec()
            .iter()
            .map(|new_joker| {
                let new_cards = replace_joker(cards, new_joker);
                get_hand_type(&new_cards)
            })
            .min()
            .ok_or(())?;

        Ok(best_hand)
    }
}

fn replace_joker(cards: &Vec<Card>, new_joker: &Card) -> Vec<Card> {
    cards
        .iter()
        .map(|card| if let Card::J = card { new_joker } else { card })
        .cloned()
        .collect()
}

fn get_hand_type(cards: &Vec<Card>) -> HandType {
    let counter = cards.iter().collect::<Counter<_>>();
    let values = counter.values().sorted().collect::<Vec<_>>();

    match values.as_slice() {
        &[5] => HandType::FiveOfKind,
        &[1, 4] => HandType::FourOfKind,
        &[1, 1, 3] => HandType::ThreeOfKind,
        &[2, 3] => HandType::FullHouse,
        &[1, 2, 2] => HandType::TwoPair,
        &[1, 1, 1, 2] => HandType::OnePair,
        &[1, 1, 1, 1, 1] => HandType::HighCard,
        _ => HandType::HighCard,
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
        let type_ordering = self.hand_type.partial_cmp(&other.hand_type)?;

        if type_ordering != std::cmp::Ordering::Equal {
            return Some(type_ordering);
        }

        for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
            let card_ordering = card1.partial_cmp(card2)?;

            if card_ordering != std::cmp::Ordering::Equal {
                return Some(card_ordering);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

#[timed]
fn part2(input: &str) -> u64 {
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
            // println!("{:?} {}", hand, bid);
            (hand, rank, bid)
        })
        .map(|(_, rank, bid)| rank * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part2_test() {
        let expected = 5905;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test2() {
        let expected = 6839;
        let result = part2(&read_input(DAY, InputType::Other("test2".to_string())).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_wj() {
        let expected = 255632664;
        let result = part2(&read_input(DAY, InputType::Other("WJ".to_string())).unwrap());
        assert_eq!(result, expected);
    }
}
