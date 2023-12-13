advent_of_code::solution!(5);

use rayon::prelude::*;
use std::{sync::{Arc, Mutex}, collections::HashSet};
use std::iter::FromIterator;

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
    //     let mut result = dest;
    //     for map in self.map_chain.iter().rev() {
    //         result = map.rev_map(result);
    //     }
    //     result
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
    let seed_ranges = seed_ranges
        .chunks(2)
        .map(|chunk| {
            assert!(chunk.len() == 2);
            chunk[0]..(chunk[0] + chunk[1])
        });

    // let seeds = seed_ranges.chunks(2).map(|chunk| {
    //     assert!(chunk.len() == 2);
    //     (chunk[0]..(chunk[0] + chunk[1])).collect::<Vec<u32>>()
    // }).flatten().collect::<Vec<u32>>();

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

    // work out interesting points in the input seed range that cause the output to have a drastic change
    // by traversing backwards through the chain map
    let mut interesting_points = Vec::new();
    // start at the end of the chain
    for map in chain.map_chain.iter().rev() {
        // run everything currently in interesting_points backwards through this map
        for point in &mut interesting_points {
            println!("Reversing point: {} -> {}", *point, map.rev_map(*point));
            *point = map.rev_map(*point);
        }
        for map_range in map.map_ranges.iter() {
            //                   {        }
            // *     *        *               *        *
            // 0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29
            // 0  1  3  4  5  2  3  4  5  6  50 51 52 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26
            // if let Some(point) = map_range.1.checked_sub(1) {
            //     interesting_points.push(point);
            // }
            interesting_points.push(map_range.1); // source
            println!("Adding interesting point {}", map_range.1);
            // interesting_points.push(map_range.1 + map_range.2 - 1);
            // interesting_points.push(map_range.1 + map_range.2);
        }
    }
    // zero is always an interesting point
    interesting_points.push(0);
    interesting_points.sort();
    // remove duplicates
    interesting_points.dedup();

    // duplicate array, and sort it by minimum seed value
    let mut interesting_points_min = interesting_points.clone().iter().map(|point| {
        (*point, chain.map(*point))
    }).collect::<Vec<(u32, u32)>>();
    interesting_points_min.sort_by(|(_, location1), (_, location2)| {
        location1.partial_cmp(location2).unwrap()
    });

    println!("Interesting_points:");
    for point in &interesting_points {
        println!("  Interesting point {}", point);
    }
    println!("Interesting_points_min:");
    for (point, location) in &interesting_points_min {
        println!("  Interesting point ({}, {})", point, location);
    }

    // print out mappings from 0 to 100 for debug
    for i in 0..100 {
        print!("{: >3}", i);
    }
    println!();
    for i in 0..100 {
        print!("{: >3}", chain.map(i));
    }
    println!();
    for i in 0..100 {
        print!("{}", if interesting_points.contains(&i) {
            "  *"
        } else {
            "   "
        });
    }
    println!();

    // fancy stuff
    println!("Searching for minimum");
    for (point, location) in &interesting_points_min {
        println!("  Checking point {}", point);
        // check if one of the seed ranges contains our interesting point
        if seed_ranges.clone().any(|range| range.contains(point)) {
            println!("    Point was in range! Returning {}", location);
            // this must be the seed that produces the minimum, so we can just return it
            return Some(*location);
        }
        // check if there are any seed ranges starting between the current interesting point and the next
        let current_point_index = interesting_points.iter().position(|&x| x == *point).unwrap(); // guaranteed to be in this list
        let next_point = interesting_points[(current_point_index+1) as usize]; // TODO: this can panic
        let range_of_interest = *point..next_point;
        println!("    Range of interest: {:?}", range_of_interest);
        for seed_range in seed_ranges.clone() {
            if range_of_interest.contains(&seed_range.start) {
                println!("    Range of interest contained the start of a seed range! Start of seed range was {}. Returning {}", seed_range.start, chain.map(seed_range.start));
                // the start must map to the minimum then
                return Some(chain.map(seed_range.start))
            }
        }
    }

    // apply maps to seeds
    // let results = seeds.iter().map(|seed| {
    //     chain.map(*seed)
    // });
    // Some(results.min().unwrap())
    None
    // println!("Seeds len: {}", seeds.len());
    // let batch_size = (seeds.len()/32).max(1); // adjust this based on your requirements and experimentation
    // println!("batch size: {}", batch_size);
    // let seeds_arc = Arc::new(seeds);
    // let results = Mutex::new(Vec::new());

    // seeds_arc.par_chunks(batch_size).for_each(|batch| {
    //     let mut local_results = Vec::new();
    //     for seed in batch {
    //         local_results.push(chain.map(*seed));
    //     }
    //     let mut global_results = results.lock().unwrap();
    //     global_results.extend(local_results);
    // });

    // let min_result = results.lock().unwrap().iter().min().cloned().unwrap();
    // Some(min_result)
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
