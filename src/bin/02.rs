advent_of_code::solution!(2);

use std::{cmp::Ordering, str::FromStr};

use strum::EnumString;

#[derive(Clone, Copy, PartialEq, Eq, EnumString)]
enum Choice {
    #[strum(serialize = "A", serialize = "X")]
    Rock = 1,
    #[strum(serialize = "B", serialize = "Y")]
    Paper = 2,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors = 3,
}

struct Battle {
    opponent: Choice,
    me: Choice,
}

impl Battle {
    fn fix_v2(mut self) -> Self {
        self.me = match (self.opponent, self.me) {
            (Choice::Rock, Choice::Rock)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Paper) => Choice::Scissors,
            (Choice::Rock, Choice::Paper)
            | (Choice::Scissors, Choice::Scissors)
            | (Choice::Paper, Choice::Rock) => Choice::Rock,
            (Choice::Rock, Choice::Scissors)
            | (Choice::Paper, Choice::Paper)
            | (Choice::Scissors, Choice::Rock) => Choice::Paper,
        };
        self
    }

    fn score(&self) -> u64 {
        self.win_score() + self.me as u64
    }

    fn win(&self) -> Ordering {
        use Ordering::*;
        match self.opponent as i64 - self.me as i64 {
            0 => Equal,
            1 | -2 => Less,
            2 | -1 => Greater,
            _ => unreachable!(),
        }
    }

    fn win_score(&self) -> u64 {
        match self.win() {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
    }
}

impl FromStr for Battle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent_str, me_str) = s.split_once(' ').unwrap();
        let opponent = opponent_str.parse()?;
        let me = me_str.parse()?;
        Ok(Self { opponent, me })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<Battle>().unwrap().score())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| l.parse::<Battle>().unwrap().fix_v2().score())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
