advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    
    let parsed_lines = lines.iter().map(|line| {
        let mut numbers_iter = line.chars().filter_map(|c| c.to_digit(10));
        
        // this is ok because input text will always have at least one number
        let first = numbers_iter.next().unwrap();
        let last = numbers_iter.next_back().unwrap_or(first);
        first * 10 + last
    });
    Some(parsed_lines.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let parsed_lines = lines.iter().map(|line| {
        let match_strings = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let mut numbers_iter = line.char_indices()
            .map(|(i, c)| {
                let after_str = &line[i..];
                (c, after_str)
            })
            .filter_map(|(c, after_str)| {
                let word_match = match_strings.iter().enumerate().find_map(|(i, &s)| {
                    if after_str.starts_with(s) { Some(i) } else { None }
                });

                if let Some(digit) = c.to_digit(10) {
                    Some(digit)
                } else {
                    word_match.map(|i| i as u32)
                }
            });
        
        // this is ok because input text will always have at least one number
        let first = numbers_iter.next().unwrap();
        let last = numbers_iter.next_back().unwrap_or(first);
        first * 10 + last
    });
    Some(parsed_lines.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
