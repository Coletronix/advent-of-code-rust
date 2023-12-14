use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    // first line contains directions
    let mut lr_provider = lines.next().unwrap().chars().cycle();

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    // build map graph
    for line in lines.skip(1) {
        let mut parts = line.split(" = ");
        let value = parts.next().unwrap();
        let edges = parts.next().unwrap().split(", ").map(|edge| {
            if edge.starts_with('(') {
                &edge[1..]
            } else if edge.ends_with(')') {
                &edge[0..edge.len() - 1]
            } else {
                edge
            }
        });
        let edges_vec = edges.collect::<Vec<&str>>();
        map.insert(value, edges_vec);
    }

    let mut num_steps = 0;
    let mut current_element = "AAA";
    // traverse graph until we get to the end (ZZZ)
    while let Some(direction) = lr_provider.next() {
        current_element = map.get(current_element).unwrap()[if direction == 'L' { 0 } else { 1 }];
        num_steps += 1;
        if current_element == "ZZZ" {
            break;
        }
    }
    Some(num_steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();

    // first line contains directions
    let mut lr_provider = lines.next().unwrap().chars().cycle();

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut starting_nodes = Vec::new();

    // build map graph, and find starting nodes
    for line in lines.skip(1) {
        let mut parts = line.split(" = ");
        let value = parts.next().unwrap();
        let edges = parts.next().unwrap().split(", ").map(|edge| {
            if edge.starts_with('(') {
                &edge[1..]
            } else if edge.ends_with(')') {
                &edge[0..edge.len() - 1]
            } else {
                edge
            }
        });

        let edges_vec = edges.collect::<Vec<&str>>();
        map.insert(value, edges_vec);

        if value.ends_with('A') {
            starting_nodes.push(value);
        }
    }

    let mut num_steps = 0;
    let mut current_elements = starting_nodes; // renaming
    println!("starting elements {:?}", current_elements);
    // traverse graph until we get to the end (all states end with Z)
    while let Some(direction) = lr_provider.next() {
        num_steps += 1;
        for current_element in current_elements.iter_mut() {
            *current_element =
                map.get(current_element).unwrap()[if direction == 'L' { 0 } else { 1 }];
        }
        println!("Computed new elements: {:?}", current_elements);
        if current_elements.iter().all(|elem| elem.ends_with('Z')) {
            break;
        }
    }
    Some(num_steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
