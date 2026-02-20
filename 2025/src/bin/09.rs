use std::{ops::RangeInclusive, str::FromStr};

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: u64,
    y: u64,
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(())?;
        Ok(Self {
            x: x.parse().map_err(|_| ())?,
            y: y.parse().map_err(|_| ())?,
        })
    }
}

struct Rect {
    x: RangeInclusive<u64>,
    y: RangeInclusive<u64>,
}

impl Rect {
    fn new(a: &Coord, b: &Coord) -> Self {
        Self {
            x: a.x.min(b.x)..=a.x.max(b.x),
            y: a.y.min(b.y)..=a.y.max(b.y),
        }
    }

    fn area(&self) -> u64 {
        (self.x.end() - self.x.start() + 1) * (self.y.end() - self.y.start() + 1)
    }

    fn inner(&self) -> Self {
        Self {
            x: self.x.start() + 1..=self.x.end() - 1,
            y: self.y.start() + 1..=self.y.end() - 1,
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        overlaps(&self.x, &other.x) && overlaps(&self.y, &other.y)
    }
}

fn overlaps(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.start().max(b.start()) <= a.end().min(b.end())
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords = input
        .lines()
        .map(Coord::from_str)
        .collect::<Result<Vec<_>, ()>>()
        .ok()?;

    let max_area = coords
        .iter()
        .enumerate()
        .flat_map(|(i, a)| coords[i + 1..].iter().map(|b| Rect::new(a, b)))
        .map(|r| r.area())
        .max();

    max_area
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords = input
        .lines()
        .map(Coord::from_str)
        .collect::<Result<Vec<_>, ()>>()
        .ok()?;

    let mut rects: Vec<Rect> = coords
        .iter()
        .enumerate()
        .flat_map(|(i, a)| coords[i + 1..].iter().map(|b| Rect::new(a, b)))
        .collect();
    rects.sort_by_key(|r| u64::MAX - r.area());

    let lines: Vec<Rect> = {
        let mut coords = coords;
        coords.push(coords[0]);
        coords
            .windows(2)
            .map(|coords| Rect::new(&coords[0], &coords[1]))
            .collect()
    };

    rects
        .into_iter()
        .find(|rect| {
            let inner = rect.inner();
            !lines.iter().any(|l| l.overlaps(&inner))
        })
        .map(|r| r.area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
