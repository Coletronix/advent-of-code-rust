advent_of_code::solution!(5);

use rayon::prelude::*;
use std::sync::{Arc, Mutex};

struct ThingMap {
    map_ranges: Vec<(u32, u32, u32)>, // destination start, source start, length
}

impl ThingMap {
    fn map(&self, source: u32) -> u32 {
        for (dest_start, source_start, length) in &self.map_ranges {
            if source >= *source_start && source < *source_start + *length {
                return dest_start + (source - source_start);
            }
        }
        source
    }

    fn rev_map(&self, dest: u32) -> u32 {
        for (dest_start, source_start, length) in &self.map_ranges {
            if dest >= *dest_start && dest < *dest_start + *length {
                return source_start + (dest - dest_start);
            }
        }
        dest
    }
}

struct MapChain {
    map_chain: Vec<ThingMap>,
}

impl MapChain {
    fn new() -> Self {
        Self {
            map_chain: Vec::new()
        }
    }

    fn map(&self, source: u32) -> u32 {
        let mut result = source;
        for map in self.map_chain.iter() {
            result = map.map(result);
        }
        result
    }

    // fn rev_map(&self, dest: u32) -> u32 {
    //     for (dest_start, source_start, length) in &self.map_ranges {
    //         if dest >= *dest_start && dest < *dest_start + *length {
    //             return source_start + (dest - dest_start);
    //         }
    //     }
    //     dest
    // }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut lines_iter = lines.iter();
    
    // first line contains seeds and is formatted as follows:
    // seeds: 79 14 55 13
    let seeds = lines_iter.next().unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    
    // skip next line as it is empty
    lines_iter.next();
    
    // split into groups of lines seperated by empty lines
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for line in lines_iter {
        if line.is_empty() {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(line);
        }
    }
    // to account for end case, check if our current group is empty, and if not add it
    if !group.is_empty() {
        groups.push(group);
    }

    // parse groups into ThingMaps
    let map_chain = groups.iter().map(|group| {
        let mut group_lines_iter = group.iter();
        // first line contains the name of the map
        let _map_name_iter = group_lines_iter.next();
        // let map_from = map_name_iter.next().unwrap();
        // let map_to = map_name_iter.last().unwrap();
        
        // create ThingMap
        let mut map = ThingMap {
            map_ranges: Vec::new(),
        };

        // each remaining line contains a mapping
        for line in group_lines_iter {
            let mut line_iter = line.split_whitespace();
            let dest_start = line_iter.next().unwrap().parse::<u32>().unwrap();
            let source_start = line_iter.next().unwrap().parse::<u32>().unwrap();
            let length = line_iter.next().unwrap().parse::<u32>().unwrap();
            map.map_ranges.push((dest_start, source_start, length));
        }
        
        map
    }).collect::<Vec<ThingMap>>();
    
    // apply maps to seeds
    let results = seeds.iter().map(|seed| {
        let mut result = *seed;
        // println!("Starting with seed {}", result);
        for map in &map_chain {
            result = map.map(result);
            // println!("Mapped to {} by convert to {}", result, map.to)
        }
        result
    });
    Some(results.min().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut lines_iter = lines.iter();
    
    // first line contains seeds and is formatted as follows:
    // seeds: 79 14 55 13
    let seed_ranges = lines_iter.next().unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let seeds = seed_ranges.chunks(2).map(|chunk| {
        assert!(chunk.len() == 2);
        (chunk[0]..(chunk[0] + chunk[1])).collect::<Vec<u32>>()
    }).flatten().collect::<Vec<u32>>();
    
    // skip next line as it is always empty
    lines_iter.next();
    
    // split into groups of lines seperated by empty lines
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for line in lines_iter {
        if line.is_empty() {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(line);
        }
    }
    // to account for end case, check if our current group is empty, and if not add it
    if !group.is_empty() {
        groups.push(group);
    }

    // parse groups into ThingMaps
    let mut chain = MapChain::new();
    chain.map_chain = groups.iter().map(|group| {
        let mut group_lines_iter = group.iter();
        // first line contains the name of the map
        let _map_name_iter = group_lines_iter.next();
        
        // create ThingMap
        let mut map = ThingMap {
            map_ranges: Vec::new(),
        };

        // each remaining line contains a mapping
        for line in group_lines_iter {
            let mut line_iter = line.split_whitespace();
            let dest_start = line_iter.next().unwrap().parse::<u32>().unwrap();
            let source_start = line_iter.next().unwrap().parse::<u32>().unwrap();
            let length = line_iter.next().unwrap().parse::<u32>().unwrap();
            map.map_ranges.push((dest_start, source_start, length));
        }
        
        map
    }).collect::<Vec<ThingMap>>();
    
    // // apply maps to seeds
    // let results = seeds.iter().map(|seed| {
    //     let mut result = *seed;
    //     // println!("Starting with seed {}", result);
    //     for map in &map_chain {
    //         result = map.map(result);
    //         // println!("Mapped to {} by convert to {}", result, map.to)
    //     }
    //     result
    // });
    // Some(results.min().unwrap())
    let batch_size = seeds.len()/32; // adjust this based on your requirements and experimentation
    let seeds_arc = Arc::new(seeds);
    let results = Mutex::new(Vec::new());

    seeds_arc.par_chunks(batch_size).for_each(|batch| {
        let mut local_results = Vec::new();
        for seed in batch {
            local_results.push(chain.map(*seed));
        }
        let mut global_results = results.lock().unwrap();
        global_results.extend(local_results);
    });

    let min_result = results.lock().unwrap().iter().min().cloned().unwrap();
    Some(min_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
