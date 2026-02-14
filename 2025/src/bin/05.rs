advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (fresh_ranges, available) = input.split_once("\n\n").unwrap();
    let fresh_ranges: Vec<_> = fresh_ranges
        .lines()
        .map(|range| range.split_once('-').unwrap())
        .map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse().unwrap())
        .collect();

    let fresh_available = available
        .lines()
        .map(|n| n.parse().unwrap())
        .filter(|id| fresh_ranges.iter().any(|range| range.contains(id)))
        .count();

    Some(fresh_available as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (fresh_ranges, _) = input.split_once("\n\n").unwrap();
    let mut fresh_ranges: Vec<_> = fresh_ranges
        .lines()
        .map(|range| range.split_once('-').unwrap())
        .map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse().unwrap())
        .collect();
    fresh_ranges.sort_by_key(|r| *r.end());
    fresh_ranges.sort_by_key(|r| *r.start());

    let mut total = 0;
    let mut i = 0;
    while i < fresh_ranges.len() {
        let current = fresh_ranges[i].clone();

        let overlapping_range = fresh_ranges[i + 1..]
            .iter()
            .filter(|range| current.contains(range.start()))
            .min_by_key(|range| *range.start());

        if let Some(overlap) = overlapping_range
            && current.contains(overlap.end())
        {
            fresh_ranges.remove(fresh_ranges.iter().position(|r| r == overlap).unwrap());
            continue;
        }

        let limit = overlapping_range.map_or(*current.end(), |limit| *limit.start() - 1);

        if limit >= *current.start() {
            total += limit - current.start() + 1;
        }
        i += 1;
    }

    // let total_fresh = fresh_ranges
    //     .into_iter()
    //     .fold((0, vec![]), |(total, prev_ranges), e| {
    //         let overlapping = prev_ranges.iter().filter(|b| range_overlaps(&e, b));

    //         (0, vec![e])
    //     });

    Some(total)
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
        assert_eq!(result, Some(14));
    }
}
