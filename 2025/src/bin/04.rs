advent_of_code::solution!(4);

fn make_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

pub fn part_one(input: &str) -> Option<u64> {
    let grid = make_grid(input);
    let mut accessible = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '@' {
                let occupied = NEIGHBORS
                    .iter()
                    .map(|(dx, dy)| (x as i32 - dx, y as i32 - dy))
                    .filter(|&(x, y)| {
                        x >= 0 && y >= 0 && x < row.len() as i32 && y < grid.len() as i32
                    })
                    .filter(|&(x, y)| grid[y as usize][x as usize] == '@')
                    .count();

                if occupied < 4 {
                    // println!("{x},{y}: {occupied}");
                    accessible += 1;
                }
            }
        }
    }

    Some(accessible)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = make_grid(input);
    let mut total_removed = 0;

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut removed = true;
    while removed {
        removed = false;
        for y in 0..height {
            for x in 0..width {
                if grid[y as usize][x as usize] == '@' {
                    let occupied = NEIGHBORS
                        .iter()
                        .map(|(dx, dy)| (x - dx, y - dy))
                        .filter(|&(x, y)| x >= 0 && y >= 0 && x < width && y < height)
                        .filter(|&(x, y)| grid[y as usize][x as usize] == '@')
                        .count();

                    if occupied < 4 {
                        // println!("{x},{y}: {occupied}");
                        grid[y as usize][x as usize] = '.';
                        total_removed += 1;
                        removed = true;
                    }
                }
            }
        }
    }

    Some(total_removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
