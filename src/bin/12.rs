use std::{
    collections::{BinaryHeap, HashMap},
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::bail;

use glam::USizeVec2;

advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<u8>>,
    len: usize,
    height: usize,
    start: USizeVec2,
    end: USizeVec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    fscore: usize,
    pt: USizeVec2,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fscore.cmp(&other.fscore).reverse()
    }
}

impl Map {
    fn neighbors(&self, pt: USizeVec2) -> impl Iterator<Item = USizeVec2> {
        (pt.x.saturating_sub(1)..self.len.min(pt.x + 2))
            .flat_map(move |x| {
                (pt.y.saturating_sub(1)..self.height.min(pt.y + 2)).map(move |y| USizeVec2 { x, y })
            })
            .filter(move |&n| pt.manhattan_distance(n) == 1)
            .filter(move |&n| self[n] <= self[pt] + 1)
    }

    fn h(&self, pt1: USizeVec2, pt2: USizeVec2) -> usize {
        pt1.manhattan_distance(pt2)
    }

    fn part_two_starting_neighbors(&self) -> impl Iterator<Item = USizeVec2> {
        (0..self.height)
            .flat_map(move |y| (0..self.len).map(move |x| USizeVec2 { x, y }))
            .filter(|&v| self[v] == 0)
    }

    fn a_star(&self) -> usize {
        let mut open_set = BinaryHeap::new();
        open_set.push(Node {
            pt: self.start,
            fscore: self.h(self.start, self.end),
        });
        let mut g_score = HashMap::new();
        g_score.insert(self.start, 0);
        while let Some(current) = open_set.pop() {
            if current.pt == self.end {
                return current.fscore;
            }
            for neighbor in self.neighbors(current.pt) {
                let tentative_g_score = g_score[&current.pt] + 1;
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                    g_score.insert(neighbor, tentative_g_score);
                    let fscore = tentative_g_score + self.h(neighbor, self.end);
                    open_set.push(Node {
                        pt: neighbor,
                        fscore,
                    });
                }
            }
        }
        usize::MAX
    }

    fn a_star_two(&self) -> usize {
        let mut open_set = BinaryHeap::new();
        let mut g_score = HashMap::new();
        for pt in self.part_two_starting_neighbors() {
            open_set.push(Node {
                pt,
                fscore: self.h(pt, self.end),
            });
            g_score.insert(pt, 0);
        }
        while let Some(current) = open_set.pop() {
            if current.pt == self.end {
                return current.fscore;
            }
            for neighbor in self.neighbors(current.pt) {
                let tentative_g_score = g_score[&current.pt] + 1;
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                    g_score.insert(neighbor, tentative_g_score);
                    let fscore = tentative_g_score + self.h(neighbor, self.end);
                    open_set.push(Node {
                        pt: neighbor,
                        fscore,
                    });
                }
            }
        }
        usize::MAX
    }
}

impl Index<USizeVec2> for Map {
    type Output = u8;

    fn index(&self, index: USizeVec2) -> &Self::Output {
        &self.grid[index.y][index.x]
    }
}

impl IndexMut<USizeVec2> for Map {
    fn index_mut(&mut self, index: USizeVec2) -> &mut Self::Output {
        &mut self.grid[index.y][index.x]
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let len = lines.peek().unwrap().len();
        let mut grid = Vec::new();
        let mut start = USizeVec2::ZERO;
        let mut end = USizeVec2::ZERO;
        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::with_capacity(len);
            for (x, c) in line.char_indices() {
                match c {
                    // SAFETY: We already checked `c` was ascii lowercase
                    'a'..='z' => row.push(unsafe { u8::try_from(c).unwrap_unchecked() } - b'a'),
                    'S' => {
                        row.push(0);
                        start = USizeVec2 { x, y };
                    }
                    'E' => {
                        row.push(25);
                        end = USizeVec2 { x, y };
                    }
                    _ => bail!("Unknown char {c}"),
                }
            }
            grid.push(row);
        }
        let height = grid.len();
        Ok(Self {
            grid,
            height,
            len,
            start,
            end,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = input.parse::<Map>().unwrap();
    Some(map.a_star())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = input.parse::<Map>().unwrap();
    Some(map.a_star_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(29));
    }
}
