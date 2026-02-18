use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

advent_of_code::solution!(8);

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coord(i64, i64, i64);

impl Coord {
    fn distance(&self, other: &Self) -> f64 {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)) as f64)
            .sqrt()
    }
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',').map(|n| n.parse().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        Ok(Coord(x, y, z))
    }
}

fn parse_coords(input: &str) -> Result<Vec<Coord>, ()> {
    input.lines().map(Coord::from_str).collect()
}

fn calc_distances(coords: &[Coord]) -> HashMap<(&Coord, &Coord), f64> {
    let mut distances = HashMap::new();

    for (i, a) in coords.iter().enumerate() {
        for b in coords[i + 1..].iter() {
            let d = a.distance(b);
            distances.insert((a, b), d);
        }
    }

    distances
}

fn sorted_pairs_by_distance(coords: &[Coord]) -> Vec<(&Coord, &Coord)> {
    let distances = calc_distances(coords);

    let mut pairs: Vec<_> = distances.into_iter().collect();
    pairs.sort_by_key(|(_, d)| *d as i64);
    pairs.into_iter().map(|(pair, _)| pair).collect()
}

fn build_start_circuits(coords: &[Coord]) -> Vec<HashSet<&Coord>> {
    coords
        .iter()
        .map(|c| {
            let mut set = HashSet::new();
            set.insert(c);
            set
        })
        .collect()
}

fn connect_pair(circuits: &mut Vec<HashSet<&Coord>>, a: &Coord, b: &Coord) {
    let circuit_a = circuits.iter().position(|c| c.contains(a)).unwrap();
    let circuit_b = circuits.iter().position(|c| c.contains(b)).unwrap();

    if circuit_a != circuit_b {
        let min = circuit_a.min(circuit_b);
        let max = circuit_a.max(circuit_b);

        let iter = circuits.remove(max);
        circuits[min].extend(iter);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse_coords(input).ok()?;
    let pairs = sorted_pairs_by_distance(&coords);
    let mut circuits = build_start_circuits(&coords);

    let max_pairs = if coords.len() <= 20 { 10 } else { 1000 };

    for (a, b) in &pairs[..max_pairs] {
        connect_pair(&mut circuits, a, b);
    }

    circuits.sort_by_key(|c| usize::MAX - c.len());

    Some(circuits[..3].iter().map(|c| c.len() as u64).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords = parse_coords(input).ok()?;
    let mut pairs = sorted_pairs_by_distance(&coords).into_iter();
    let mut circuits = build_start_circuits(&coords);

    while let Some((a, b)) = pairs.next()
        && circuits.len() > 1
    {
        connect_pair(&mut circuits, a, b);

        if circuits.len() == 1 {
            return Some(a.0 as u64 * b.0 as u64);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
