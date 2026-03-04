use std::str::FromStr;

use anyhow::anyhow;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Op {
    AddX(isize),
    Noop,
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let instruction = words.next().unwrap();
        match instruction {
            "noop" => Ok(Op::Noop),
            "addx" => {
                let param = words.next().unwrap().parse()?;
                Ok(Op::AddX(param))
            }
            _ => Err(anyhow!("Unknown instruction {s}")),
        }
    }
}

impl Op {
    fn len(&self) -> usize {
        match self {
            Op::AddX(_) => 2,
            Op::Noop => 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Cpu {
    x: isize,
    pc: usize,
    pending_op: Option<(Op, usize)>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x: 1,
            pc: 0,
            pending_op: Default::default(),
        }
    }
}

impl Cpu {
    fn step<T>(&mut self, instructions: &mut T) -> bool
    where
        T: Iterator<Item = Op>,
    {
        if let Some((pending_op, remaining_steps)) = self.pending_op.take() {
            if remaining_steps == 0 {
                match pending_op {
                    Op::AddX(i) => self.x += i,
                    Op::Noop => {}
                }
            } else {
                self.pending_op = Some((pending_op, remaining_steps - 1))
            }
        }
        self.pc += 1;
        if self.pending_op.is_none() {
            if let Some(op) = instructions.next() {
                self.pending_op = Some((op, op.len() - 1));
            } else {
                return false;
            }
        }
        true
    }

    fn step_print<T>(&mut self, instructions: &mut T, out_str: &mut String) -> bool
    where
        T: Iterator<Item = Op>,
    {
        if let Some((pending_op, remaining_steps)) = self.pending_op.take() {
            if remaining_steps == 0 {
                match pending_op {
                    Op::AddX(i) => self.x += i,
                    Op::Noop => {}
                }
            } else {
                self.pending_op = Some((pending_op, remaining_steps - 1))
            }
        }
        if self.pending_op.is_none() {
            if let Some(op) = instructions.next() {
                self.pending_op = Some((op, op.len() - 1));
            } else {
                out_str.push('\n');
                return false;
            }
        }
        let col_num = (self.pc as isize - 1) % 40 + 1;

        if col_num == self.x || col_num == self.x + 1 || col_num == self.x - 1 {
            out_str.push('#');
        } else {
            out_str.push('.');
        }
        if col_num == 40 {
            out_str.push('\n');
        }
        self.pc += 1;
        true
    }

    fn nstep<T>(&mut self, instructions: &mut T, num: isize) -> Option<isize>
    where
        T: Iterator<Item = Op>,
    {
        for _ in 0..num {
            if !self.step(instructions) {
                return None;
            }
        }
        Some(self.x)
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut ops = input.lines().map(|l| l.parse::<Op>().unwrap());
    let mut cpu = Cpu::default();
    let mut total = 0;
    for inc in [0, 20, 60, 100, 140, 180, 220].windows(2) {
        total += inc[1] * cpu.nstep(&mut ops, inc[1] - inc[0]).unwrap();
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut ops = input.lines().map(|l| l.parse::<Op>().unwrap());
    let mut cpu = Cpu::default();
    let mut out_str = String::with_capacity(80 * 6);
    while cpu.step_print(&mut ops, &mut out_str) {}
    Some(out_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            Some(
                r#"##..##..##..##..##..##..##..##..##..##...
##...###...###...###...###...###...###..
###....####....####....####....####.....
####.....#####.....#####.....#####......
#####......######......######......####.
######.......#######.......#######.....
"#
                .into()
            )
        );
    }
}
