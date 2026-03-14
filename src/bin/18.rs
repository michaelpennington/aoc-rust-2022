advent_of_code::solution!(18);

use std::collections::HashSet;

use glam::{IVec3, UVec3, uvec3};

const OFFSETS: [IVec3; 6] = [
    IVec3::X,
    IVec3::Y,
    IVec3::Z,
    IVec3::NEG_X,
    IVec3::NEG_Y,
    IVec3::NEG_Z,
];

fn neighbors(v: &UVec3) -> impl Iterator<Item = UVec3> {
    OFFSETS.into_iter().filter_map(|o| v.checked_add_signed(o))
}

fn neighbors_clamped(v: &UVec3, max: u32) -> impl Iterator<Item = UVec3> {
    OFFSETS
        .into_iter()
        .filter_map(|o| v.checked_add_signed(o))
        .filter(move |o| o.x <= max && o.y <= max && o.z <= max)
}

pub fn part_one(input: &str) -> Option<u64> {
    let pts: HashSet<UVec3> = input
        .lines()
        .map(|l| {
            let mut pts = l.split(',');
            let (x, y, z) = (
                pts.next().unwrap().parse().unwrap(),
                pts.next().unwrap().parse().unwrap(),
                pts.next().unwrap().parse().unwrap(),
            );
            uvec3(x, y, z)
        })
        .collect();
    Some(
        pts.iter()
            .map(|p| 6 - neighbors(p).filter(|n| pts.contains(n)).count() as u64)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut points = HashSet::new();
    let mut max = 0;
    for line in input.lines() {
        let mut pts = line.split(',');
        let (x, y, z) = (
            pts.next().unwrap().parse::<u32>().unwrap() + 1,
            pts.next().unwrap().parse::<u32>().unwrap() + 1,
            pts.next().unwrap().parse::<u32>().unwrap() + 1,
        );
        max = max.max(x).max(y).max(z);
        points.insert(uvec3(x, y, z));
    }
    max += 1;
    let mut visited = HashSet::new();
    let mut stack = vec![uvec3(0, 0, 0)];
    while let Some(p) = stack.pop() {
        visited.insert(p);
        for neighbor in
            neighbors_clamped(&p, max).filter(|n| !points.contains(n) && !visited.contains(n))
        {
            stack.push(neighbor);
        }
    }
    Some(
        points
            .iter()
            .map(|p| neighbors(p).filter(|n| visited.contains(n)).count() as u64)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58));
    }
}
