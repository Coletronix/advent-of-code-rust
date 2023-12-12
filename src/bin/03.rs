use core::panic;

advent_of_code::solution!(3);

#[derive(Debug)]
struct PartNumber {
    row: i32,
    left_pos: i32,
    right_pos: i32,
}

impl PartNumber {
    fn parse_to_num(&self, lines: &Vec<&str>) -> u32 {
        // we already know the position of it, now just get the string slice and parse it
        let num_str = lines[self.row as usize].chars().skip(self.left_pos as usize).take((self.right_pos - self.left_pos + 1) as usize).collect::<String>();
        let thing = num_str.parse::<u32>();
        match thing {
            Ok(num) => num,
            Err(e) => {
                println!("Error parsing number: {}. Raw str: {:?}", e, num_str);
                panic!();
            }
        }
    }
}

// parse out all numbers (concecutive digits) and their positions
fn parse_out_part_nums(lines: &Vec<&str>) -> Vec<PartNumber> {
    let mut part_nums: Vec<PartNumber> = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        let mut left_pos_set = false;
        let mut left_pos = 0;
        let mut right_pos = 0;
        for (col, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                right_pos = col;
                if !left_pos_set {
                    left_pos = col;
                    left_pos_set = true;
                }
            } else if left_pos_set {
                left_pos_set = false;
                // we have a number
                part_nums.push(PartNumber {
                    row: row as i32,
                    left_pos: left_pos as i32,
                    right_pos: right_pos as i32,
                });
            }
        }
        // check end case
        if left_pos_set {
            part_nums.push(PartNumber {
                row: row as i32,
                left_pos: left_pos as i32,
                right_pos: right_pos as i32,
            });
        }
    }
    part_nums
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    // parse out all numbers (concecutive digits) and their positions
    let part_nums = parse_out_part_nums(&lines);
    
    // for each of the part numbers, check around it for a symbol that isn't '.'
    let valid_part_nums = part_nums.iter().filter(|part_num| {
        if let Some(line) = lines.get(part_num.row as usize) {
            // check to the left
            if part_num.left_pos != 0 {
                if let Some(c) = line.get((part_num.left_pos - 1) as usize..(part_num.left_pos) as usize) {
                    if c.chars().next().unwrap() != '.' {
                        return true
                    }
                }
            }
            
            // check to the right
            if part_num.right_pos != line.len() as i32 - 1 {
                if let Some(c) = line.get((part_num.right_pos + 1) as usize..(part_num.right_pos + 2) as usize) {
                    if c.chars().next().unwrap() != '.' {
                        return true
                    }
                }
            }
        }
        // now check lines above and below
        if part_num.row != 0 {
            if let Some(line_up) = lines.get((part_num.row - 1) as usize) {
                if let Some(c) = line_up.get((part_num.left_pos-1).max(0) as usize..(part_num.right_pos+2).min(line_up.len() as i32) as usize) {
                    if c.chars().any(|c| c != '.') {
                        return true
                    }
                }
            }
        }
        if part_num.row != lines.len() as i32 - 1 {
            if let Some(line_down) = lines.get((part_num.row + 1) as usize) {
                if let Some(c) = line_down.get((part_num.left_pos-1).max(0) as usize..(part_num.right_pos+2).min(line_down.len() as i32) as usize) {
                    if c.chars().any(|c| c != '.') {
                        return true
                    }
                }
            }
        }
        false
    });
    
    Some(valid_part_nums.map(|part_num| part_num.parse_to_num(&lines)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
