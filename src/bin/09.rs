use na::DMatrix;
use nalgebra::{self as na, OMatrix, OVector, U2};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let lines = input.lines().collect::<Vec<&str>>();

    let predicted_values = lines.iter().map(|line| {
        let datapoints = line
            .split(' ')
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut differences_vec = Vec::new();
        differences_vec.push(datapoints);

        while !differences_vec.last().unwrap().iter().all(|&elem| elem == 0) {
            let new_vec = differences_vec.last().unwrap().windows(2).map(|window| {
                window[1] - window[0]
            }).collect();
            differences_vec.push(new_vec);
        }

        // compute next element
        // push an extra 0 on the last one
        differences_vec.last_mut().unwrap().push(0);
        let mut rev_differences_iter = differences_vec.iter_mut().rev();
        let mut prev_diff = rev_differences_iter.next().unwrap();
        for differences in rev_differences_iter {
            differences.push(differences.last().unwrap() + prev_diff.last().unwrap());
            prev_diff = differences;
        }

        *differences_vec.first().unwrap().last().unwrap()
    });

    Some(predicted_values.sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let lines = input.lines().collect::<Vec<&str>>();

    let predicted_values = lines.iter().map(|line| {
        let datapoints = line
            .split(' ')
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut differences_vec = Vec::new();
        differences_vec.push(datapoints);

        while !differences_vec.last().unwrap().iter().all(|&elem| elem == 0) {
            let new_vec = differences_vec.last().unwrap().windows(2).map(|window| {
                window[1] - window[0]
            }).collect();
            differences_vec.push(new_vec);
        }

        // compute previous element
        // push an extra 0 on the last one
        let mut rev_differences_iter = differences_vec.iter_mut().rev();
        let mut prev_diff = rev_differences_iter.next().unwrap();
        for differences in rev_differences_iter {
            differences.push(differences.first().unwrap() - prev_diff.last().unwrap());
            prev_diff = differences;
        }

        *differences_vec.first().unwrap().last().unwrap()
    });

    Some(predicted_values.sum())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
