use primitive_types::U256;

advent_of_code::solution!(11);

#[derive(Debug)]
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
    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let mut dist = (galaxies[i].x as i32 - galaxies[j].x as i32).abs()
                + (galaxies[i].y as i32 - galaxies[j].y as i32).abs();

            dist += (compute_mask_range(
                std::cmp::min(galaxies[i].y, galaxies[j].y),
                std::cmp::max(galaxies[i].y, galaxies[j].y),
            ) & occupied_rows)
                .count_ones() as i32;
            dist += (compute_mask_range(
                std::cmp::min(galaxies[i].x, galaxies[j].x),
                std::cmp::max(galaxies[i].x, galaxies[j].x),
            ) & occupied_cols)
                .count_ones() as i32;

            total_distance += dist as u32;
        }
    }

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u64> {
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
    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let mut dist = (galaxies[i].x as i64 - galaxies[j].x as i64).abs()
                + (galaxies[i].y as i64 - galaxies[j].y as i64).abs();

            dist += ((compute_mask_range(
                std::cmp::min(galaxies[i].y, galaxies[j].y),
                std::cmp::max(galaxies[i].y, galaxies[j].y),
            ) & occupied_rows)
                .count_ones() as i64)
                * 999999;
            dist += ((compute_mask_range(
                std::cmp::min(galaxies[i].x, galaxies[j].x),
                std::cmp::max(galaxies[i].x, galaxies[j].x),
            ) & occupied_cols)
                .count_ones() as i64)
                * 999999;

            total_distance += dist as u64;
        }
    }

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
        assert_eq!(result, None);
    }
}
