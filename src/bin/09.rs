advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse_points(input);
    let mut result = 0u64;
    for i in 0..coords.len() {
        let (ax, ay) = coords[i];
        for &(bx, by) in &coords[i + 1..] {
            result = result.max((ax.abs_diff(bx) + 1) * (ay.abs_diff(by) + 1));
        }
    }
    Some(result)
}

// based on https://github.com/timvisee/advent-of-code-2025/blob/a47046a4ba864bb006b943010e56ae0594144f94/day09b/src/main.rs
// https://old.reddit.com/r/adventofcode/comments/1phywvn/2025_day_9_solutions/nt9j7b0/
// Optimization from my og part 2:
// Separate horizontal and vertical edges and sort them by coordinate.
// This allows binary search to check only ~10-20 relevant edges per rectangle instead of all 100-1000 edges.
// Additionally, checking the longest edges first provides early rejection for most invalid rectangles.
// The naive approach checks ALL edges for EVERY rectangle (O(n^3) with poor constants),
// while this approach uses binary search + range checking (O(n^2.5) with good constants).
pub fn part_two(input: &str) -> Option<u64> {
    let coords = parse_points(input);
    let n = coords.len();

    let mut h_edges = Vec::new(); // (y, x_min, x_max)
    let mut v_edges = Vec::new(); // (x, y_min, y_max)

    for i in 0..n {
        let (x1, y1) = coords[i];
        let (x2, y2) = coords[(i + 1) % n];

        if x1 == x2 {
            v_edges.push((x1, y1.min(y2), y1.max(y2)));
        } else {
            h_edges.push((y1, x1.min(x2), x1.max(x2)));
        }
    }

    // Sort edges by their perpendicular coordinate
    h_edges.sort_unstable_by_key(|&(y, _, _)| y);
    v_edges.sort_unstable_by_key(|&(x, _, _)| x);

    let h_ys: Vec<i64> = h_edges.iter().map(|&(y, _, _)| y).collect();
    let v_xs: Vec<i64> = v_edges.iter().map(|&(x, _, _)| x).collect();

    let h_max = h_edges.iter().max_by_key(|e| e.2 - e.1).copied();
    let v_max = v_edges.iter().max_by_key(|e| e.2 - e.1).copied();

    let mut result = 0u64;

    for i in 0..n {
        let (ax, ay) = coords[i];

        for &(bx, by) in &coords[i + 1..] {
            let width = ax.abs_diff(bx) + 1;
            let height = ay.abs_diff(by) + 1;
            let area = width * height;

            if area <= result {
                continue;
            }

            let rect = (ax.min(bx), ay.min(by), ax.max(bx), ay.max(by));

            if check_rectangle(rect, &h_edges, &v_edges, h_max, v_max, &h_ys, &v_xs) {
                result = area;
            }
        }
    }

    Some(result)
}

#[inline]
fn check_rectangle(
    (x_min, y_min, x_max, y_max): (i64, i64, i64, i64),
    h_edges: &[(i64, i64, i64)],
    v_edges: &[(i64, i64, i64)],
    h_max: Option<(i64, i64, i64)>,
    v_max: Option<(i64, i64, i64)>,
    h_ys: &[i64],
    v_xs: &[i64],
) -> bool {
    if let Some((y, x_l, x_r)) = h_max
        && y_min < y
        && y < y_max
        && ((x_l < x_max && x_max <= x_r) || (x_l <= x_min && x_min < x_r))
    {
        return false;
    }

    if let Some((x, y_t, y_b)) = v_max
        && x_min < x
        && x < x_max
        && ((y_t < y_max && y_max <= y_b) || (y_t <= y_min && y_min < y_b))
    {
        return false;
    }

    // Check horizontal edges that could intersect (those with y_min < y < y_max)
    let start = h_ys.partition_point(|&y| y <= y_min);
    let end = h_ys[start..].partition_point(|&y| y < y_max) + start;

    for &(_, x_l, x_r) in &h_edges[start..end] {
        // Edge intersects if it crosses either vertical side
        if (x_l <= x_min && x_min < x_r) || (x_l < x_max && x_max <= x_r) {
            return false;
        }
    }

    // Check vertical edges that could intersect (those with x_min < x < x_max)
    let start = v_xs.partition_point(|&x| x <= x_min);
    let end = v_xs[start..].partition_point(|&x| x < x_max) + start;

    for &(_, y_t, y_b) in &v_edges[start..end] {
        // Edge intersects if it crosses either horizontal side
        if (y_t <= y_min && y_min < y_b) || (y_t < y_max && y_max <= y_b) {
            return false;
        }
    }

    true
}

// og part 2
pub fn part_two_slow(input: &str) -> Option<u64> {
    let points = parse_points(input);
    let n = points.len();

    // Pre-compute edges as AABBs
    let mut edges = Vec::with_capacity(n);
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];
        edges.push((x1.min(x2), x1.max(x2), y1.min(y2), y1.max(y2)));
    }

    let mut result = 0u64;

    // Try all pairs of points
    for i in 0..n {
        let (ax, ay) = points[i];

        for j in i + 1..n {
            let (bx, by) = points[j];

            let width = ax.abs_diff(bx) + 1;
            let height = ay.abs_diff(by) + 1;
            let area = width * height;

            // Skip if this can't beat current best
            if area <= result {
                continue;
            }

            // Check if rectangle is valid (doesn't intersect polygon edges except at corners)
            if rectangle_is_valid(&edges, ax, ay, bx, by, i, j, n) {
                result = area;
            }
        }
    }

    Some(result)
}

// og part 2
#[inline]
#[allow(clippy::too_many_arguments)]
fn rectangle_is_valid(
    edges: &[(i64, i64, i64, i64)],
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    idx_a: usize,
    idx_b: usize,
    n: usize,
) -> bool {
    // Rectangle bounds
    let (l1, r1, t1, b1) = (ax.min(bx), ax.max(bx), ay.min(by), ay.max(by));

    // Check all edges of the polygon
    for edge_idx in 0..n {
        let next_idx = (edge_idx + 1) % n;

        // Skip edges that are incident to our chosen corner points
        // An edge is incident if either endpoint is one of our corners
        if edge_idx == idx_a || edge_idx == idx_b || next_idx == idx_a || next_idx == idx_b {
            continue;
        }

        let (l2, r2, t2, b2) = edges[edge_idx];

        // Check if edge AABB intersects rectangle AABB
        if l1 < r2 && r1 > l2 && t1 < b2 && b1 > t2 {
            return false;
        }
    }

    true
}

#[inline(always)]
fn parse_points(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter_map(|l| {
            let (xs, ys) = l.split_once(',')?;
            Some((xs.parse().ok()?, ys.parse().ok()?))
        })
        .collect()
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
