advent_of_code::solution!(18);

#[derive(Clone, Copy, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
    // color: Option<Color>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    
    let mut current_point = Point {
        x: 0,
        y: 0,
        // color: None,
    };
    
    let mut points = vec![current_point];
    
    for line in lines {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let length = parts.next().unwrap().parse::<u32>().unwrap();
        // not used for now
        let _color = parts.last().unwrap()[2..8].to_string();
        
        let direction = match direction {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("Unknown direction"),
        };

        for _ in 0..length {
            current_point.x += direction.0;
            current_point.y += direction.1;
            points.push(current_point);
        }
    }
    
    // find max and min x and y
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for point in &points {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }
    
    let mut grid = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    // go through each point and add it to the grid
    for point in &points {
        grid[(point.y - min_y) as usize][(point.x - min_x) as usize] = '#';
    }
    
    // print grid
    // for row in &grid {
    //     for col in row {
    //         print!("{}", col);
    //     }
    //     println!();
    // }
    
    // start point
    // TODO: not right, just works for the examples
    let x = grid[0].len() / 2;
    let y = grid.len() / 2;
    
    // flood fill
    let area = flood_fill(&mut grid, x, y);
    
    Some(area + points.len() as u32 - 1)
}

/// returns area of the flood fill
fn flood_fill(grid: &mut Vec<Vec<char>>, x: usize, y: usize) -> u32{
    let mut area = 0;
    if grid[y][x] == '.' {
        grid[y][x] = 'X';
        area += 1;
        if x > 0 {
            area += flood_fill(grid, x - 1, y);
        }
        if x < grid[y].len() - 1 {
            area += flood_fill(grid, x + 1, y);
        }
        if y > 0 {
            area += flood_fill(grid, x, y - 1);
        }
        if y < grid.len() - 1 {
            area += flood_fill(grid, x, y + 1);
        }
    }
    area
}

pub fn part_two(input: &str) -> Option<u64> {
    let _lines = input.lines().collect::<Vec<&str>>();
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
