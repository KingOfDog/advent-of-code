use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = grid
        .first()
        .unwrap()
        .iter()
        .position(|&c| c == 'S')
        .unwrap();

    let mut beam_heads = HashSet::new();
    beam_heads.insert(start);

    let mut splits = 0;

    for row in &grid[1..] {
        let mut new_heads = HashSet::new();
        for x in beam_heads.drain() {
            let c = row[x];
            if c == '^' {
                new_heads.insert(x - 1);
                new_heads.insert(x + 1);
                splits += 1;
            } else {
                new_heads.insert(x);
            }
        }
        beam_heads = new_heads;
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = grid
        .first()
        .unwrap()
        .iter()
        .position(|&c| c == 'S')
        .unwrap();

    let mut beam_heads = HashMap::new();
    beam_heads.insert(start, 1);

    for row in &grid[1..] {
        let mut new_heads = HashMap::new();
        for (x, count) in beam_heads.drain() {
            let c = row[x];
            if c == '^' {
                *new_heads.entry(x - 1).or_insert(0) += count;
                *new_heads.entry(x + 1).or_insert(0) += count;
            } else {
                *new_heads.entry(x).or_insert(0) += count;
            }
        }
        beam_heads = new_heads;
    }

    Some(beam_heads.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
