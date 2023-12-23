advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut total_arrangements = 0;
    for line in lines {}

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let _lines = input.lines().collect::<Vec<&str>>();
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
