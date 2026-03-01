advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    input
        .split("\n\n")
        .map(|cal| cal.lines().map(|i| i.parse::<u64>().unwrap()).sum::<u64>())
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .split("\n\n")
            .map(|cal| cal.lines().map(|i| i.parse::<u64>().unwrap()).sum::<u64>())
            .fold([0, 0, 0], |[a, b, c], sum| {
                if sum > a {
                    [sum, a, b]
                } else if sum > b {
                    [a, sum, b]
                } else if sum > c {
                    [a, b, sum]
                } else {
                    [a, b, c]
                }
            })
            .iter()
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
