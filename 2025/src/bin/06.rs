use std::{iter::zip, ops::Not, str::FromStr};

advent_of_code::solution!(6);

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Mul),
            _ => Err(()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut rows = input
        .trim()
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>());
    let operators: Vec<Operation> = rows
        .next_back()
        .unwrap()
        .into_iter()
        .map(|op| op.parse().unwrap())
        .collect();
    let mut rows = rows.map(|row| {
        row.into_iter()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });

    let first_row = rows.next().unwrap();
    let total = rows
        .fold(first_row, |acc, row| {
            zip(zip(acc, row), &operators)
                .map(|((a, b), op)| match op {
                    Operation::Add => a + b,
                    Operation::Mul => a * b,
                })
                .collect()
        })
        .into_iter()
        .sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut total = 0;

    let mut numbers_group: Vec<u64> = vec![];
    while rows[0].is_empty().not() {
        let mut current_number = vec![];
        let mut op = None;
        for row in rows.iter_mut() {
            let Some(c) = row.pop() else { continue };
            if c.is_digit(10) {
                current_number.push(c);
            } else if c == '+' {
                op = Some(Operation::Add)
            } else if c == '*' {
                op = Some(Operation::Mul)
            }
        }
        if !current_number.is_empty() {
            numbers_group.push(
                current_number
                    .into_iter()
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            );
        }
        match op {
            Some(Operation::Add) => total += numbers_group.drain(..).sum::<u64>(),
            Some(Operation::Mul) => total += numbers_group.drain(..).product::<u64>(),
            None => (),
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
