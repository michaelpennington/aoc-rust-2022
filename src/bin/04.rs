advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(p1, p2)| (p1.split_once('-').unwrap(), p2.split_once('-').unwrap()))
            .map(|((a, b), (c, d))| {
                (
                    (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()),
                    (c.parse::<u64>().unwrap(), d.parse::<u64>().unwrap()),
                )
            })
            .filter(|((a1, b1), (a2, b2))| (a1 <= a2 && b2 <= b1) || (a2 <= a1 && b1 <= b2))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(p1, p2)| (p1.split_once('-').unwrap(), p2.split_once('-').unwrap()))
            .map(|((a, b), (c, d))| {
                (
                    (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()),
                    (c.parse::<u64>().unwrap(), d.parse::<u64>().unwrap()),
                )
            })
            .filter(|((a1, b1), (a2, b2))| {
                (a1 <= a2 && a2 <= b1)
                    || (a1 <= b2 && b2 <= b1)
                    || (a2 <= a1 && a1 <= b2)
                    || (a2 <= b1 && b1 <= b2)
            })
            .count() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
