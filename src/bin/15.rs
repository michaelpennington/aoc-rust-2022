use std::str::FromStr;

use anyhow::Context;
use glam::{IVec2, ivec2};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Range<T> {
    pub start: T,
    pub end: T,
}

advent_of_code::solution!(15);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Sensor {
    loc: IVec2,
    nearest: IVec2,
}

impl Sensor {
    fn forbidden_range(&self, y: i32) -> Range<i32> {
        let a = self.loc.x;
        let b = self.loc.y.abs_diff(y) as i32;
        let c = self.loc.manhattan_distance(self.nearest) as i32;

        let start = a + b - c;
        let end = start.max(a - b + c);
        Range { start, end }
    }
}

impl FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor, beacon) = s
            .strip_prefix("Sensor at ")
            .context("Expected `Sensor at `")?
            .split_once(": closest beacon is at ")
            .context("Expected `: closest beacon is at `")?;
        let (sx, sy) = sensor.split_once(", ").context("Expected `, `")?;
        let (bx, by) = beacon.split_once(", ").context("Expected `, `")?;
        let loc = ivec2(
            sx.strip_prefix("x=").context("Expected `x=`")?.parse()?,
            sy.strip_prefix("y=").context("Expected `y=`")?.parse()?,
        );
        let nearest = ivec2(
            bx.strip_prefix("x=").context("Expected `x=`")?.parse()?,
            by.strip_prefix("y=").context("Expected `y=`")?.parse()?,
        );
        Ok(Self { loc, nearest })
    }
}

#[derive(Clone, Debug)]
struct Sensors {
    sensors: Vec<Sensor>,
}

impl FromStr for Sensors {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sensors: s.lines().map(|l| l.parse().unwrap()).collect(),
        })
    }
}

impl Sensors {
    fn merged_intervals(&self, y: i32) -> Vec<Range<i32>> {
        let mut intervals = self
            .sensors
            .iter()
            .map(|s| s.forbidden_range(y))
            .collect::<Vec<_>>();
        intervals.sort_by_key(|i| i.start);
        let mut out = Vec::with_capacity(intervals.len());
        let mut intervals_iter = intervals.into_iter();
        out.push(intervals_iter.next().unwrap());
        for curr in intervals_iter {
            let last = out.last_mut().unwrap();
            let last_end = last.end;
            if curr.start <= last_end {
                last.end = last_end.max(curr.end);
            } else {
                out.push(curr);
            }
        }
        out
    }

    fn num_disallowed(&self, y: i32) -> u64 {
        self.merged_intervals(y)
            .iter()
            .map(|i| (i.end - i.start) as u64)
            .sum()
    }

    fn find(&self, range: i32) -> u64 {
        for y in 0..range {
            for int in self.merged_intervals(y) {
                if 0 <= int.end && int.end < range {
                    return 4000000 * (int.end as u64 + 1) + y as u64;
                }
            }
        }
        0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_inner(input, 2_000_000)
}

fn part_one_inner(input: &str, n: i32) -> Option<u64> {
    let sensors = input.parse::<Sensors>().unwrap();
    Some(sensors.num_disallowed(n))
}

pub fn part_two(input: &str) -> Option<u64> {
    part_two_inner(input, 4_000_000)
}

fn part_two_inner(input: &str, n: i32) -> Option<u64> {
    let sensors = input.parse::<Sensors>().unwrap();
    Some(sensors.find(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_inner(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_inner(&advent_of_code::template::read_file("examples", DAY), 20);
        assert_eq!(result, Some(56000011));
    }
}
