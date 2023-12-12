advent_of_code::solution!(2);

enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}


pub fn part_one(input: &str) -> Option<u32> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    
    let lines = input.lines().collect::<Vec<&str>>();

    let valid_ids = lines.iter().filter_map(|line| {
        let mut parts = line.split(':');
        let game_id = parts.next().unwrap().trim().split(' ').last().unwrap().parse::<u32>().unwrap();
        let games_raw = parts.next().unwrap().trim().split(';');
        let num_raw_games = games_raw.clone().count();
        let valid_games = games_raw.filter_map(|game| {
            let cubes = game.trim().split(',').map(|cube| {
                let mut parts = cube.trim().split(' ');
                let count = parts.next().unwrap().parse::<u32>().unwrap();
                let color = parts.next().unwrap();
                match color {
                    "red" => Cube::Red(count),
                    "green" => Cube::Green(count),
                    "blue" => Cube::Blue(count),
                    _ => panic!("invalid color"),
                }
            });
            // sum up the red green and blue into 3 variables
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for cube in cubes {
                match cube {
                    Cube::Red(count) => red += count,
                    Cube::Green(count) => green += count,
                    Cube::Blue(count) => blue += count,
                }
            }
            if red <= max_red && green <= max_green && blue <= max_blue {
                Some(())
            } else {
                None
            }
        });
        if valid_games.count() == num_raw_games {
            Some(game_id)
        } else {
            None
        }
    });
    Some(valid_ids.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    let valid_powers = lines.iter().map(|line| {
        let mut parts = line.split(':');
        let _game_id = parts.next();
        let games_raw = parts.next().unwrap().trim().split(';');
        let valid_games = games_raw.map(|game| {
            game.trim().split(',').map(|cube| {
                let mut parts = cube.trim().split(' ');
                let count = parts.next().unwrap().parse::<u32>().unwrap();
                let color = parts.next().unwrap();
                (count, color)
            }).fold((0, 0, 0), |(red, green, blue), (count, color)| {
                match color {
                    "red" => (red + count, green, blue),
                    "green" => (red, green + count, blue),
                    "blue" => (red, green, blue + count),
                    _ => panic!("invalid color"),
                }
            })
        });
        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);

        for (red, green, blue) in valid_games {
            if red > max_red {
                max_red = red;
            }
            if green > max_green {
                max_green = green;
            }
            if blue > max_blue {
                max_blue = blue;
            }
        }
        let power = max_red * max_green * max_blue;
        power
    });
    Some(valid_powers.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let file_in = &advent_of_code::template::read_file("examples", DAY);
        let result = part_one(file_in);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let file_in = &advent_of_code::template::read_file("examples", DAY);
        let result = part_two(file_in);
        assert_eq!(result, Some(2286));
    }
}
