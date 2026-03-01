use std::{collections::VecDeque, str::FromStr};

advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Procedure {
    count: usize,
    src: usize,
    dest: usize,
}

impl FromStr for Procedure {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let count = words.nth(1).unwrap().parse()?;
        let src = words.nth(1).unwrap().parse()?;
        let dest = words.nth(1).unwrap().parse()?;
        Ok(Self { count, src, dest })
    }
}

#[derive(Debug)]
struct Crates {
    bins: Vec<VecDeque<char>>,
    len: usize,
}

impl FromStr for Crates {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();

        let len = (lines.peek().unwrap().len() + 1) / 4;
        let mut c = vec![VecDeque::with_capacity(40); len + 1];
        for line in lines {
            for (index, ct) in c.iter_mut().enumerate() {
                if let Some(c) = line
                    .get(1 + index * 4..2 + index * 4)
                    .and_then(|c| c.chars().next())
                    && c.is_ascii_uppercase()
                {
                    ct.push_front(c);
                }
            }
        }

        Ok(Crates { bins: c, len })
    }
}

impl Crates {
    fn apply(&mut self, procedure: Procedure) {
        for _ in 0..procedure.count {
            let c = self.bins[procedure.src - 1].pop_back().unwrap();
            self.bins[procedure.dest - 1].push_back(c);
        }
    }

    fn apply_v2(&mut self, procedure: Procedure) {
        for _ in 0..procedure.count {
            let c = self.bins[procedure.src - 1].pop_back().unwrap();
            self.bins[self.len].push_back(c);
        }
        for _ in 0..procedure.count {
            let c = self.bins[self.len].pop_back().unwrap();
            self.bins[procedure.dest - 1].push_back(c);
        }
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for line in &self.bins[..self.len] {
            out.push(*line.back().unwrap());
        }
        out
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (crates, inst) = input.split_once("\n\n").unwrap();
    let mut crates = crates.parse::<Crates>().ok()?;
    for procedure in inst.lines().map(|l| l.parse::<Procedure>().unwrap()) {
        crates.apply(procedure);
    }
    Some(crates.string())
}

pub fn part_two(input: &str) -> Option<String> {
    let (crates, inst) = input.split_once("\n\n").unwrap();
    let mut crates = crates.parse::<Crates>().ok()?;
    for procedure in inst.lines().map(|l| l.parse::<Procedure>().unwrap()) {
        crates.apply_v2(procedure);
    }
    Some(crates.string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("CMZ".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("MCD".into()));
    }
}
