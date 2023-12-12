advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    Some(lines.iter().map(|line| parse_line(line)).sum())
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_line(line: &str) -> u32 {
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
            } else if let Some(index) = word_match {
                Some(index as u32)
            } else {
                None
            }
        });
    
    // this is ok because solution will always have at least one number
    let first = numbers_iter.next().unwrap();
    let last = numbers_iter.next_back().unwrap_or(first);
    first * 10 + last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("result: {:?}", result);
        assert_eq!(result, Some(53894));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
