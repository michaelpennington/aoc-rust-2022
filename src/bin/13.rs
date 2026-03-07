use anyhow::anyhow;

use std::{cmp::Ordering, str::FromStr};

advent_of_code::solution!(13);

#[derive(Debug, Clone)]
enum NestedValue {
    Item(u32),
    List(Vec<NestedValue>),
}

#[derive(Debug, Clone)]
struct Packet(Vec<NestedValue>);

fn ps(s: &str) -> Vec<NestedValue> {
    let mut out = Vec::new();
    let mut indices = Vec::new();
    let mut level = 0;
    let mut start = 0;
    for (i, c) in s.char_indices() {
        match c {
            ',' if level == 0 => {
                indices.push((start, i));
                start = i + 1;
            }
            '[' => {
                level += 1;
            }
            ']' => {
                level -= 1;
            }
            _ => {}
        }
    }
    if s.is_empty() {
        return Vec::new();
    }
    indices.push((start, s.len()));
    for (start, end) in indices {
        if let Some(inner_list) = s[start..end]
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
        {
            out.push(NestedValue::List(ps(inner_list)));
        } else {
            out.push(NestedValue::Item(s[start..end].parse().unwrap()));
        }
    }
    out
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match valid(&self.0, &other.0) {
            Res::Valid => false,
            Res::Invalid => false,
            Res::Continue => true,
        }
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match valid(&self.0, &other.0) {
            Res::Valid => Ordering::Less,
            Res::Invalid => Ordering::Greater,
            Res::Continue => Ordering::Equal,
        }
    }
}

impl FromStr for Packet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .ok_or(anyhow!("Each packet is always a list"))?;

        Ok(Self(ps(s)))
    }
}

enum Res {
    Valid,
    Invalid,
    Continue,
}

fn valid(left: &[NestedValue], right: &[NestedValue]) -> Res {
    let mut left = left.iter();
    let mut right = right.iter();
    loop {
        match (left.next(), right.next()) {
            (None, None) => return Res::Continue,
            (None, Some(_)) => return Res::Valid,
            (Some(_), None) => return Res::Invalid,
            (Some(nv1), Some(nv2)) => match valid_inner(nv1, nv2) {
                Res::Valid => return Res::Valid,
                Res::Invalid => return Res::Invalid,
                Res::Continue => {}
            },
        }
    }
}

fn valid_inner(left: &NestedValue, right: &NestedValue) -> Res {
    match (left, right) {
        (NestedValue::Item(i1), NestedValue::Item(i2)) => match i1.cmp(i2) {
            Ordering::Less => Res::Valid,
            Ordering::Equal => Res::Continue,
            Ordering::Greater => Res::Invalid,
        },
        (NestedValue::Item(i1), NestedValue::List(nlist)) => {
            valid(&[NestedValue::Item(*i1)], nlist)
        }
        (NestedValue::List(nv), NestedValue::Item(i)) => valid(nv, &[NestedValue::Item(*i)]),
        (NestedValue::List(nv1), NestedValue::List(nv2)) => valid(nv1, nv2),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split("\n\n")
            .enumerate()
            .filter_map(|(index, pair)| {
                let (left, right) = pair.split_once('\n').unwrap();
                let (left, right): (Packet, Packet) =
                    (left.trim().parse().unwrap(), right.trim().parse().unwrap());
                match valid(&left.0, &right.0) {
                    Res::Valid => Some(index + 1),
                    Res::Invalid => None,
                    Res::Continue => unreachable!(),
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<Packet>().unwrap())
        .collect::<Vec<_>>();
    let dp1: Packet = "[[2]]".parse().unwrap();
    let dp2: Packet = "[[6]]".parse().unwrap();
    packets.push(dp1.clone());
    packets.push(dp2.clone());
    packets.sort();
    let dp1i = packets.binary_search(&dp1).unwrap();
    let dp2i = packets.binary_search(&dp2).unwrap();

    Some((dp1i + 1) * (dp2i + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }
}
