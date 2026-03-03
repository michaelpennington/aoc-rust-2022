use std::{collections::HashSet, str::FromStr};

advent_of_code::solution!(8);

#[derive(Debug)]
struct Map {
    inner: Vec<Vec<i8>>,
    size: usize,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let size = lines.peek().unwrap().len();
        Ok(Self {
            inner: lines
                .map(|l| l.chars().map(|c| c as i8 - b'0' as i8).collect())
                .collect(),
            size,
        })
    }
}

impl Map {
    fn count_trees(&self) -> u64 {
        let mut pts = HashSet::with_capacity(self.size * self.size / 4);
        for (y, line) in self.inner.iter().enumerate() {
            let mut tallest = i8::MIN;
            for (x, &t) in line.iter().enumerate() {
                if t > tallest {
                    tallest = t;
                    pts.insert((x, y));
                }
            }
        }
        for (y, line) in self.inner.iter().enumerate() {
            let mut tallest = i8::MIN;
            for (x, &t) in line.iter().enumerate().rev() {
                if t > tallest {
                    tallest = t;
                    pts.insert((x, y));
                }
            }
        }
        for x in 0..self.size {
            let mut tallest = i8::MIN;
            for y in 0..self.size {
                let t = self.inner[y][x];
                if t > tallest {
                    tallest = t;
                    pts.insert((x, y));
                }
            }
        }
        for x in 0..self.size {
            let mut tallest = i8::MIN;
            for y in (0..self.size).rev() {
                let t = self.inner[y][x];
                if t > tallest {
                    tallest = t;
                    pts.insert((x, y));
                }
            }
        }
        pts.len() as u64
    }

    fn best_score(&self) -> u64 {
        let mut best_score = 0;
        for (y, line) in self.inner.iter().enumerate() {
            for (x, &t) in line.iter().enumerate() {
                let score = self.calc_score(t, (x, y));
                if score >= best_score {
                    best_score = score;
                }
            }
        }
        best_score
    }

    fn calc_score(&self, ht: i8, pt: (usize, usize)) -> u64 {
        ({
            let mut sum = 0;
            for i in 1..(self.size - pt.0) {
                sum += 1;
                if self.inner[pt.1][pt.0 + i] >= ht {
                    break;
                }
            }
            sum
        }) * {
            let mut sum = 0;
            for i in 1..=pt.0 {
                sum += 1;
                if self.inner[pt.1][pt.0 - i] >= ht {
                    break;
                }
            }

            sum
        } * {
            let mut sum = 0;
            for i in 1..(self.size - pt.1) {
                sum += 1;
                if self.inner[pt.1 + i][pt.0] >= ht {
                    break;
                }
            }
            sum
        } * {
            let mut sum = 0;
            for i in 1..=pt.1 {
                sum += 1;
                if self.inner[pt.1 - i][pt.0] >= ht {
                    break;
                }
            }
            sum
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input.parse::<Map>().unwrap();
    Some(map.count_trees())
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input.parse::<Map>().unwrap();
    Some(map.best_score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
