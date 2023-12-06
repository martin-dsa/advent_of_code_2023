use std::{
    collections::{HashMap, HashSet},
    vec,
};

use fancy_regex::Regex;
use lazy_static::lazy_static;

advent_of_code::solution!(3);

lazy_static! {
    static ref RE_NUMBER: Regex = Regex::new(r"(\d+)").unwrap();
    static ref RE_SYMBOL: Regex = Regex::new(r"([^\d\.\n])+").unwrap();
    static ref RE_ASTERISK: Regex = Regex::new(r"(\*)").unwrap();
}

#[derive(Debug)]
struct Position(u32, u32);
impl Position {
    fn is_near(&self, other: &Self) -> bool {
        let x_d = (self.0).abs_diff(other.0).cmp(&1).is_le();
        let y_d = (self.1).abs_diff(other.1).cmp(&1).is_le();
        x_d && y_d
    }
}

struct Number {
    position: Position,
    value: u32,
}

impl Number {
    fn get_positions(&self) -> impl Iterator<Item = Position> + '_ {
        (self.position.1..self.position.1 + self.value.to_string().len() as u32)
            .map(|y| Position(self.position.0, y))
    }

    fn near_symbol(&self, symbol_positions: &Vec<Position>) -> bool {
        let positions = self.get_positions();

        for pos in positions {
            for s_pos in symbol_positions {
                if s_pos.is_near(&pos) {
                    return true;
                }
            }
        }
        false
    }

    fn get_near_asterisks_indices(&self, symbol_positions: &[Position]) -> Vec<u32> {
        let positions = self.get_positions();

        let mut indices: Vec<u32> = vec![];

        for pos in positions {
            for (i, s_pos) in symbol_positions.iter().enumerate() {
                if s_pos.is_near(&pos) {
                    indices.push(i as u32);
                }
            }
        }
        indices
    }
}

fn get_data(input: &str, symbol_regex: &Regex) -> (Vec<Position>, Vec<Number>) {
    let mut symbol_positions: Vec<Position> = vec![];
    let mut numbers: Vec<Number> = vec![];

    for (x, line) in input.lines().enumerate() {
        numbers.append(
            &mut RE_NUMBER
                .captures_iter(line)
                .map(|captures| {
                    let m = captures.unwrap().get(1).unwrap();

                    let val_str = m.as_str();

                    let value = val_str.parse::<u32>().unwrap();
                    let start = m.start();
                    let position = Position(x as u32, start as u32);

                    Number { position, value }
                })
                .collect::<Vec<_>>(),
        );

        symbol_positions.append(
            &mut symbol_regex
                .captures_iter(line)
                .map(|captures| {
                    Position(x as u32, (captures.unwrap().get(1).unwrap().start()) as u32)
                })
                .collect::<Vec<_>>(),
        );
    }

    (symbol_positions, numbers)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (symbol_positions, numbers) = get_data(input, &RE_SYMBOL);

    let result = numbers
        .iter()
        .filter(|n| n.near_symbol(&symbol_positions))
        .map(|n| n.value)
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (symbol_positions, numbers) = get_data(input, &RE_ASTERISK);

    let mut asterisk_to_number: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (n, indices) in numbers
        .iter()
        .map(|qwe| (qwe, qwe.get_near_asterisks_indices(&symbol_positions)))
    {
        for idx in indices {
            asterisk_to_number.entry(idx).or_default().insert(n.value);
        }
    }

    let result = asterisk_to_number
        .iter()
        .filter(|entry| entry.1.len() == 2)
        .map(|entry: (&u32, &HashSet<u32>)| entry.1.iter().product::<u32>())
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
