advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<i64> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let start_col = grid[0].iter().position(|&c| c == 'S')?;
    let mut split_count = 0;

    if rows > 1 {
        grid[1][start_col] = '|';
    }

    for r in 1..rows - 1 {
        for c in 0..cols {
            if grid[r][c] != '|' {
                continue;
            }

            let next_r = r + 1;

            match grid[next_r][c] {
                '^' => {
                    split_count += 1;

                    if c > 0 && grid[next_r][c - 1] != '|' {
                        grid[next_r][c - 1] = '|';
                    }
                    if c + 1 < cols && grid[next_r][c + 1] != '|' {
                        grid[next_r][c + 1] = '|';
                    }
                }
                '.' => {
                    grid[next_r][c] = '|';
                }
                _ => {} // Already a beam here
            }
        }
    }

    Some(split_count)
}

// Could solve using DFS?
pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut count: Vec<Vec<u64>> = vec![vec![0; cols]; rows];
    count[0][grid[0].iter().position(|&c| c == 'S')?] = 1;

    for r in 0..rows - 1 {
        for c in 0..cols {
            if count[r][c] == 0 {
                continue;
            }

            let current_timelines = count[r][c];
            let next_r = r + 1;

            if grid[next_r][c] == '^' {
                if c > 0 {
                    count[next_r][c - 1] += current_timelines;
                }
                if c + 1 < cols {
                    count[next_r][c + 1] += current_timelines;
                }
            } else {
                count[next_r][c] += current_timelines;
            }
        }
    }

    Some(count[rows - 1].iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
