advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut pos = 50;
    let mut zeros_reached = 0;

    input
        .lines()
        .map(|line| (&line[..1] == "L", line[1..].parse::<i64>().unwrap()))
        .for_each(|(left_dir, amount)| {
            if left_dir {
                pos = (pos - amount).rem_euclid(100);
            } else {
                pos = (pos + amount).rem_euclid(100);
            }

            if pos == 0 {
                zeros_reached += 1;
            }
        });

    Some(zeros_reached)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pos = 50;
    let mut zeros_reached = 0;

    input
        .lines()
        .map(|line| (&line[..1] == "L", line[1..].parse::<i64>().unwrap()))
        .for_each(|(left_dir, amount)| {
            if left_dir {
                let new = pos - amount;
                if new.is_negative() {
                    zeros_reached += -new.div_euclid(100);
                    if pos == 0 {
                        zeros_reached -= 1;
                    }
                }
                pos = new.rem_euclid(100);
            } else {
                let new = pos + amount;
                if new > 99 {
                    zeros_reached += new.div_euclid(100);
                }
                pos = new.rem_euclid(100);
                if pos == 0 {
                    zeros_reached -= 1;
                }
            }

            if pos == 0 {
                zeros_reached += 1;
            }
        });

    Some(zeros_reached as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
