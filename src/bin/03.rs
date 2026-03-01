use std::collections::HashSet;

advent_of_code::solution!(3);

fn common(s: &str) -> char {
    let len = s.len();
    let (l, r) = s.split_at(len / 2);
    let left = l.chars().collect::<HashSet<_>>();
    r.chars().find(|c| left.contains(c)).unwrap()
}

fn score(c: char) -> u64 {
    match c {
        'a'..='z' => c as u64 - 'a' as u64 + 1,
        'A'..='Z' => c as u64 - 'A' as u64 + 27,
        _ => unimplemented!(),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|l| score(common(l))).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut total = 0;
    while let (Some(l1), Some(l2), Some(l3)) = (lines.next(), lines.next(), lines.next()) {
        let l1cs = l1.chars().collect::<HashSet<_>>();
        let l2cs = l1cs
            .intersection(&l2.chars().collect::<HashSet<_>>())
            .copied()
            .collect::<HashSet<_>>();
        total += score(
            l2cs.intersection(&l3.chars().collect::<HashSet<_>>())
                .copied()
                .next()
                .unwrap(),
        );
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(70));
    }
}
