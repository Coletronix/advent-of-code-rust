use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(13);


fn get_char_ascii_grid(grid: &[&str], pos: (i32, i32)) -> Option<char> {
    grid.get(pos.1 as usize)?
        .as_bytes()
        .get(pos.0 as usize)
        .map(|&c| c as char)
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    // input contains a bunch of grids seperated by empty lines.
    let grids = lines.split(|line| line.is_empty()).collect::<Vec<_>>();

    let grid_scores = grids.iter().map(|grid| {
        // bitset containing all columns that are candidates for a line of reflection.
        // starts out with all possible columns set to 1
        let mut column_candidates: u32 = u32::MAX;
        // for each line in the grid, find a list of possible line of reflection candidates
        for (row, &line) in grid.iter().enumerate() {
            let mut line_column_candidates: u32 = 0;
            // an iterator that that supplies the index of the next occurance of two consecutive identical characters
            let double_chars = line
                .char_indices()
                .tuple_windows::<(_, _)>()
                .filter(|((_, a), (_, b))| a == b)
                .map(|((i, _), _)| i);
            
            // now go through each of those, and check left and right to see if the surrounding characters are also identical
            for i in double_chars {
                let mut left = i as i32 - 1;
                let mut right = i as i32 + 2;
                // add this column to the list of candidates, then check if that decision was correct
                line_column_candidates |= 1 << i;
                loop {
                    let left_val = get_char_ascii_grid(grid, (left, row as i32));
                    let right_val = get_char_ascii_grid(grid, (right, row as i32));
                    if left_val.is_some() && right_val.is_some() {
                        if left_val.unwrap() != right_val.unwrap() {
                            // remove that as a candidate
                            column_candidates &= !(1 << i);
                            break;
                        }
                    } else {
                        break;
                    }
                    left -= 1;
                    right += 1;
                }
            }
            column_candidates &= line_column_candidates;
        }
        
        match column_candidates.count_ones() {
            // in this case, there are no candidates, so the grid must be horizontal. Run the above algorithm vertically
            0 => {
                // println!("No candidates found for grid, so it must be horizontal: {:?}", grid);
                
                let mut row_candidates: u32 = u32::MAX;
                // need to go through each column. Since they aren't contiguous in memory,
                // we need to construct a custom iterator that provides vertical characters
                let column_iter = (0..grid[0].len()).map(|i| {
                    grid.iter().map(move |line| line.as_bytes()[i] as char)
                });
                for (col, line) in column_iter.enumerate() {
                    let mut line_row_candidates: u32 = 0;
                    let double_chars = line
                        .enumerate()
                        .tuple_windows::<(_, _)>()
                        .filter(|((_, a), (_, b))| a == b)
                        .map(|((i, _), _)| i);
                    for i in double_chars {
                        let mut top = i as i32 - 1;
                        let mut bottom = i as i32 + 2;
                        line_row_candidates |= 1 << i;
                        loop {
                            let top_val = get_char_ascii_grid(grid, (col as i32, top));
                            let bottom_val = get_char_ascii_grid(grid, (col as i32, bottom));
                            if top_val.is_some() && bottom_val.is_some() {
                                if top_val.unwrap() != bottom_val.unwrap() {
                                    row_candidates &= !(1 << i);
                                    break;
                                }
                            } else {
                                break;
                            }
                            top -= 1;
                            bottom += 1;
                        }
                    }
                    row_candidates &= line_row_candidates;
                }
                match row_candidates.count_ones() {
                    0 => panic!("No candidates found for grid: {:?}", grid),
                    1 => (row_candidates.trailing_zeros()+1) * 100, // per AOC instructions
                    _ => panic!("Multiple candidates found for grid: {:?}, {:b}", grid, row_candidates),
                }
            },
            1 => column_candidates.trailing_zeros()+1,
            _ => panic!("Multiple candidates found for grid: {:?}, {:b}", grid, column_candidates),
        }
    });

    Some(grid_scores.sum())
}

#[derive(Debug, PartialEq, Eq)]
enum Reflection {
    Horizontal(u32),
    Vertical(u32),
    None,
}

fn find_reflection(grid: &[&str]) -> Reflection {
    // bitset containing all columns that are candidates for a line of reflection.
    // starts out with all possible columns set to 1
    let mut column_candidates: u32 = u32::MAX;
    // for each line in the grid, find a list of possible line of reflection candidates
    for (row, &line) in grid.iter().enumerate() {
        let mut line_column_candidates: u32 = 0;
        // an iterator that that supplies the index of the next occurance of two consecutive identical characters
        let double_chars = line
            .char_indices()
            .tuple_windows::<(_, _)>()
            .filter(|((_, a), (_, b))| a == b)
            .map(|((i, _), _)| i);
        
        // now go through each of those, and check left and right to see if the surrounding characters are also identical
        for i in double_chars {
            let mut left = i as i32 - 1;
            let mut right = i as i32 + 2;
            // add this column to the list of candidates, then check if that decision was correct
            line_column_candidates |= 1 << i;
            loop {
                let left_val = get_char_ascii_grid(grid, (left, row as i32));
                let right_val = get_char_ascii_grid(grid, (right, row as i32));
                if left_val.is_some() && right_val.is_some() {
                    if left_val.unwrap() != right_val.unwrap() {
                        // remove that as a candidate
                        column_candidates &= !(1 << i);
                        break;
                    }
                } else {
                    break;
                }
                left -= 1;
                right += 1;
            }
        }
        column_candidates &= line_column_candidates;
    }
    
    match column_candidates.count_ones() {
        // in this case, there are no candidates, so the grid must be horizontal. Run the above algorithm vertically
        0 => {
            // println!("No candidates found for grid, so it must be horizontal: {:?}", grid);
            
            let mut row_candidates: u32 = u32::MAX;
            // need to go through each column. Since they aren't contiguous in memory,
            // we need to construct a custom iterator that provides vertical characters
            let column_iter = (0..grid[0].len()).map(|i| {
                grid.iter().map(move |line| line.as_bytes()[i] as char)
            });
            for (col, line) in column_iter.enumerate() {
                let mut line_row_candidates: u32 = 0;
                let double_chars = line
                    .enumerate()
                    .tuple_windows::<(_, _)>()
                    .filter(|((_, a), (_, b))| a == b)
                    .map(|((i, _), _)| i);
                for i in double_chars {
                    let mut top = i as i32 - 1;
                    let mut bottom = i as i32 + 2;
                    line_row_candidates |= 1 << i;
                    loop {
                        let top_val = get_char_ascii_grid(grid, (col as i32, top));
                        let bottom_val = get_char_ascii_grid(grid, (col as i32, bottom));
                        if top_val.is_some() && bottom_val.is_some() {
                            if top_val.unwrap() != bottom_val.unwrap() {
                                row_candidates &= !(1 << i);
                                break;
                            }
                        } else {
                            break;
                        }
                        top -= 1;
                        bottom += 1;
                    }
                }
                row_candidates &= line_row_candidates;
            }
            match row_candidates.count_ones() {
                0 => Reflection::None,
                1 => Reflection::Horizontal((row_candidates.trailing_zeros()+1) * 100), // per AOC instructions
                _ => Reflection::None,
            }
        },
        1 => Reflection::Vertical(column_candidates.trailing_zeros()+1),
        _ => Reflection::None,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    // input contains a bunch of grids seperated by empty lines.
    let grids = lines.split(|line| line.is_empty()).collect::<Vec<_>>();

    let grid_scores = grids.par_iter().map(|grid| {
        // find the default reflection of the grid
        let default_reflection = find_reflection(grid);
        let mut changed_reflection = Reflection::Horizontal(0); // placeholder value

        // now create a new grid for each character in the original grid, with one character flipped
        'outer_loop: for (row, line) in grid.iter().enumerate() {
            for (col, c) in line.char_indices() {
                // Deep clone of grid
                let mut clone_grid = grid.iter().map(|&line| line.to_owned()).collect::<Vec<String>>();

                // Flip character
                let flipped_char = match c {
                    '#' => '.',
                    '.' => '#',
                    _ => c,
                };

                // Replace character in the clone
                clone_grid[row].replace_range(col..=col, &flipped_char.to_string());

                // Check reflection
                let clone_grid_refs: Vec<&str> = clone_grid.iter().map(String::as_str).collect();
                let reflection = find_reflection(&clone_grid_refs);
                if reflection != default_reflection && reflection != Reflection::None {
                    changed_reflection = reflection;
                    break 'outer_loop;
                }
            }
        }
        
        // return the score of the reflection that changed
        match changed_reflection {
            Reflection::Horizontal(score) => score,
            Reflection::Vertical(score) => 100 * score, // per spec
            _ => panic!("Not possible to get here"),
        }
    });

    Some(grid_scores.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
