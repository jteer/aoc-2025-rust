advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i64,
    end: i64,
}

fn merge_intervals(mut ranges: Vec<Interval>) -> Vec<Interval> {
    ranges.sort_by_key(|i| i.start);

    let mut merged: Vec<Interval> = Vec::new();
    for r in ranges {
        match merged.last_mut() {
            Some(last) if r.start <= last.end => {
                last.end = last.end.max(r.end);
            }
            _ => merged.push(r),
        }
    }

    merged
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut blocks = input.split("\n\n").filter(|s| !s.is_empty());

    let ranges: Vec<Interval> = blocks
        .next()?
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once('-')?;
            Some(Interval {
                start: a.parse().ok()?,
                end: b.parse().ok()?,
            })
        })
        .collect();

    let merged = merge_intervals(ranges);

    let queries: Vec<i64> = blocks
        .next()?
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    Some(queries.iter().filter(|&&q| contains(&merged, q)).count())
}

fn contains(intervals: &[Interval], q: i64) -> bool {
    intervals
        .binary_search_by(|int| {
            if q < int.start {
                std::cmp::Ordering::Greater
            } else if q > int.end {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut blocks = input.split("\n\n").filter(|s| !s.is_empty());
    let ranges: Vec<Interval> = blocks
        .next()?
        .lines()
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            Some(Interval {
                start: start.parse().ok()?,
                end: end.parse().ok()?,
            })
        })
        .collect();

    let merged = merge_intervals(ranges);

    Some(merged.iter().map(|i| (i.end - i.start + 1)).sum::<i64>())
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
