use std::{cell::RefCell, hash::Hash, str::FromStr};
advent_of_code::solution!(11);

const MODULO: u64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Operation {
    Square,
    Mul(u64),
    Add(u64),
}

impl Operation {
    fn apply(&self, input: u64) -> u64 {
        match self {
            Operation::Square => input * input,
            Operation::Mul(c) => input * c,
            Operation::Add(c) => input + c,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    test: u64,
    yes: usize,
    no: usize,
    items_inspected: u64,
}

impl Monkey {
    fn throw<const P: bool>(&mut self) -> impl Iterator<Item = (usize, u64)> + use<'_, P> {
        self.items
            .drain(..)
            .inspect(|_| self.items_inspected += 1)
            .map(|worry_level| self.op.apply(worry_level) / if P { 1 } else { 3 })
            .map(|worry_level| {
                if worry_level % self.test == 0 {
                    (self.yes, worry_level % MODULO)
                } else {
                    (self.no, worry_level % MODULO)
                }
            })
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1);
        let items = lines
            .next()
            .unwrap()
            .split([',', ':'])
            .skip(1)
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let next_line = lines.next().unwrap();
        let op = if next_line.contains("*") {
            if next_line.ends_with("old") {
                Operation::Square
            } else {
                Operation::Mul(next_line.rsplit(' ').next().unwrap().parse()?)
            }
        } else {
            Operation::Add(next_line.rsplit(' ').next().unwrap().parse()?)
        };
        let test = lines.next().unwrap().rsplit(' ').next().unwrap().parse()?;
        let yes = lines.next().unwrap().rsplit(' ').next().unwrap().parse()?;
        let no = lines.next().unwrap().rsplit(' ').next().unwrap().parse()?;
        Ok(Self {
            items,
            op,
            test,
            yes,
            no,
            items_inspected: 0,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Monkeys {
    monkeys: Vec<RefCell<Monkey>>,
    count: usize,
}

impl FromStr for Monkeys {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkeys = Vec::new();
        for m in s.split("\n\n") {
            monkeys.push(RefCell::new(m.parse()?));
        }
        let count = monkeys.len();
        Ok(Self { monkeys, count })
    }
}

impl Monkeys {
    fn run_round<const P: bool>(&mut self) {
        for index in 0..self.count {
            let mut monkey = self.monkeys[index].borrow_mut();
            for (dest, worry_level) in monkey.throw::<P>() {
                self.monkeys[dest].borrow_mut().items.push(worry_level);
            }
        }
    }

    fn monkey_business(&self) -> u64 {
        let mut scores: Vec<_> = self
            .monkeys
            .iter()
            .map(|m| m.borrow().items_inspected)
            .collect();
        scores.sort();
        scores[self.count - 1] * scores[self.count - 2]
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut monkeys = input.parse::<Monkeys>().unwrap();
    for _ in 0..20 {
        monkeys.run_round::<false>();
    }
    Some(monkeys.monkey_business())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys = input.parse::<Monkeys>().unwrap();
    for _ in 0..10000 {
        monkeys.run_round::<true>();
    }
    Some(monkeys.monkey_business())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2713310158));
    }
}
