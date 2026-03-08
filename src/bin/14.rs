use std::{collections::HashMap, fmt::Display, str::FromStr};

use glam::{IVec2, UVec2, ivec2, uvec2};

advent_of_code::solution!(14);

#[derive(Debug, Clone)]
struct Cave {
    map: HashMap<UVec2, Space>,
    extent: (UVec2, UVec2),
}

const BELOW: IVec2 = ivec2(0, 1);
const BELOW_LEFT: IVec2 = ivec2(-1, 1);
const BELOW_RIGHT: IVec2 = ivec2(1, 1);

impl Cave {
    // Returns true if the sand falls off the edge
    fn drop(&mut self) -> bool {
        let mut sand = uvec2(500, 0);
        while let Some(next) = self.next(sand) {
            if next.y > self.extent.1.y {
                return true;
            }
            sand = next;
        }
        self.map.insert(sand, Space::Sand);
        false
    }

    fn drop_two(&mut self) -> bool {
        let mut sand = uvec2(500, 0);
        while let Some(next) = self.next_two(sand) {
            sand = next;
        }
        if sand.y == 0 {
            return true;
        }
        self.map.insert(sand, Space::Sand);
        false
    }

    fn next(&self, sand: UVec2) -> Option<UVec2> {
        if !self.map.contains_key(&sand.wrapping_add_signed(BELOW)) {
            Some(sand.wrapping_add_signed(BELOW))
        } else if !self.map.contains_key(&sand.wrapping_add_signed(BELOW_LEFT)) {
            Some(sand.wrapping_add_signed(BELOW_LEFT))
        } else if !self
            .map
            .contains_key(&sand.wrapping_add_signed(BELOW_RIGHT))
        {
            Some(sand.wrapping_add_signed(BELOW_RIGHT))
        } else {
            None
        }
    }

    fn next_two(&self, sand: UVec2) -> Option<UVec2> {
        if sand.y == self.extent.1.y + 1 {
            None
        } else if !self.map.contains_key(&sand.wrapping_add_signed(BELOW)) {
            Some(sand.wrapping_add_signed(BELOW))
        } else if !self.map.contains_key(&sand.wrapping_add_signed(BELOW_LEFT)) {
            Some(sand.wrapping_add_signed(BELOW_LEFT))
        } else if !self
            .map
            .contains_key(&sand.wrapping_add_signed(BELOW_RIGHT))
        {
            Some(sand.wrapping_add_signed(BELOW_RIGHT))
        } else {
            None
        }
    }
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<UVec2, Space> = HashMap::new();
        let mut extent = (uvec2(500, 0), uvec2(500, 0));
        for line in s.lines() {
            let mut last_pt = None;
            for pt in line.split(" -> ").map(|s| {
                s.split_once(',')
                    .map(|(x, y)| uvec2(x.parse().unwrap(), y.parse().unwrap()))
                    .unwrap()
            }) {
                extent.0.x = pt.x.min(extent.0.x);
                extent.0.y = pt.y.min(extent.0.y);
                extent.1.x = pt.x.max(extent.1.x);
                extent.1.y = pt.y.max(extent.1.y);
                if let Some(last_pt) = last_pt {
                    let inc = inc(&last_pt, &pt);
                    let mut new_pt = last_pt.wrapping_add_signed(inc);
                    while new_pt != pt {
                        map.insert(new_pt, Space::Rock);
                        new_pt = new_pt.wrapping_add_signed(inc);
                    }
                }
                map.insert(pt, Space::Rock);
                last_pt = Some(pt);
            }
        }
        Ok(Self { map, extent })
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.extent.0.y..=self.extent.1.y {
            for x in self.extent.0.x..=self.extent.1.x {
                match self.map.get(&uvec2(x, y)) {
                    Some(Space::Rock) => write!(f, "#")?,
                    Some(Space::Sand) => write!(f, "o")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Rock,
    Sand,
}

fn inc(from: &UVec2, to: &UVec2) -> IVec2 {
    (to.as_ivec2() - from.as_ivec2()).map(i32::signum)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut cave = input.parse::<Cave>().unwrap();
    // println!("{cave}");
    #[allow(clippy::manual_find)]
    for i in 0.. {
        if cave.drop() {
            return Some(i);
        }
        // println!("{cave}");
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cave = input.parse::<Cave>().unwrap();
    // println!("{cave}");
    #[allow(clippy::manual_find)]
    for i in 0.. {
        if cave.drop_two() {
            return Some(i + 1);
        }
        // println!("{cave}");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(93));
    }
}
