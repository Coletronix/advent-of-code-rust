use std::{cmp::Ordering, collections::{HashSet, BinaryHeap, HashMap}};

advent_of_code::solution!(17);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    position: Point,
    sum: u32,
    last_three_moves: [Option<Direction>; 3],
}

// Custom ordering for the priority queue
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.sum.cmp(&self.sum) // Min-heap
            .then_with(|| self.position.x.cmp(&other.position.x))
            .then_with(|| self.position.y.cmp(&other.position.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstras(grid: &[&str], start: Point, target: Point) -> Option<u32> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Point, (Point, u32)> = HashMap::new();

    open_set.push(State {
        position: start,
        sum: grid[start.y].chars().nth(start.x).unwrap().to_digit(10).unwrap() as u32,
        last_three_moves: [None, None, None],
    });

    while let Some(current_state) = open_set.pop() {
        if current_state.position == target {
            return Some(current_state.sum);
        }

        let valid_directions = get_valid_directions(&current_state.last_three_moves);

        for direction in &valid_directions {
            if is_valid_move(direction, &current_state.last_three_moves) {
                if let Some(new_position) = move_to(&current_state.position, *direction, rows, cols) {
                    let new_sum = current_state.sum + grid[new_position.y].chars().nth(new_position.x).unwrap().to_digit(10).unwrap() as u32;

                    if !came_from.contains_key(&new_position) || new_sum < came_from[&new_position].1 {
                        open_set.push(State {
                            position: new_position,
                            sum: new_sum,
                            last_three_moves: [Some(*direction), current_state.last_three_moves[0], current_state.last_three_moves[1]],
                        });
                        came_from.insert(new_position, (current_state.position, new_sum));
                    }
                }
            }
        }
    }

    None
}

fn a_star(grid: &[&str], start: Point, target: Point) -> Option<u32> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Point, (Point, u32)> = HashMap::new();

    open_set.push(State {
        position: start,
        sum: 0, // first one doesn't count per AOC instructions
        last_three_moves: [None, None, None],
    });
    
    // debug visited grid
    let mut visited_grid = grid.iter().map(|line| line.to_string()).collect::<Vec<String>>();

    while let Some(current_state) = open_set.pop() {
        // println!("Current state: {:?}", current_state);
       
        // add state position to debug visited grid
        // character added depends on previous direction
        let character = match current_state.last_three_moves[0] {
            Some(Direction::Up) => 'v',
            Some(Direction::Down) => '^',
            Some(Direction::Left) => '>',
            Some(Direction::Right) => '<',
            None => 'X',
        };
        visited_grid[current_state.position.y].replace_range(current_state.position.x..current_state.position.x + 1, &character.to_string());

        let visited_grid_slices = visited_grid.iter().map(|line| line.as_str()).collect::<Vec<&str>>();
        // print_grid(&visited_grid_slices);
        
        // std::thread::sleep(std::time::Duration::from_millis(2000));
        
        if current_state.position == target {
            // traverse came_from to get path, and modify visited_grid for debug
            let mut current_position = target;
            while let Some((previous_position, _)) = came_from.get(&current_position) {
                visited_grid[current_position.y].replace_range(current_position.x..current_position.x + 1, "#");
                current_position = *previous_position;
            }
            
            let visited_grid_slices = visited_grid.iter().map(|line| line.as_str()).collect::<Vec<&str>>();
            print_grid(&visited_grid_slices);

            return Some(current_state.sum);
        }
        
        let valid_directions = get_valid_directions(&current_state.last_three_moves);
        // println!("Valid directions: {:?} for {:?}", valid_directions, current_state.position);

        for direction in &valid_directions {
            if is_valid_move(direction, &current_state.last_three_moves) {
                if let Some(new_position) = move_to(&current_state.position, *direction, rows, cols) {
                    let new_sum = current_state.sum + grid[new_position.y].chars().nth(new_position.x).unwrap().to_digit(10).unwrap() as u32;

                    if !came_from.contains_key(&new_position) || new_sum < came_from[&new_position].1 {
                        open_set.push(State {
                            position: new_position,
                            sum: new_sum,
                            last_three_moves: [Some(*direction), current_state.last_three_moves[0], current_state.last_three_moves[1]],
                        });
                        came_from.insert(new_position, (current_state.position, new_sum));
                    }
                }
            }
        }
    }

    None
}

fn print_grid(grid: &[&str]) {
    for line in grid {
        println!("{}", line);
    }
    println!();
}

fn get_valid_directions(last_three_moves: &[Option<Direction>; 3]) -> Vec<Direction> {
    if let Some(current_direction) = last_three_moves[0] {
        return match current_direction {
            Direction::Up => vec![Direction::Left, Direction::Right, Direction::Up],
            Direction::Down => vec![Direction::Left, Direction::Right, Direction::Down],
            Direction::Left => vec![Direction::Up, Direction::Down, Direction::Left],
            Direction::Right => vec![Direction::Up, Direction::Down, Direction::Right],
        }
    } 
    // every direction
    vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]
}

fn is_valid_move(new_direction: &Direction, last_three_moves: &[Option<Direction>; 3]) -> bool {
    // cannot have last three moves all be the same direction
    match (last_three_moves[0], last_three_moves[1], last_three_moves[2]) {
        (Some(dir1), Some(dir2), Some(dir3)) => dir1 != dir2 || dir2 != dir3 || dir1 != dir3 || dir1 != *new_direction,
        _ => true,
    }
}

fn move_to(position: &Point, direction: Direction, rows: usize, cols: usize) -> Option<Point> {
    let (new_x, new_y) = match direction {
        Direction::Up if position.y > 0 => (position.x, position.y - 1),
        Direction::Down if position.y < rows - 1 => (position.x, position.y + 1),
        Direction::Left if position.x > 0 => (position.x - 1, position.y),
        Direction::Right if position.x < cols - 1 => (position.x + 1, position.y),
        _ => return None,
    };
    Some(Point { x: new_x, y: new_y })
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().collect::<Vec<&str>>();
    
    let start = Point { x: 0, y: 0 };
    let target = Point { x: grid.len() - 1, y: grid[0].len() - 1 };
    // let min_sum = dijkstras(&grid, start, target);
    let min_sum = a_star(&grid, start, target);
    println!("Min sum: {:?}", min_sum);
    
    min_sum
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
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
