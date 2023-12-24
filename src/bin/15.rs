advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    // one line, comma seperated list. Seperate out steps into string vector
    let steps = input
        .lines()
        .next()?
        .split(',')
        .collect::<Vec<&str>>();
    
    // run custom hash function on each step
    let hashes = steps.iter().map(|s| {
        let mut current_value = 0;
        // hash is computed by taking the ascii character, adding it to the current value, multiplying it by 17, and then modding 256
        for c in s.chars() {
            current_value = (current_value + c as u32) * 17 & 0xff;
        }
        current_value
    });
    
    Some(hashes.sum())
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
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
