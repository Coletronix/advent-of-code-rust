use core::panic;
use std::collections::HashMap;

advent_of_code::solution!(3);

#[derive(Debug)]
struct PartNumber {
    row: i32,
    left_pos: i32,
    right_pos: i32,
}

impl PartNumber {
    fn parse_to_num(&self, lines: &[&str]) -> u32 {
        // we already know the position of it, now just get the string slice and parse it
        let num_str = lines[self.row as usize]
            .chars()
            .skip(self.left_pos as usize)
            .take((self.right_pos - self.left_pos + 1) as usize)
            .collect::<String>();
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
fn parse_out_part_nums(lines: &[&str]) -> Vec<PartNumber> {
    let mut part_nums: Vec<PartNumber> = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        let mut left_pos_set = false;
        let mut left_pos = 0;
        let mut right_pos = 0;
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
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

#[inline]
fn get_ascii_char_at(source: &str, index: usize) -> Option<char> {
    source.as_bytes().get(index).map(|c| *c as char)
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    // parse out all numbers (concecutive digits) and their positions
    let part_nums = parse_out_part_nums(&lines);

    // for each of the part numbers, check around it for a symbol that isn't '.'
    let valid_part_nums = part_nums.iter().filter(|part_num| {
        // current line
        if let Some(line) = lines.get(part_num.row as usize) {
            // check to the left
            if let Some(c) = get_ascii_char_at(line, (part_num.left_pos - 1) as usize) {
                if c != '.' {
                    return true;
                }
            }

            // check to the right
            if let Some(c) = get_ascii_char_at(line, (part_num.right_pos + 1) as usize) {
                if c != '.' {
                    return true;
                }
            }
        }
        // line above
        if let Some(line) = lines.get((part_num.row - 1) as usize) {
            for c in (part_num.left_pos - 1)..(part_num.right_pos + 2) {
                if let Some(c) = get_ascii_char_at(line, c as usize) {
                    if c != '.' {
                        return true;
                    }
                }
            }
        }
        // line below
        if let Some(line) = lines.get((part_num.row + 1) as usize) {
            for c in (part_num.left_pos - 1)..(part_num.right_pos + 2) {
                if let Some(c) = get_ascii_char_at(line, c as usize) {
                    if c != '.' {
                        return true;
                    }
                }
            }
        }
        false
    });

    Some(
        valid_part_nums
            .map(|part_num| part_num.parse_to_num(&lines))
            .sum(),
    )
}

// needs to be printed and put in a hashmap
#[derive(Debug)]
struct Gear<'a> {
    parts_adjacent: Vec<&'a PartNumber>,
}

impl Gear<'_> {
    fn get_gear_ratio(&self, lines: &[&str]) -> Option<u32> {
        // none if not exactly 2 parts adjacent. Otherwise product of two parts
        if self.parts_adjacent.len() != 2 {
            return None;
        }
        let part1 = self.parts_adjacent[0].parse_to_num(lines);
        let part2 = self.parts_adjacent[1].parse_to_num(lines);
        Some(part1 * part2)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    // parse out all numbers (concecutive digits) and their positions
    let part_nums = parse_out_part_nums(&lines);

    // hashmap to store unique gears
    let mut gears: HashMap<(u32, u32), Gear> = HashMap::new();

    // for each of the part numbers, check around it for a symbol that isn't '.'
    for part_num in part_nums.iter() {
        // current line
        if let Some(line) = lines.get(part_num.row as usize) {
            // check to the left
            if let Some(c) = get_ascii_char_at(line, (part_num.left_pos - 1) as usize) {
                if c == '*' {
                    gears
                        .entry((part_num.row as u32, (part_num.left_pos - 1) as u32))
                        .or_insert(Gear {
                            parts_adjacent: Vec::new(),
                        })
                        .parts_adjacent
                        .push(part_num);
                }
            }

            // check to the right
            if let Some(c) = get_ascii_char_at(line, (part_num.right_pos + 1) as usize) {
                if c == '*' {
                    gears
                        .entry((part_num.row as u32, (part_num.right_pos + 1) as u32))
                        .or_insert(Gear {
                            parts_adjacent: Vec::new(),
                        })
                        .parts_adjacent
                        .push(part_num);
                }
            }
        }
        // line above
        if let Some(line) = lines.get((part_num.row - 1) as usize) {
            for col in (part_num.left_pos - 1)..(part_num.right_pos + 2) {
                if let Some(c) = get_ascii_char_at(line, col as usize) {
                    if c == '*' {
                        gears
                            .entry(((part_num.row - 1) as u32, col as u32))
                            .or_insert(Gear {
                                parts_adjacent: Vec::new(),
                            })
                            .parts_adjacent
                            .push(part_num);
                    }
                }
            }
        }
        // line below
        if let Some(line) = lines.get((part_num.row + 1) as usize) {
            for col in (part_num.left_pos - 1)..(part_num.right_pos + 2) {
                if let Some(c) = get_ascii_char_at(line, col as usize) {
                    if c == '*' {
                        gears
                            .entry(((part_num.row + 1) as u32, col as u32))
                            .or_insert(Gear {
                                parts_adjacent: Vec::new(),
                            })
                            .parts_adjacent
                            .push(part_num);
                    }
                }
            }
        }
    }

    Some(
        gears
            .iter()
            .filter_map(|(_, gear)| gear.get_gear_ratio(&lines))
            .sum(),
    )
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
        assert_eq!(result, Some(467835));
    }
}
