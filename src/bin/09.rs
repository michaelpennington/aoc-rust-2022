use std::collections::HashSet;

use glam::IVec2;

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Chain<const N: usize> {
    head: IVec2,
    tails: [(IVec2, Pos); N],
}

impl<const N: usize> Default for Chain<N> {
    fn default() -> Self {
        Self {
            head: Default::default(),
            tails: [Default::default(); N],
        }
    }
}

impl<const N: usize> Chain<N> {
    /// Returns new position of the tail if it moved
    fn apply(&mut self, dir: Pos) -> Option<IVec2> {
        self.head += <Pos as Into<IVec2>>::into(dir);
        let mut parent = Some(dir);
        let mut i = 0;
        while i < N
            && let Some(rent) = parent.take()
        {
            let child = &mut self.tails[i];
            let (new_pos, new_move) = moves(rent, child.1);
            if let Some(mv) = new_move {
                child.0 += <Pos as Into<IVec2>>::into(mv);
            }
            child.1 = new_pos;
            parent = new_move;
            i += 1;
        }
        parent.map(|_| self.tails.last().unwrap().0)
    }
}

fn moves(dir: Pos, pos: Pos) -> (Pos, Option<Pos>) {
    match (pos, dir) {
        (Pos::NW, Pos::NW)
        | (Pos::N, Pos::N)
        | (Pos::NE, Pos::NE)
        | (Pos::W, Pos::W)
        | (Pos::O, Pos::O)
        | (Pos::E, Pos::E)
        | (Pos::SW, Pos::SW)
        | (Pos::S, Pos::S)
        | (Pos::SE, Pos::SE) => (Pos::O, None),
        (Pos::NW, Pos::N)
        | (Pos::N, Pos::NE)
        | (Pos::W, Pos::O)
        | (Pos::O, Pos::E)
        | (Pos::SW, Pos::S)
        | (Pos::S, Pos::SE) => (Pos::W, None),
        (Pos::SW, Pos::SE) | (Pos::NW, Pos::NE) | (Pos::W, Pos::E) => (Pos::W, Some(Pos::E)),
        (Pos::NW, Pos::E) | (Pos::W, Pos::SE) => (Pos::W, Some(Pos::SE)),
        (Pos::W, Pos::NE) | (Pos::SW, Pos::E) => (Pos::W, Some(Pos::NE)),
        (Pos::NW, Pos::W)
        | (Pos::N, Pos::O)
        | (Pos::NE, Pos::E)
        | (Pos::W, Pos::SW)
        | (Pos::O, Pos::S)
        | (Pos::E, Pos::SE) => (Pos::N, None),
        (Pos::NW, Pos::SW) | (Pos::NE, Pos::SE) | (Pos::N, Pos::S) => (Pos::N, Some(Pos::S)),
        (Pos::NW, Pos::S) | (Pos::N, Pos::SE) => (Pos::N, Some(Pos::SE)),
        (Pos::NE, Pos::S) | (Pos::N, Pos::SW) => (Pos::N, Some(Pos::SW)),
        (Pos::NW, Pos::O) | (Pos::N, Pos::E) | (Pos::W, Pos::S) | (Pos::O, Pos::SE) => {
            (Pos::NW, None)
        }
        (Pos::NW, Pos::SE) => (Pos::NW, Some(Pos::SE)),
        (Pos::N, Pos::NW)
        | (Pos::NE, Pos::N)
        | (Pos::O, Pos::W)
        | (Pos::E, Pos::O)
        | (Pos::SE, Pos::S)
        | (Pos::S, Pos::SW) => (Pos::E, None),
        (Pos::NE, Pos::NW) | (Pos::E, Pos::W) | (Pos::SE, Pos::SW) => (Pos::E, Some(Pos::W)),
        (Pos::E, Pos::NW) | (Pos::SE, Pos::W) => (Pos::E, Some(Pos::NW)),
        (Pos::E, Pos::SW) | (Pos::NE, Pos::W) => (Pos::E, Some(Pos::SW)),
        (Pos::N, Pos::W) | (Pos::NE, Pos::O) | (Pos::O, Pos::SW) | (Pos::E, Pos::S) => {
            (Pos::NE, None)
        }
        (Pos::NE, Pos::SW) => (Pos::NE, Some(Pos::SW)),
        (Pos::W, Pos::NW)
        | (Pos::O, Pos::N)
        | (Pos::E, Pos::NE)
        | (Pos::SW, Pos::W)
        | (Pos::S, Pos::O)
        | (Pos::SE, Pos::E) => (Pos::S, None),
        (Pos::SW, Pos::NW) | (Pos::S, Pos::N) | (Pos::SE, Pos::NE) => (Pos::S, Some(Pos::N)),
        (Pos::SW, Pos::N) | (Pos::S, Pos::NE) => (Pos::S, Some(Pos::NE)),
        (Pos::SE, Pos::N) | (Pos::S, Pos::NW) => (Pos::S, Some(Pos::NW)),
        (Pos::W, Pos::N) | (Pos::O, Pos::NE) | (Pos::SW, Pos::O) | (Pos::S, Pos::E) => {
            (Pos::SW, None)
        }
        (Pos::SW, Pos::NE) => (Pos::SW, Some(Pos::NE)),
        (Pos::O, Pos::NW) | (Pos::E, Pos::N) | (Pos::S, Pos::W) | (Pos::SE, Pos::O) => {
            (Pos::SE, None)
        }
        (Pos::SE, Pos::NW) => (Pos::SE, Some(Pos::NW)),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, strum::EnumString)]
enum Pos {
    #[strum(disabled)]
    NW,
    #[strum(serialize = "U", serialize = "N", ascii_case_insensitive)]
    N,
    #[strum(disabled)]
    NE,
    #[strum(serialize = "L", serialize = "N", ascii_case_insensitive)]
    W,
    #[default]
    #[strum(disabled)]
    O,
    #[strum(serialize = "R", serialize = "E", ascii_case_insensitive)]
    E,
    #[strum(disabled)]
    SW,
    #[strum(serialize = "D", serialize = "S", ascii_case_insensitive)]
    S,
    #[strum(disabled)]
    SE,
}

impl From<Pos> for IVec2 {
    fn from(p: Pos) -> Self {
        match p {
            Pos::NW => IVec2::NEG_ONE,
            Pos::N => IVec2::NEG_Y,
            Pos::NE => IVec2::X + IVec2::NEG_Y,
            Pos::W => IVec2::NEG_X,
            Pos::O => IVec2::ZERO,
            Pos::E => IVec2::X,
            Pos::SW => IVec2::NEG_X + IVec2::Y,
            Pos::S => IVec2::Y,
            Pos::SE => IVec2::ONE,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut chain = Chain::<1>::default();
    let mut tails_seen = HashSet::new();
    tails_seen.insert(chain.tails.last().unwrap().0);
    for line in input.lines() {
        let (dir, len) = line
            .split_once(" ")
            .map(|(dir, len)| (dir.parse::<Pos>().unwrap(), len.parse::<u64>().unwrap()))
            .unwrap();
        for _ in 0..len {
            if let Some(new_tail) = chain.apply(dir) {
                tails_seen.insert(new_tail);
            }
        }
    }
    Some(tails_seen.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut chain = Chain::<9>::default();
    let mut tails_seen = HashSet::new();
    tails_seen.insert(chain.tails.last().unwrap().0);
    for line in input.lines() {
        let (dir, len) = line
            .split_once(" ")
            .map(|(dir, len)| (dir.parse::<Pos>().unwrap(), len.parse::<u64>().unwrap()))
            .unwrap();
        for _ in 0..len {
            if let Some(new_tail) = chain.apply(dir) {
                tails_seen.insert(new_tail);
            }
        }
    }
    Some(tails_seen.len() as u64)
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
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(36));
    }
}
