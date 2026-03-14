use std::{collections::HashMap, fmt::Display};

advent_of_code::solution!(17);

#[derive(Debug)]
struct Tower<'a> {
    /// true = left, false = right
    wind: &'a [bool],
    wind_ptr: usize,
    blocks: Vec<u8>,
    height: usize,
}

impl Display for Tower<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.blocks.iter().rev() {
            write!(f, "|")?;
            for bit_pos in 0..7 {
                let c = if (line >> bit_pos) & 1 == 0 { '.' } else { '#' };
                write!(f, "{c}")?
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")
    }
}

/// rock_id, wind_idx, top_rows
type StateKey = (usize, usize, [u8; 30]);

impl<'a> Tower<'a> {
    fn new(wind: &'a [bool]) -> Self {
        Self {
            wind,
            wind_ptr: 0,
            blocks: Vec::new(),
            height: 0,
        }
    }

    fn drop(&mut self, block: Block) {
        let mut height = self.height;
        let mut block = block;
        for _ in 0..3 {
            self.blow_naive(&mut block);
            self.wind_ptr += 1;
        }
        while self.blow(&mut block, height) {
            height -= 1;
        }
        for (i, b) in block.iter().enumerate() {
            if let Some(row) = self.blocks.get_mut(i + height) {
                *row |= b;
            } else {
                self.blocks.push(*b);
            }
        }
        while self.blocks.last().is_some_and(|l| *l == 0) {
            self.blocks.pop();
        }
        self.height = self.blocks.len();
    }

    fn drop2(&mut self, num_times: usize) -> u64 {
        let mut map: HashMap<StateKey, _> = HashMap::new();
        let mut block_iter = BLOCKS
            .into_iter()
            .enumerate()
            .cycle()
            .take(num_times)
            .enumerate();
        let (height_per_cycle, num_skipped, leftover_blocks) = loop {
            if let Some((block_count, (block_id, block))) = block_iter.next() {
                self.drop(block);
                if let Some(last) = map.insert(
                    (block_id, self.wind_ptr % self.wind.len(), self.topo()),
                    (block_count, self.height),
                ) {
                    let cycle_len = block_count - last.0;
                    let height_per_cycle = self.height - last.1;
                    let remaining_blocks = num_times - (block_count + 1);
                    let num_skipped = remaining_blocks / cycle_len;
                    let leftover_blocks = remaining_blocks % cycle_len;
                    break (height_per_cycle, num_skipped, leftover_blocks);
                }
            } else {
                return self.height();
            }
        };
        let height_offset = (height_per_cycle * num_skipped) as u64;
        for rock_id in ((num_times - leftover_blocks)..num_times).map(|n| n % 5) {
            self.drop(BLOCKS[rock_id]);
        }
        self.height() + height_offset
    }

    fn height(&self) -> u64 {
        self.height as u64
    }

    fn topo(&self) -> [u8; 30] {
        let mut out = [0; 30];
        for (i, &r) in self.blocks.iter().rev().take(30).enumerate() {
            out[i] = r;
        }
        out
    }

    fn blow_naive(&self, block: &mut [u8; 4]) {
        let wind = self.wind[self.wind_ptr % self.wind.len()];
        if wind {
            if block.iter().any(|l| l & 0b0000_0001 != 0) {
                return;
            }
            for b in block {
                *b >>= 1;
            }
        } else {
            if block.iter().any(|l| l & 0b0100_0000 != 0) {
                return;
            }
            for b in block {
                *b <<= 1;
            }
        }
    }

    /// returns true if we can keep going, false if not
    fn blow(&mut self, block: &mut [u8; 4], height: usize) -> bool {
        let mut b1 = *block;
        self.blow_naive(&mut b1);
        self.wind_ptr += 1;
        if !self.collision(&b1, height) {
            *block = b1;
        }
        height != 0 && !self.collision(block, height - 1)
    }

    fn collision(&self, block: &[u8; 4], height: usize) -> bool {
        for (i, b) in block.iter().enumerate() {
            if let Some(r) = self.blocks.get(height + i)
                && r & b != 0
            {
                return true;
            }
        }
        false
    }
}

type Block = [u8; 4];

const BLOCKS: [Block; 5] = [
    [0b00111100, 0, 0, 0],
    [0b00001000, 0b00011100, 0b00001000, 0],
    [0b00011100, 0b00010000, 0b00010000, 0],
    [0b00000100, 0b00000100, 0b00000100, 0b00000100],
    [0b00001100, 0b00001100, 0, 0],
];

pub fn part_one(input: &str) -> Option<u64> {
    let wind_pattern: Vec<_> = input
        .chars()
        .filter(|&c| c == '<' || c == '>')
        .map(|c| c == '<')
        .collect();
    let mut tower = Tower::new(&wind_pattern);
    for block in BLOCKS.iter().copied().cycle().take(2022) {
        tower.drop(block);
    }

    Some(tower.height())
}

pub fn part_two(input: &str) -> Option<u64> {
    let wind_pattern: Vec<_> = input
        .chars()
        .filter(|&c| c == '<' || c == '>')
        .map(|c| c == '<')
        .collect();
    let mut tower = Tower::new(&wind_pattern);
    Some(tower.drop2(1000000000000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3068));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1514285714288));
    }
}
