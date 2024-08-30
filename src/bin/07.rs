use itertools::Itertools;
use std::fmt::Debug;
use std::{cmp::Ordering, collections::HashMap, convert::Infallible, str::FromStr};

advent_of_code::solution!(7);

#[derive(Debug, PartialOrd, PartialEq, Eq)]
enum PokerHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl PokerHandType {
    fn upgrade(&self) -> Self {
        match self {
            PokerHandType::HighCard => PokerHandType::OnePair,
            PokerHandType::OnePair => PokerHandType::ThreeOfAKind,
            PokerHandType::TwoPair => PokerHandType::FullHouse,
            PokerHandType::ThreeOfAKind => PokerHandType::FourOfAKind,
            PokerHandType::FullHouse => PokerHandType::FullHouse,
            PokerHandType::FourOfAKind => PokerHandType::FiveOfAKind,
            PokerHandType::FiveOfAKind => PokerHandType::FiveOfAKind,
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq)]
struct PokerHandRegular {
    hand_type: PokerHandType,
}

impl FromStr for PokerHandRegular {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut char_count: HashMap<char, u32> = HashMap::new();
        for c in s.chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }

        let values: Vec<u32> = char_count.into_values().collect();

        Ok(PokerHandRegular {
            hand_type: match values.as_slice() {
                [5] => PokerHandType::FiveOfAKind,
                [1, 4] | [4, 1] => PokerHandType::FourOfAKind,
                [3, 2] | [2, 3] => PokerHandType::FullHouse,
                [3, 1, 1] | [1, 3, 1] | [1, 1, 3] => PokerHandType::ThreeOfAKind,
                [2, 2, 1] | [2, 1, 2] | [1, 2, 2] => PokerHandType::TwoPair,
                [2, 1, 1, 1] | [1, 2, 1, 1] | [1, 1, 2, 1] | [1, 1, 1, 2] => PokerHandType::OnePair,
                [1, 1, 1, 1, 1] => PokerHandType::HighCard,
                _ => panic!(),
            },
        })
    }
}
#[derive(Debug, PartialOrd, PartialEq, Eq)]

struct PokerHandWithJokers {
    hand_type: PokerHandType,
}
impl FromStr for PokerHandWithJokers {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut char_count: HashMap<char, u32> = HashMap::new();

        let jokers = s.chars().filter(|x| *x == 'J').count();
        for c in s.chars().filter(|x| *x != 'J') {
            *char_count.entry(c).or_insert(0) += 1;
        }

        let values: Vec<u32> = char_count.into_values().collect();
        let mut hand_type: PokerHandType = match values.as_slice() {
            [5] | [] => PokerHandType::FiveOfAKind,
            [1, 4] | [4, 1] | [4] => PokerHandType::FourOfAKind,
            [3, 2] | [2, 3] => PokerHandType::FullHouse,
            [3, ..] | [.., 3, _] | [.., 3] => PokerHandType::ThreeOfAKind,
            [2, 2, 1] | [2, 1, 2] | [1, 2, 2] | [2, 2] => PokerHandType::TwoPair,
            [2, ..] | [.., 2] | [_, 2, ..] | [.., 2, _] => PokerHandType::OnePair,
            [1, 1, 1, 1, 1] | [1, 1, 1, 1] | [1, 1, 1] | [1, 1] | [1] => PokerHandType::HighCard,
            _ => panic!(),
        };
        for _ in 0..jokers {
            hand_type = hand_type.upgrade();
        }
        Ok(PokerHandWithJokers { hand_type })
    }
}
trait HT {
    fn get_ranks() -> Vec<char>;
    fn get_hand_type(&self) -> &PokerHandType;
}
impl HT for PokerHandRegular {
    fn get_hand_type(&self) -> &PokerHandType {
        &self.hand_type
    }

    fn get_ranks() -> Vec<char> {
        vec![
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ]
    }
}
impl HT for PokerHandWithJokers {
    fn get_hand_type(&self) -> &PokerHandType {
        &self.hand_type
    }
    fn get_ranks() -> Vec<char> {
        vec![
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ]
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand<T>
where
    T: HT,
{
    cards: String,
    hand_type: T,
    score: u32,
}

impl<T> PartialOrd for Hand<T>
where
    T: HT + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Hand<T>
where
    T: HT + Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type.get_hand_type() < other.hand_type.get_hand_type() {
            Ordering::Less
        } else if self.hand_type.get_hand_type() > other.hand_type.get_hand_type() {
            Ordering::Greater
        } else {
            for (c1, c2) in self.cards.chars().zip(other.cards.chars()) {
                let p1 = T::get_ranks().iter().position(|x| *x == c1).unwrap();
                let p2 = T::get_ranks().iter().position(|x| *x == c2).unwrap();
                if p1 > p2 {
                    return Ordering::Less;
                }
                if p1 < p2 {
                    return Ordering::Greater;
                }
            }

            panic!()
        }
    }
}

impl<T> FromStr for Hand<T>
where
    T: HT + Eq + FromStr + Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, score) = s.split_once(' ').unwrap();
        let score = score.parse::<u32>().unwrap();
        Ok(Self {
            hand_type: cards.parse::<T>().unwrap(),
            cards: cards.to_string(),
            score,
        })
    }
}

fn solve<T>(input: &str) -> Option<u32>
where
    T: HT + Eq + FromStr + Debug,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|l| l.parse::<Hand<T>>().unwrap())
        .sorted()
        .enumerate()
        .map(|(i, hand)| hand.score * (i as u32 + 1))
        .sum::<u32>()
        .into()
}

pub fn part_one(input: &str) -> Option<u32> {
    solve::<PokerHandRegular>(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve::<PokerHandWithJokers>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

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
