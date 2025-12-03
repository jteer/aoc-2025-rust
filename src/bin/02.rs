use std::collections::HashSet;

advent_of_code::solution!(2);

/**
 * If m has d digits, then
 *  The first occurrence of m is just m
 *  The second occurrence needs to be shifted left by d digits
 *  The third needs to be shifted left by 2d digits, etc.
 *
 * m repeated r times = m × 10^{d×(r-1)} + m × 10^{d×(r-2)} + ... + m × 10^{d×1} + m × 10^{0}
 *
 * Every number consisting of a repeating block is essentially m multiplied by a "repunit" (all-1s number) scaled by the block length.
 */
pub fn part_one(input: &str) -> Option<u64> {
    // input
    //     .lines()
    //     .next()?
    //     .split(',')
    //     .filter_map(|range| {
    //         let (start, end) = range.split_once('-')?;
    //         let start: i64 = start.parse().ok()?;
    //         let end: i64 = end.parse().ok()?;
    //         Some(start..=end)
    //     })
    //     .flatten()
    //     .filter(|&n| {
    //         let s = n.to_string();
    //         s.len() % 2 == 0 && s[..s.len() / 2] == s[s.len() / 2..]
    //     })
    //     .sum::<i64>()
    //     .into()
    //
    let total = input
        .lines()
        .next()?
        .split(',')
        .filter_map(parse_range)
        .map(|(l, r)| {
            let mut sum = 0;
            let mut seen = HashSet::new();

            for d in 1.. {
                let pow10_d = ten_pow(d);
                let multiplier = pow10_d + 1;
                let min_m = ten_pow(d - 1);

                if min_m * multiplier > r {
                    break;
                }

                sum += (min_m..ten_pow(d))
                    .map(|m| m * multiplier)
                    .take_while(|&candidate| candidate <= r)
                    .filter(|&candidate| candidate >= l && seen.insert(candidate))
                    .sum::<u64>();
            }
            sum
        })
        .sum();

    Some(total)
}
pub fn part_two(input: &str) -> Option<u64> {
    // input
    //     .lines()
    //     .next()?
    //     .split(',')
    //     .filter_map(|range| {
    //         let (start, end) = range.split_once('-')?;
    //         let start: i64 = start.parse().ok()?;
    //         let end: i64 = end.parse().ok()?;
    //         Some(start..=end)
    //     })
    //     .flatten()
    //     .filter(|&n| {
    //         let s = n.to_string();
    //         let len = s.len();
    //         (1..=len / 2)
    //             .filter(|&block_length| len % block_length == 0)
    //             .any(|block_length| s[..block_length].repeat(len / block_length) == s)
    //     })
    //     .sum::<i64>()
    //     .into()
    let mut total = 0;
    for range_str in input.lines().next()?.split(',') {
        let (l, r) = parse_range(range_str)?;
        let max_digits = r.ilog10() + 1;
        let mut seen = HashSet::new();

        // For all possible block lengths d
        for d in 1..=max_digits / 2 {
            let pow10_d = ten_pow(d);
            let denominator = pow10_d - 1;

            // For all repetition counts r >= 2
            for rep in 2.. {
                let total_digits = d * rep;
                if total_digits > max_digits {
                    break;
                }

                // multiplier = (10^(d*r) - 1) / (10^d - 1)
                let pow10_dr = ten_pow(total_digits);
                // denominator can't be 0 since d >= 1
                let multiplier = (pow10_dr - 1) / denominator;

                // Smallest m with d digits
                let min_m = ten_pow(d - 1);

                // if smallest candidate > R
                if min_m * multiplier > r {
                    break;
                }

                // Try all m with exactly d digits
                for m in min_m..ten_pow(d) {
                    let candidate = m * multiplier;
                    if candidate > r {
                        break;
                    }
                    if candidate >= l && seen.insert(candidate) {
                        total += candidate;
                    }
                }
            }
        }
    }

    Some(total)
}

fn ten_pow(exp: u32) -> u64 {
    10_u64.pow(exp)
}

// Parse a single range "L-R" into (L, R)
fn parse_range(range_str: &str) -> Option<(u64, u64)> {
    let mut parts = range_str.split('-');
    let start = parts.next()?.parse().ok()?;
    let end = parts.next()?.parse().ok()?;
    Some((start, end))
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
