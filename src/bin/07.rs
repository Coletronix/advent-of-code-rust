advent_of_code::solution!(7);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let card_strengths = [
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<char, u32>>();

    // each line contains a poker hand and a bid, seperated by a space
    let hand_bids = lines.iter().map(|line| {
        let mut parts = line.split_whitespace();
        let hand = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<u32>().unwrap();
        (hand, bid)
    });

    let hand_types = hand_bids.map(|(hand, bid)| {
        // Work out hand type (five of a kind, four of a kind, full house, etc).
        let mut counts = HashMap::new();
        for c in hand.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut counts: Vec<_> = counts.values().collect();
        counts.sort();

        let hand_type = match counts.as_slice() {
            [5] => 6, // Five of a kind
            [1, 4] => 5, // Four of a kind
            [2, 3] => 4, // Full house
            [1, 1, 3] => 3, // Three of a kind
            [1, 2, 2] => 2, // Two pair
            [1, 1, 1, 2] => 1, // One pair
            [1, 1, 1, 1, 1] => 0, // High card
            _ => 0, // unreachable
        };

        (hand, bid, hand_type)
    });

    let mut hand_strengths = hand_types.collect::<Vec<(&str, u32, u32)>>();
    hand_strengths.sort_by(|(hand1, _, hand_type1), (hand2, _, hand_type2)| {
        // sort by hand type
        if hand_type1 > hand_type2 {
            return std::cmp::Ordering::Greater;
        } else if hand_type1 < hand_type2 {
            return std::cmp::Ordering::Less;
        }
        // then sort by card strength
        for (c1, c2) in hand1.chars().zip(hand2.chars()) {
            let s1 = card_strengths.get(&c1).unwrap();
            let s2 = card_strengths.get(&c2).unwrap();
            if s1 > s2 {
                return std::cmp::Ordering::Greater;
            } else if s1 < s2 {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Equal
    });

    Some(
        hand_strengths
            .iter()
            .enumerate()
            .map(|(i, (_, bid, _))| bid * (i + 1) as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let card_strengths = [
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<char, u32>>();

    // each line contains a poker hand and a bid, seperated by a space
    let hand_bids = lines.iter().map(|line| {
        let mut parts = line.split_whitespace();
        let hand = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<u32>().unwrap();
        (hand, bid)
    });

    let hand_types = hand_bids.map(|(hand, bid)| {
        // Work out hand type (five of a kind, four of a kind, full house, etc).
        let mut counts = HashMap::new();
        for c in hand.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut counts: Vec<_> = counts.values().collect();
        counts.sort();

        let hand_type = match counts.as_slice() {
            [5] => 6, // Five of a kind
            [1, 4] => 5, // Four of a kind
            [2, 3] => 4, // Full house
            [1, 1, 3] => 3, // Three of a kind
            [1, 2, 2] => 2, // Two pair
            [1, 1, 1, 2] => 1, // One pair
            [1, 1, 1, 1, 1] => 0, // High card
            _ => 0, // unreachable
        };

        (hand, bid, hand_type)
    });

    let mut hand_strengths = hand_types.collect::<Vec<(&str, u32, u32)>>();
    hand_strengths.sort_by(|(hand1, _, hand_type1), (hand2, _, hand_type2)| {
        // sort by hand type
        if hand_type1 > hand_type2 {
            return std::cmp::Ordering::Greater;
        } else if hand_type1 < hand_type2 {
            return std::cmp::Ordering::Less;
        }
        // then sort by card strength
        for (c1, c2) in hand1.chars().zip(hand2.chars()) {
            let s1 = card_strengths.get(&c1).unwrap();
            let s2 = card_strengths.get(&c2).unwrap();
            if s1 > s2 {
                return std::cmp::Ordering::Greater;
            } else if s1 < s2 {
                return std::cmp::Ordering::Less;
            }
        }
        std::cmp::Ordering::Equal
    });

    Some(
        hand_strengths
            .iter()
            .enumerate()
            .map(|(i, (_, bid, _))| bid * (i + 1) as u32)
            .sum(),
    )

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_one_edge_cases() {
        let input = "9KKK7 890\nJAA48 80\n755Q5 520\n4JQQJ 223\n997T9 405\n8A888 575\nK369T 730\nA5565 847";
        let result = part_one(input);
        assert_eq!(result, Some(21533));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
