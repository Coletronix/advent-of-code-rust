use primitive_types::U256;
use rayon::prelude::*;

advent_of_code::solution!(11);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

// count ones since U256 doesn't implement it
trait CountOnes {
    fn count_ones(&self) -> u32;
}

impl CountOnes for U256 {
    fn count_ones(&self) -> u32 {
        let mut count = 0;
        for part in self.0 {
            count += part.count_ones();
        }
        count
    }
}

fn compute_mask_range(start: usize, end: usize) -> U256 {
    let mask_end: U256 = (U256::from(1) << end) - 1;
    let mask_start: U256 = (U256::from(1) << start) - 1;
    mask_end ^ mask_start
}

fn min_max<T: Ord>(a: T, b: T) -> (T, T) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    // bitset representing which rows and columns are occupied
    let mut occupied_rows = (U256::from(1) << lines.len()) - 1;
    let mut occupied_cols = (U256::from(1) << lines[0].len()) - 1;

    let mut galaxies = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                occupied_rows &= !(U256::from(1) << y);
                occupied_cols &= !(U256::from(1) << x);

                galaxies.push(Position { x, y });
            }
        }
    }

    // compute the manhattan distance between every pair of galaxies
    let total_distance: u32 = (0..galaxies.len())
        .into_par_iter()
        .map(|i| {
            let mut dist_sum = 0;
            for j in i + 1..galaxies.len() {
                let (x_min, x_max) = min_max(galaxies[i].x, galaxies[j].x);
                let (y_min, y_max) = min_max(galaxies[i].y, galaxies[j].y);

                let mut dist = (x_max - x_min) as i32 + (y_max - y_min) as i32;

                dist +=
                    ((compute_mask_range(x_min, x_max) & occupied_cols).count_ones() as i32) * 1;
                dist +=
                    ((compute_mask_range(y_min, y_max) & occupied_rows).count_ones() as i32) * 1;

                dist_sum += dist as u32;
            }
            dist_sum
        })
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();

    println!("Line len: {}", lines[0].len());

    // bitset representing which rows and columns are occupied
    let mut occupied_rows = (U256::from(1) << lines.len()) - 1;
    let mut occupied_cols = (U256::from(1) << lines[0].len()) - 1;

    let mut galaxies = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                occupied_rows &= !(U256::from(1) << y);
                occupied_cols &= !(U256::from(1) << x);

                galaxies.push(Position { x, y });
            }
        }
    }

    // compute the manhattan distance between every pair of galaxies
    let total_distance: u64 = (0..galaxies.len())
        .into_par_iter()
        .map(|i| {
            let mut dist_sum = 0;
            for j in i + 1..galaxies.len() {
                let (x_min, x_max) = min_max(galaxies[i].x, galaxies[j].x);
                let (y_min, y_max) = min_max(galaxies[i].y, galaxies[j].y);

                let mut dist = (x_max - x_min) as i64 + (y_max - y_min) as i64;

                dist += ((compute_mask_range(x_min, x_max) & occupied_cols).count_ones() as i64)
                    * 999999;
                dist += ((compute_mask_range(y_min, y_max) & occupied_rows).count_ones() as i64)
                    * 999999;

                dist_sum += dist as u64;
            }
            dist_sum
        })
        .sum();

    Some(total_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
