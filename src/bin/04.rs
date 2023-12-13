advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    
    let scratchcard_points = lines.iter().map(|line| {
        let mut parts = line.split(": ");
        let _card_ident = parts.next().unwrap();
        let mut numbers = parts.next().unwrap().split(" | ");
        let winning_numbers = numbers.next().unwrap().split(' ').filter(|n| !n.is_empty()).map(|n| n.trim().parse::<u32>().unwrap());
        let drawn_numbers = numbers.next().unwrap().split(' ').filter(|n| !n.is_empty()).map(|n| n.trim().parse::<u32>().unwrap());

        // construct a bitset of winning numbers and drawn numbers stored in a u128
        let mut winning_numbers_bitset = 0u128;
        let mut drawn_numbers_bitset = 0u128;
        for number in winning_numbers {
            winning_numbers_bitset |= 1 << number;
        }
        for number in drawn_numbers {
            drawn_numbers_bitset |= 1 << number;
        }
        
        let intersection = winning_numbers_bitset & drawn_numbers_bitset;
        let num_winners = intersection.count_ones();
        if num_winners == 0 {
            0
        } else {
           1 << (num_winners-1)
        }
    });
    
    Some(scratchcard_points.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    
    let mut num_card_copies = vec![0; lines.len() + 1];
    
    for line in lines.iter() {
        let mut parts = line.split(": ");
        let card_num = parts.next().unwrap().split(' ').last().unwrap().parse::<u32>().unwrap();
        let mut numbers = parts.next().unwrap().split(" | ");
        let winning_numbers = numbers.next().unwrap().split_whitespace().map(|n| n.trim().parse::<u32>().unwrap());
        let drawn_numbers = numbers.next().unwrap().split_whitespace().map(|n| n.trim().parse::<u32>().unwrap());

        // construct a bitset of winning numbers and drawn numbers stored in a u128
        let mut winning_numbers_bitset = 0u128;
        let mut drawn_numbers_bitset = 0u128;
        for number in winning_numbers {
            winning_numbers_bitset |= 1 << number;
        }
        for number in drawn_numbers {
            drawn_numbers_bitset |= 1 << number;
        }
        
        let intersection = winning_numbers_bitset & drawn_numbers_bitset;
        let num_winners = intersection.count_ones();
        
        for i in card_num..(card_num + num_winners) {
            num_card_copies[i as usize] += num_card_copies[card_num as usize - 1] + 1;
        }
    };
    
    // print out all numbers of card copies
    // for (i, num) in num_card_copies.iter().enumerate() {
    //     println!("{}: {}", i, num);
    // }
    
    Some(num_card_copies.iter().sum::<u32>() + lines.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
