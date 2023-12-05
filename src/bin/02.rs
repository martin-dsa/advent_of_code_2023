use std::{convert::Infallible, str::FromStr};

advent_of_code::solution!(2);

static BAG: Set = Set([Some(12), Some(13), Some(14)]);

struct Game(Vec<Set>);

impl Game {
    fn min_cubes(&self) -> u32 {
        (0..=2)
            .map(|i| {
                self.0
                    .iter()
                    .map(|set| set.0[i].unwrap_or(0))
                    .max()
                    .unwrap()
            })
            .product()
    }

    fn is_possible(&self) -> bool {
        self.0.iter().all(|set| {
            set.0
                .iter()
                .zip(BAG.0.iter())
                .all(|(set, bag)| set.cmp(bag).is_le())
        })
    }
}
impl FromStr for Game {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_once(':')
                .unwrap()
                .1
                .split(';')
                .map(|chunk| chunk.parse::<Set>().unwrap())
                .collect::<Vec<Set>>(),
        ))
    }
}

struct Set([Option<u32>; 3]);

impl FromStr for Set {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set: Set = Set([None, None, None]);

        for (amount, color) in s.split(',').map(|s| s.trim().split_once(' ').unwrap()) {
            let i = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => panic!(),
            };

            set.0[i] = amount.parse::<u32>().ok()
        }

        Ok(set)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .enumerate()
            .map(|(i, line)| (i + 1, line.parse::<Game>().unwrap()))
            .filter(|(_id, game)| game.is_possible())
            .map(|t| t.0 as u32)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| line.parse::<Game>().unwrap())
            .map(|game: Game| game.min_cubes())
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
