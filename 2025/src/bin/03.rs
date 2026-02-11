advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|bank| {
                let highest_value = bank[..bank.len() - 1].iter().max().unwrap();
                let idx = bank.iter().position(|b| b == highest_value).unwrap();

                let second_digit = bank[(idx + 1)..].iter().max().unwrap();

                (highest_value * 10 + second_digit) as u64
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    const DIGITS: usize = 12;
    Some(
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .map(|bank| {
                let mut output = vec![];

                let mut bank = &bank[..];
                for i in (0..DIGITS).rev() {
                    let remaining_digits = i;
                    let highest_value = bank[..bank.len() - remaining_digits].iter().max().unwrap();
                    let idx = bank.iter().position(|b| b == highest_value).unwrap();
                    output.push(*highest_value);

                    bank = &bank[idx + 1..];
                }

                u64::from_str_radix(&output.into_iter().collect::<String>(), 10).unwrap()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
