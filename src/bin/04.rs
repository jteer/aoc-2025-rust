advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, rows, cols) = parse_grid(input);
    Some(
        grid.iter()
            .enumerate()
            .filter(|&(idx, &cell)| cell == b'@' && count_neighbors(&grid, idx, rows, cols) < 4)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut grid, rows, cols) = parse_grid(input);

    let mut queue: Vec<_> = (0..grid.len())
        .filter(|&idx| grid[idx] == b'@' && count_neighbors(&grid, idx, rows, cols) < 4)
        .collect();

    let mut rolls = 0;
    while let Some(idx) = queue.pop() {
        if grid[idx] != b'@' || count_neighbors(&grid, idx, rows, cols) >= 4 {
            continue;
        }

        grid[idx] = b'.';
        rolls += 1;

        for &(dr, dc) in &NEIGHBORS {
            let nr = (idx / cols) as isize + dr;
            let nc = (idx % cols) as isize + dc;

            if (0..rows as isize).contains(&nr) && (0..cols as isize).contains(&nc) {
                let n_idx = nr as usize * cols + nc as usize;
                if grid[n_idx] == b'@' && count_neighbors(&grid, n_idx, rows, cols) < 4 {
                    queue.push(n_idx);
                }
            }
        }
    }

    Some(rolls)
}

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse_grid(input: &str) -> (Vec<u8>, usize, usize) {
    let rows = input.lines().count();
    let cols = input.lines().next().map(|l| l.len()).unwrap_or(0);
    let grid = input.lines().flat_map(|line| line.bytes()).collect();
    (grid, rows, cols)
}

#[inline]
fn count_neighbors(grid: &[u8], idx: usize, rows: usize, cols: usize) -> u8 {
    let mut count = 0;
    for &(dr, dc) in &NEIGHBORS {
        let nr = (idx / cols) as isize + dr;
        let nc = (idx % cols) as isize + dc;

        if (0..rows as isize).contains(&nr)
            && (0..cols as isize).contains(&nc)
            && grid[nr as usize * cols + nc as usize] == b'@'
        {
            count += 1;
            if count >= 4 {
                return count;
            }
        }
    }
    count
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
