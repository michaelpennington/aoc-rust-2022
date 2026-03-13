use std::collections::{HashMap, HashSet};

use advent_of_code::util::graph::Graph;

advent_of_code::solution!(16);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Valve {
    flow_rate: u64,
}

#[derive(Debug)]
struct Pipes {
    g: Graph<Valve, usize, usize>,
    valuable_valves: Vec<usize>,
}

#[inline(always)]
fn str_to_idx(s: &str) -> usize {
    let bytes = s.as_bytes();
    let char1 = (bytes[0] - b'A') as usize;
    let char2 = (bytes[1] - b'A') as usize;
    debug_assert!(char1 < 26 && char2 < 26);
    (char1 * 26) + char2
}

impl Pipes {
    fn from_str(s: &str) -> Self {
        let mut g = Graph::new();
        let mut added_nodes = HashSet::new();
        let mut waiting_on: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut valuable_valves = Vec::new();
        for line in s.lines() {
            let (begin, end) = line.split_once("; ").unwrap();
            let (valve_name, flow_rate) = begin
                .trim_start_matches("Valve ")
                .split_once(" has flow rate=")
                .unwrap();
            let valve_name = str_to_idx(valve_name);
            let valve = Valve {
                flow_rate: flow_rate.parse::<u64>().unwrap(),
            };
            if valve.flow_rate > 0 {
                valuable_valves.push(valve_name);
            }
            g.add_node(valve_name, valve);
            added_nodes.insert(valve_name);
            if let Some(w) = waiting_on.remove(&valve_name) {
                for label in w {
                    g.add_edge(valve_name, label, 1);
                    g.add_edge(label, valve_name, 1);
                }
            }
            for label in end
                .split_whitespace()
                .skip(4)
                .map(|l| l.trim_end_matches(','))
                .map(str_to_idx)
            {
                if added_nodes.contains(&label) {
                    g.add_edge(valve_name, label, 1);
                    g.add_edge(label, valve_name, 1);
                } else {
                    waiting_on.entry(label).or_default().push(valve_name);
                }
            }
        }
        valuable_valves.sort();
        Self { g, valuable_valves }
    }

    fn compress(self) -> Self {
        let old_g = self.g;
        let mut g = Graph::new();
        g.add_node(0, Valve { flow_rate: 0 });
        for valve in &self.valuable_valves {
            let v = old_g.get_node(valve).unwrap();
            g.add_node(*valve, *v);
        }
        for (node, d) in old_g.dijkstra(&0) {
            if self.valuable_valves.contains(&node) {
                g.add_edge(0, node, d);
            }
        }
        for valve in &self.valuable_valves {
            for (node, d) in old_g.dijkstra(valve) {
                if node != *valve && self.valuable_valves.contains(&node) {
                    g.add_edge(*valve, node, d);
                    g.add_edge(node, *valve, d);
                }
            }
        }
        let valuable_valves = self.valuable_valves;
        Self { g, valuable_valves }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct DfsState {
    current: usize,
    time_remaining: usize,
    visited: u32,
}

fn dfs(
    state: DfsState,
    g: &Graph<Valve>,
    bit_ids: &HashMap<usize, u32>,
    memo: &mut HashMap<DfsState, u64>,
) -> u64 {
    if state.time_remaining == 0 {
        return 0;
    }

    if let Some(&cached_result) = memo.get(&state) {
        return cached_result;
    }

    let mut max_pressure = 0;

    let Some(neighbors) = g.neighbors(&state.current) else {
        return 0;
    };
    for (&next_node, cost) in neighbors
        .map(|(nn, tt)| (nn, tt + 1))
        .filter(|&(_, tt)| tt < state.time_remaining)
    {
        let bit_id = bit_ids.get(&next_node).unwrap();
        if (state.visited & (1 << bit_id)) == 0 {
            let next_time = state.time_remaining - cost;
            let flow_rate = g.get_node(&next_node).unwrap().flow_rate;
            let points = (next_time as u64) * flow_rate;
            let next_visited = state.visited | (1 << bit_id);
            let path_pressure = points
                + dfs(
                    DfsState {
                        current: next_node,
                        time_remaining: next_time,
                        visited: next_visited,
                    },
                    g,
                    bit_ids,
                    memo,
                );
            max_pressure = max_pressure.max(path_pressure);
        }
    }
    memo.insert(state, max_pressure);
    max_pressure
}

pub fn part_one(input: &str) -> Option<u64> {
    let pipes = Pipes::from_str(input).compress();
    let mut bit_ids = HashMap::new();
    for (i, &valve) in pipes.valuable_valves.iter().enumerate() {
        bit_ids.insert(valve, i as u32);
    }

    let mut memo = HashMap::new();
    let result = dfs(
        DfsState {
            current: 0,
            time_remaining: 30,
            visited: 0,
        },
        &pipes.g,
        &bit_ids,
        &mut memo,
    );
    Some(result)
}

fn build_path_map(
    state: DfsState,
    current_pressure: u64,
    g: &Graph<Valve>,
    bit_ids: &HashMap<usize, u32>,
    best_paths: &mut HashMap<u32, u64>,
) {
    let current_best = best_paths.get(&state.visited).copied().unwrap_or(0);
    if current_pressure > current_best {
        best_paths.insert(state.visited, current_pressure);
    }

    if state.time_remaining == 0 {
        return;
    }
    let neighbors = g.neighbors(&state.current).unwrap();

    for (&next_node, cost) in neighbors
        .map(|(nn, tt)| (nn, tt + 1))
        .filter(|&(_, tt)| tt < state.time_remaining)
    {
        let bit_id = bit_ids.get(&next_node).unwrap();

        if (state.visited & (1 << bit_id)) == 0 {
            let next_time = state.time_remaining - cost;
            let flow_rate = g.get_node(&next_node).unwrap().flow_rate;
            let points = (next_time as u64) * flow_rate;
            let next_visited = state.visited | (1 << bit_id);

            build_path_map(
                DfsState {
                    current: next_node,
                    time_remaining: next_time,
                    visited: next_visited,
                },
                current_pressure + points,
                g,
                bit_ids,
                best_paths,
            );
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let pipes = Pipes::from_str(input).compress();
    let mut bit_ids = HashMap::new();
    for (i, &valve) in pipes.valuable_valves.iter().enumerate() {
        bit_ids.insert(valve, i as u32);
    }
    let mut best_paths = HashMap::new();

    build_path_map(
        DfsState {
            current: 0,
            time_remaining: 26,
            visited: 0,
        },
        0,
        &pipes.g,
        &bit_ids,
        &mut best_paths,
    );
    let mut max_pressure = 0;
    let paths: Vec<(u32, u64)> = best_paths.into_iter().collect();
    for (i, (mask_human, score_human)) in paths.iter().enumerate() {
        for (mask_elephant, score_elephant) in &paths[i + 1..] {
            if (mask_elephant & mask_human) == 0 {
                max_pressure = max_pressure.max(score_human + score_elephant);
            }
        }
    }

    Some(max_pressure)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1651));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1707));
    }
}
