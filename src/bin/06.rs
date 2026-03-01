use std::collections::HashMap;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let chars = input.chars().collect::<Vec<_>>();
    Some(
        chars
            .windows(4)
            .position(|w| {
                w[0] != w[1]
                    && w[0] != w[2]
                    && w[0] != w[3]
                    && w[1] != w[2]
                    && w[1] != w[3]
                    && w[2] != w[3]
            })
            .unwrap() as u64
            + 4,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cs: HashMap<char, usize> = HashMap::with_capacity(14);
    for new_char in input.chars().take(14) {
        *cs.entry(new_char).or_default() += 1;
    }
    for ((i, new_char), old_char) in input.char_indices().skip(14).zip(input.chars()) {
        match cs.get_mut(&old_char) {
            Some(1) => {
                cs.remove(&old_char);
            }
            Some(i) => *i -= 1,
            None => unreachable!(),
        };
        *cs.entry(new_char).or_default() += 1;
        if cs.len() == 14 {
            return Some(i as u64 + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_four() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_two_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(29));
    }

    #[test]
    fn test_part_two_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(26));
    }
}
