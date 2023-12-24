use std::collections::{VecDeque, HashSet};

use rayon::prelude::*;

advent_of_code::solution!(16);

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Beam {
    position: (usize, usize),
    direction: (isize, isize),
}

fn num_touched_tiles(grid: &[&str], start_beam: Beam) -> u32 {
    let mut visited_states = HashSet::new();
    let mut visited_positions = HashSet::new();
    
    let mut queue = VecDeque::new();
    queue.push_back(start_beam);

    while let Some(beam) = queue.pop_front() {
        let valid_position = beam.position.0 < grid[0].len() && beam.position.1 < grid.len();

        if !valid_position || visited_states.contains(&beam) {
            continue;
        }
        
        visited_states.insert(beam);
        visited_positions.insert(beam.position);

        match (grid[beam.position.1].as_bytes()[beam.position.0] as char, beam.direction) {
            // blank space should just continue on in the direction it was traveling
            ('.', _) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(beam.direction.0), (beam.position.1).wrapping_add_signed(beam.direction.1));
                queue.push_back(Beam { position: new_beam_pos, direction: beam.direction });
            },
            ('/', (1, 0)) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(0), (beam.position.1).wrapping_add_signed(-1));
                queue.push_back(Beam { position: new_beam_pos, direction: (0, -1) });
            },
            ('/', (0, 1)) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(-1), (beam.position.1).wrapping_add_signed(0));
                queue.push_back(Beam { position: new_beam_pos, direction: (-1, 0) });
            },
            ('/', (-1, 0)) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(0), (beam.position.1).wrapping_add_signed(1));
                queue.push_back(Beam { position: new_beam_pos, direction: (0, 1) });
            },
            ('/', (0, -1)) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(1), (beam.position.1).wrapping_add_signed(0));
                queue.push_back(Beam { position: new_beam_pos, direction: (1, 0) });
            },
            ('\\', (1, 0)) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(0), (beam.position.1).wrapping_add_signed(1));
                queue.push_back(Beam { position: new_beam_pos, direction: (0, 1) });
            },
            ('\\', (0, 1)) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(1), (beam.position.1).wrapping_add_signed(0));
                queue.push_back(Beam { position: new_beam_pos, direction: (1, 0) });
            },
            ('\\', (-1, 0)) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(0), (beam.position.1).wrapping_add_signed(-1));
                queue.push_back(Beam { position: new_beam_pos, direction: (0, -1) });
            },
            ('\\', (0, -1)) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(-1), (beam.position.1).wrapping_add_signed(0));
                queue.push_back(Beam { position: new_beam_pos, direction: (-1, 0) });
            },
            ('-', (1, 0)) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(1), (beam.position.1).wrapping_add_signed(0));
                queue.push_back(Beam { position: new_beam_pos, direction: (1, 0) });
            },
            ('-', (-1, 0)) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(-1), (beam.position.1).wrapping_add_signed(0));
                queue.push_back(Beam { position: new_beam_pos, direction: (-1, 0) });
            },
            ('|', (0, 1)) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(0), (beam.position.1).wrapping_add_signed(1));
                queue.push_back(Beam { position: new_beam_pos, direction: (0, 1) });
            },
            ('|', (0, -1)) => {
                let new_beam_pos = ((beam.position.0).wrapping_add_signed(0), (beam.position.1).wrapping_add_signed(-1));
                queue.push_back(Beam { position: new_beam_pos, direction: (0, -1) });
            },
            ('|', _) => {
                let mut new_beam_pos = ((beam.position.0).wrapping_add_signed(0), (beam.position.1).wrapping_add_signed(-1));
                queue.push_back(Beam { position: new_beam_pos, direction: (0, -1) });
                new_beam_pos = ((beam.position.0).wrapping_add_signed(0), (beam.position.1).wrapping_add_signed(1));
                queue.push_back(Beam { position: new_beam_pos, direction: (0, 1) });
            },
            ('-', _) => {
                let mut new_beam_pos = ((beam.position.0).wrapping_add_signed(1), (beam.position.1).wrapping_add_signed(0));
                queue.push_back(Beam { position: new_beam_pos, direction: (1, 0) });
                new_beam_pos = ((beam.position.0).wrapping_add_signed(-1), (beam.position.1).wrapping_add_signed(0));
                queue.push_back(Beam { position: new_beam_pos, direction: (-1, 0) });
            }
            // anything else shouldn't happen, so panic and print out the character and direction
            (c, d) => {
                panic!("invalid character {} with direction {:?}", c, d);
            }
        }
    }
    
    // return the number of visited positions
    visited_positions.len() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().collect::<Vec<&str>>();
    
    let start = Beam { position: (0, 0), direction: (1, 0) };

    Some(num_touched_tiles(&grid, start))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.lines().collect::<Vec<&str>>();
    
    // find the maximum number of touched tiles by checking each starting position along each edge of the grid

    // start with the top edge
    let top_max = (*grid.first().unwrap()).par_char_indices().map(|(i, _)| {
        let start = Beam { position: (i, 0), direction: (0, 1) };
        num_touched_tiles(&grid, start)
    });
    
    // bottom edge
    let bottom_max = (*grid.last().unwrap()).par_char_indices().map(|(i, _)| {
        let start = Beam { position: (i, grid.len() - 1), direction: (0, -1) };
        num_touched_tiles(&grid, start)
    });
    
    // left edge
    let left_max = grid.par_iter().enumerate().map(|(i, _)| {
        let start = Beam { position: (0, i), direction: (1, 0) };
        num_touched_tiles(&grid, start)
    });

    // right edge
    let right_max = grid.par_iter().enumerate().map(|(i, _)| {
        let start = Beam { position: (grid[0].len() - 1, i), direction: (-1, 0) };
        num_touched_tiles(&grid, start)
    });

    // max of all edges is iterators chained together and maxed
    Some(top_max.chain(bottom_max).chain(left_max).chain(right_max).max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
