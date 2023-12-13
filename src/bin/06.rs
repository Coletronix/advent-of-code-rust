advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    // parse out time and distance values from the following form:
    // Time:      7  15   30
    // Distance:  9  40  200
    let times = lines
        .first()?
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse::<f32>().unwrap());
    let distances = lines
        .last()?
        .split_whitespace()
        .skip(1)
        .map(|d| d.parse::<f32>().unwrap());

    let mut num_ways_to_win = 1;

    for (time, distance) in times.zip(distances) {
        // find two points where y=distance intersects the curve y=-x^2 + (time)x
        // quadratic formula time!
        let a = -1.0;
        let b = time;
        let c = -distance;
        let discriminant = b * b - 4.0 * a * c;

        assert!(discriminant >= 0.0);

        let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
        let ways_to_win = x2.ceil() as u32 - x1.floor() as u32 - 1;
        num_ways_to_win *= ways_to_win;
    }

    Some(num_ways_to_win)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    // parse out time and distance values from the following form, concatinating all the numbers on each line into one number:
    // Time:      7  15   30
    // Distance:  9  40  200
    let time = lines
        .first()?
        .split_whitespace()
        .skip(1)
        .fold("".into(), |a, b| format!("{}{}", a, b))
        .parse::<f64>()
        .unwrap();
    let distance = lines
        .last()?
        .split_whitespace()
        .skip(1)
        .fold("".into(), |a, b| format!("{}{}", a, b))
        .parse::<f64>()
        .unwrap();

    let mut num_ways_to_win = 1;

    // find two points where y=distance intersects the curve y=-x^2 + (time)x
    // quadratic formula time!
    let a = -1.0;
    let b = time;
    let c = -distance;
    let discriminant = b * b - 4.0 * a * c;
    
    assert!(discriminant >= 0.0);

    let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
    let ways_to_win = x2.ceil() as u32 - x1.floor() as u32 - 1;
    num_ways_to_win *= ways_to_win;

    Some(num_ways_to_win)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
