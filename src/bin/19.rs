use std::{collections::HashMap, str::FromStr};

use bitflags::bitflags;

advent_of_code::solution!(19);

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    struct Action: u32 {
        const ORE = 1;
        const CLAY = 1 << 1;
        const OBSIDIAN = 1 << 2;
        const GEODE = 1 << 3;
    }
}

// TODO: SIMD?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State {
    ore_bots: u32,
    clay_bots: u32,
    obsidian_bots: u32,
    geode_bots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl State {
    fn step_n(&mut self, n: u32) {
        self.ore += self.ore_bots * n;
        self.clay += self.clay_bots * n;
        self.obsidian += self.obsidian_bots * n;
        self.geode += self.geode_bots * n;
    }

    fn normalize(&self, time_left: u32, bp: Blueprint) -> State {
        let mut capped = *self;

        capped.ore = capped.ore.min(time_left * bp.max_ore);
        capped.clay = capped.clay.min(time_left * bp.obsidian.1);
        capped.obsidian = capped.obsidian.min(time_left * bp.geode.1);
        capped
    }

    fn buy(&mut self, robot: Action, bp: Blueprint) {
        match robot {
            Action::ORE => {
                self.ore -= bp.ore;
                self.ore_bots += 1;
            }
            Action::CLAY => {
                self.ore -= bp.clay;
                self.clay_bots += 1;
            }
            Action::OBSIDIAN => {
                self.ore -= bp.obsidian.0;
                self.clay -= bp.obsidian.1;
                self.obsidian_bots += 1;
            }
            Action::GEODE => {
                self.ore -= bp.geode.0;
                self.obsidian -= bp.geode.1;
                self.geode_bots += 1;
            }
            _ => unimplemented!(),
        }
    }

    fn next_robot(&self, bp: Blueprint) -> Action {
        let mut out = Action::empty();
        if self.ore_bots < bp.max_ore {
            out.insert(Action::ORE);
        }
        if self.clay_bots < bp.obsidian.1 {
            out.insert(Action::CLAY);
        }
        if self.obsidian_bots < bp.geode.1 && self.clay_bots > 0 {
            out.insert(Action::OBSIDIAN);
        }
        if self.obsidian_bots > 0 {
            out.insert(Action::GEODE);
        }

        out
    }

    fn time_to(&self, bot: Action, bp: Blueprint) -> u32 {
        match bot {
            Action::ORE => bp.ore.saturating_sub(self.ore).div_ceil(self.ore_bots),
            Action::CLAY => bp.clay.saturating_sub(self.ore).div_ceil(self.ore_bots),
            Action::OBSIDIAN => (bp
                .obsidian
                .0
                .saturating_sub(self.ore)
                .div_ceil(self.ore_bots))
            .max(
                bp.obsidian
                    .1
                    .saturating_sub(self.clay)
                    .div_ceil(self.clay_bots),
            ),
            Action::GEODE => (bp.geode.0.saturating_sub(self.ore).div_ceil(self.ore_bots)).max(
                bp.geode
                    .1
                    .saturating_sub(self.obsidian)
                    .div_ceil(self.obsidian_bots),
            ),
            _ => unimplemented!(),
        }
    }

    fn dfs_v2(&mut self, time_left: u32, bp: Blueprint) -> u32 {
        let mut max_potential_geodes = 0;
        let mut cache = HashMap::new();
        self.dfs_inner(time_left, bp, &mut max_potential_geodes, &mut cache)
    }

    fn dfs_inner(
        &mut self,
        time_left: u32,
        bp: Blueprint,
        max_geodes_found: &mut u32,
        cache: &mut HashMap<(u32, State), u32>,
    ) -> u32 {
        let max_potential_geodes = self.geode
            + (self.geode_bots * time_left)
            + (time_left * time_left.saturating_sub(1)) / 2;
        if max_potential_geodes <= *max_geodes_found {
            return 0;
        }
        let normalized_state = self.normalize(time_left, bp);
        if let Some(&cached_max) = cache.get(&(time_left, normalized_state)) {
            return cached_max;
        }
        let next_bots = self.next_robot(bp);
        let mut max = self.geode + self.geode_bots * time_left;
        for bot in next_bots.iter_names() {
            let time_to = self.time_to(bot.1, bp) + 1;
            if time_to < time_left {
                let mut self_copy = *self;
                self_copy.step_n(time_to);
                self_copy.buy(bot.1, bp);
                max =
                    max.max(self_copy.dfs_inner(time_left - time_to, bp, max_geodes_found, cache));
            }
        }
        *max_geodes_found = (*max_geodes_found).max(max);
        cache.insert((time_left, normalized_state), max);
        max
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Blueprint {
    id: u32,
    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32),
    max_ore: u32,
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace().skip(1);
        let id = words.next().unwrap().trim_end_matches(':').parse()?;
        let ore = words.nth(4).unwrap().parse()?;
        let clay = words.nth(5).unwrap().parse()?;
        let obsidian = (
            words.nth(5).unwrap().parse()?,
            words.nth(2).unwrap().parse()?,
        );
        let geode = (
            words.nth(5).unwrap().parse()?,
            words.nth(2).unwrap().parse()?,
        );
        Ok(Self {
            id,
            ore,
            clay,
            obsidian,
            geode,
            max_ore: ore.max(clay).max(obsidian.0).max(geode.0),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for (id, line) in input.lines().enumerate().map(|(i, line)| (i + 1, line)) {
        let bp = line.parse::<Blueprint>().unwrap();
        let mut state = State::default();
        // let mut max_geodes = 0;
        let geodes = state.dfs_v2(24, bp);
        sum += geodes * id as u32;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 1;
    for line in input.lines().take(3) {
        let bp = line.parse::<Blueprint>().unwrap();
        let mut state = State::default();
        sum *= state.dfs_v2(32, bp) as u64;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3472));
    }
}
