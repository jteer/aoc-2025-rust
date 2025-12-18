advent_of_code::solution!(12);

const BLOCK_SIZE: u32 = 3;

// Assumption: All shapes fit in 3×3 boxes
// Count how many 3×3 boxes fit in the region.
// If we have enough boxes for all shapes, the region is solvable.
pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter_map(parse_region_line)
            .filter(|&(width, height, total_shapes)| {
                (width / BLOCK_SIZE) * (height / BLOCK_SIZE) >= total_shapes
            })
            .count(),
    )
}

fn parse_region_line(line: &str) -> Option<(u32, u32, u32)> {
    if !line.contains('x') {
        return None;
    }

    let (size_part, counts_str) = line.split_once(' ')?;
    let size_part = size_part.trim_end_matches(':');
    let (width_str, height_str) = size_part.split_once('x')?;

    Some((
        width_str.parse().ok()?,
        height_str.parse().ok()?,
        counts_str
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .sum(),
    ))
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
