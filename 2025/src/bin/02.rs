advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .split(',')
            .map(|range| range.split_once('-').expect("two parts in range"))
            .flat_map(|(start, end)| {
                u64::from_str_radix(start, 10).unwrap()..=u64::from_str_radix(end, 10).unwrap()
            })
            .filter(|number| {
                let str = number.to_string();
                let (left, right) = str.split_at(str.len() / 2);
                left == right
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .trim()
            .split(',')
            .map(|range| range.split_once('-').expect("two parts in range"))
            .flat_map(|(start, end)| {
                u64::from_str_radix(start, 10).unwrap()..=u64::from_str_radix(end, 10).unwrap()
            })
            .filter(|number| {
                let str: Vec<_> = number.to_string().chars().collect();
                let mut repeating = false;
                for len in 1..=str.len() / 2 {
                    if str.len() % len != 0 {
                        continue;
                    }
                    let mut chunks = str.chunks(len);
                    let first = chunks.next().unwrap();
                    if chunks.all(|c| c == first) {
                        repeating = true;
                        break;
                    }
                }
                repeating
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
