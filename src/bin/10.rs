use std::collections::HashMap;

advent_of_code::solution!(10);

// direction constants
const TO_RIGHT: (i32, i32) = (1, 0);
const TO_LEFT: (i32, i32) = (-1, 0);
const TO_UP: (i32, i32) = (0, -1);
const TO_DOWN: (i32, i32) = (0, 1);
const START: (i32, i32) = (0, 0);

const FROM_LEFT: (i32, i32) = (-1, 0);
const FROM_RIGHT: (i32, i32) = (1, 0);
const FROM_DOWN: (i32, i32) = (0, 1);
const FROM_UP: (i32, i32) = (0, -1);

fn find_char(grid: &[&str], c: char) -> Option<(i32, i32)> {
    for (i, &row) in grid.iter().enumerate() {
        for (j, item) in row.chars().enumerate() {
            if item == c {
                return Some((j as i32, i as i32));
            }
        }
    }
    None
}

fn get_char_ascii_grid(grid: &[&str], pos: (i32, i32)) -> Option<char> {
    grid.get(pos.1 as usize)?
        .as_bytes()
        .get(pos.0 as usize)
        .map(|&c| c as char)
}

#[derive(Debug)]
struct Traverser {
    pos: (i32, i32),
    to_dir: (i32, i32),
}

impl Traverser {
    fn move_it(&mut self, grid: &[&str], direction_map: &HashMap<(char, (i32, i32)), (i32, i32)>) {
        self.to_dir = *direction_map
            .get(&(
                get_char_ascii_grid(grid, self.pos).unwrap(),
                invert_to_from(&self.to_dir),
            ))
            .expect("There's a bug. You somehow got to a character that isn't a pipe");
        self.pos.0 += self.to_dir.0;
        self.pos.1 += self.to_dir.1;
    }
}

fn invert_to_from(dir: &(i32, i32)) -> (i32, i32) {
    (-dir.0, -dir.1)
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    // map of what direction you should check next when coming into a tile from a certain direction
    let direction_map: HashMap<(char, (i32, i32)), (i32, i32)> = HashMap::from([
        (('-', FROM_LEFT), TO_RIGHT),
        (('-', FROM_RIGHT), TO_LEFT),
        (('|', FROM_UP), TO_DOWN),
        (('|', FROM_DOWN), TO_UP),
        (('L', FROM_UP), TO_RIGHT),
        (('L', FROM_RIGHT), TO_UP),
        (('J', FROM_UP), TO_LEFT),
        (('J', FROM_LEFT), TO_UP),
        (('7', FROM_LEFT), TO_DOWN),
        (('7', FROM_DOWN), TO_LEFT),
        (('F', FROM_RIGHT), TO_DOWN),
        (('F', FROM_DOWN), TO_RIGHT),
        // entering the starting position from any direction should signify the end of traversal
        (('S', FROM_LEFT), START),
        (('S', FROM_RIGHT), START),
        (('S', FROM_UP), START),
        (('S', FROM_DOWN), START),
    ]);

    // start by finding the position of the S
    let mut start_pos = find_char(lines.as_slice(), 'S').unwrap();

    let mut start_dir = TO_LEFT; // chosen arbitrarily

    // TODO: this could be made more efficient
    // check up
    if let Some(c) = get_char_ascii_grid(lines.as_slice(), (start_pos.0, start_pos.1 - 1)) {
        if direction_map.contains_key(&(c, FROM_DOWN)) {
            start_dir = TO_UP;
        }
    }
    // check down
    if let Some(c) = get_char_ascii_grid(lines.as_slice(), (start_pos.0, start_pos.1 + 1)) {
        if direction_map.contains_key(&(c, FROM_UP)) {
            start_dir = TO_DOWN;
        }
    }
    // check right
    if let Some(c) = get_char_ascii_grid(lines.as_slice(), (start_pos.0 + 1, start_pos.1)) {
        if direction_map.contains_key(&(c, FROM_LEFT)) {
            start_dir = TO_RIGHT;
        }
    }

    start_pos.0 += start_dir.0;
    start_pos.1 += start_dir.1;

    let mut traverser = Traverser {
        pos: start_pos,
        to_dir: start_dir,
    };

    let mut path_length = 0;
    while traverser.to_dir != START {
        traverser.move_it(lines.as_slice(), &direction_map);
        path_length += 1;
    }

    Some(path_length / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();

    // map of what direction you should check next when coming into a tile from a certain direction
    let direction_map: HashMap<(char, (i32, i32)), (i32, i32)> = HashMap::from([
        (('-', FROM_LEFT), TO_RIGHT),
        (('-', FROM_RIGHT), TO_LEFT),
        (('|', FROM_UP), TO_DOWN),
        (('|', FROM_DOWN), TO_UP),
        (('L', FROM_UP), TO_RIGHT),
        (('L', FROM_RIGHT), TO_UP),
        (('J', FROM_UP), TO_LEFT),
        (('J', FROM_LEFT), TO_UP),
        (('7', FROM_LEFT), TO_DOWN),
        (('7', FROM_DOWN), TO_LEFT),
        (('F', FROM_RIGHT), TO_DOWN),
        (('F', FROM_DOWN), TO_RIGHT),
        // entering the starting position from any direction should signify the end of traversal
        (('S', FROM_LEFT), START),
        (('S', FROM_RIGHT), START),
        (('S', FROM_UP), START),
        (('S', FROM_DOWN), START),
    ]);

    // start by finding the position of the S
    let mut start_pos = find_char(lines.as_slice(), 'S').unwrap();

    let mut start_dir = TO_LEFT; // chosen arbitrarily

    // TODO: this could be made more efficient
    // check up
    if let Some(c) = get_char_ascii_grid(lines.as_slice(), (start_pos.0, start_pos.1 - 1)) {
        if direction_map.contains_key(&(c, FROM_DOWN)) {
            start_dir = TO_UP;
        }
    }
    // check down
    if let Some(c) = get_char_ascii_grid(lines.as_slice(), (start_pos.0, start_pos.1 + 1)) {
        if direction_map.contains_key(&(c, FROM_UP)) {
            start_dir = TO_DOWN;
        }
    }
    // check right
    if let Some(c) = get_char_ascii_grid(lines.as_slice(), (start_pos.0 + 1, start_pos.1)) {
        if direction_map.contains_key(&(c, FROM_LEFT)) {
            start_dir = TO_RIGHT;
        }
    }

    start_pos.0 += start_dir.0;
    start_pos.1 += start_dir.1;

    let mut traverser = Traverser {
        pos: start_pos,
        to_dir: start_dir,
    };

    let mut path_length = 0;
    while traverser.to_dir != START {
        traverser.move_it(lines.as_slice(), &direction_map);
        path_length += 1;
    }

    // count the number of dots that have an even number of intersections with a path to the left
    let mut num_enclosed = 0;
    for line in lines {
        let mut inside_loop = false;
        for c in line.chars() {
            if c == '.' {
                if inside_loop {
                    num_enclosed += 1;
                    print!("*");
                } else {
                    print!("^");
                }
            } else if c != '-' {
                inside_loop = !inside_loop;
                print!(".");
            } else {
                print!(".");
            }
        }
        println!();
    }
    Some(num_enclosed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_example_one() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 21));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_example_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 22));
        assert_eq!(result, Some(8));
    }
}
