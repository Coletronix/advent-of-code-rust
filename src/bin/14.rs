advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().collect::<Vec<&str>>();
    
    // for each row from bottom to top
    let mut rock_buildup = vec![0; grid[0].len()];
    let mut total_load = 0;
    for (dist_from_bottom, &line) in grid.iter().rev().enumerate() {
        for (i, c) in line.chars().enumerate() {
            match c {
                'O' => rock_buildup[i] += 1,
                '#' => {
                    if rock_buildup[i] > 0 {
                        let top_rock = dist_from_bottom;
                        let below_bottom_rock = dist_from_bottom - rock_buildup[i];
                        let load = top_rock * (top_rock+1)/2 - below_bottom_rock * (below_bottom_rock+1)/2;
                        total_load += load;
                        rock_buildup[i] = 0;
                    }
                },
                _ => (),
            };
        }
    }
    let dist_from_bottom = grid.len();
    for buildup in rock_buildup {
        if buildup > 0 {
            let top_rock = dist_from_bottom;
            let below_bottom_rock = dist_from_bottom - buildup;
            let load = top_rock * (top_rock+1)/2 - below_bottom_rock * (below_bottom_rock+1)/2;
            total_load += load;
        }
    }
    
    Some(total_load as u32)
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
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
