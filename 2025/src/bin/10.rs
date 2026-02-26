use std::{collections::HashMap, str::FromStr, u32, u64};

advent_of_code::solution!(10);

#[derive(Debug)]
struct Input {
    target_bits: u32,
    button_bits: Vec<u32>,
    button_matrix: Vec<Vec<bool>>,
    joltages: Vec<i64>,
}

impl Input {
    fn new(target: Vec<bool>, button_wirings: Vec<Vec<usize>>, joltages: Vec<i64>) -> Self {
        let target_bits = target
            .iter()
            .enumerate()
            .fold(0, |acc, (n, s)| acc | if *s { 1 << n } else { 0 });

        let button_matrix = button_wirings
            .iter()
            .map(|button| {
                let mut row = vec![false; target.len()];

                button.iter().for_each(|&idx| row[idx] = true);

                row
            })
            .collect();

        let button_bits = button_wirings
            .into_iter()
            .map(|button| button.into_iter().fold(0, |acc, x| acc | (1 << x)))
            .collect();

        Self {
            target_bits,
            button_bits,
            button_matrix,
            joltages,
        }
    }

    fn min_button_presses(&self, state: u32, index: usize, count: u64, best: &mut u64) {
        if count >= *best {
            return;
        } else if state == self.target_bits {
            *best = count;
        } else if index < self.button_bits.len() {
            self.min_button_presses(state ^ self.button_bits[index], index + 1, count + 1, best);
            self.min_button_presses(state, index + 1, count, best);
        }
    }

    fn min_lever_switches(&self, target: Vec<i64>, memory: &mut HashMap<Vec<i64>, i64>) -> i64 {
        if target.iter().all(|t| *t == 0) {
            return 0;
        }

        if let Some(&value) = memory.get(&target) {
            return value;
        }

        let button_len = self.button_matrix.len();
        let target_len = target.len();

        let limit = 1 << button_len;
        let mut best = -1;

        for mask in 0..limit {
            let mut remainder = target.clone();
            let mut cost_phase1 = 0;
            let mut poss = true;

            for b in 0..button_len {
                if mask & (1 << b) != 0 {
                    cost_phase1 += 1;
                    for i in 0..target_len {
                        remainder[i] -= self.button_matrix[b][i] as i64;
                    }
                }
            }

            for i in 0..target_len {
                if remainder[i] < 0 || remainder[i] % 2 != 0 {
                    poss = false;
                    break;
                }
            }

            if poss {
                let next_target = remainder.into_iter().map(|r| r / 2).collect();
                let res = self.min_lever_switches(next_target, memory);
                if res != -1 {
                    let total_cost = cost_phase1 + 2 * res;
                    if best == -1 || total_cost < best {
                        best = total_cost;
                    }
                }
            }
        }

        memory.insert(target, best);
        best
    }
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let target = parts
            .next()
            .unwrap()
            .trim_matches(['[', ']'])
            .chars()
            .map(|c| c == '#')
            .collect();

        let part_count = parts.clone().count();

        let mut button_wirings = Vec::new();
        let mut i = 0;
        while i < part_count - 1
            && let Some(part) = parts.next()
        {
            let part = part.trim_matches(['(', ')']);
            button_wirings.push(part.split(',').map(|b| b.parse().unwrap()).collect());
            i += 1;
        }

        let joltages = parts
            .next()
            .unwrap()
            .trim_matches(['{', '}'])
            .split(',')
            .map(|b| b.parse().unwrap())
            .collect();

        Ok(Input::new(target, button_wirings, joltages))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let inputs = input
        .lines()
        .map(Input::from_str)
        .collect::<Result<Vec<_>, ()>>()
        .unwrap();

    let result = inputs
        .into_iter()
        .map(|i| {
            let mut best = u64::MAX;
            i.min_button_presses(0, 0, 0, &mut best);
            // println!("best {best:?}");
            best
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let inputs = input
        .lines()
        .map(Input::from_str)
        .collect::<Result<Vec<_>, ()>>()
        .unwrap();

    let result = inputs
        .into_iter()
        .map(|i| {
            let mut memory = HashMap::new();
            i.min_lever_switches(i.joltages.clone(), &mut memory) as u64
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
