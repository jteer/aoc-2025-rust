use std::collections::HashMap;

advent_of_code::solution!(10);

type ButtonMatrix = Vec<Vec<usize>>;

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let (buttons, light_target, _) = parse_line(line);
                let matrix = build_matrix(&buttons, light_target.len());
                all_gf2_solutions(&matrix, &light_target)
                    .into_iter()
                    .map(|sol| sol.iter().filter(|&&x| x == 1).count() as u64)
                    .min()
            })
            .sum(),
    )
}

// see https://old.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| {
                let (buttons, _, joltage_target) = parse_line(line);
                let mut cache = HashMap::new();
                min_joltage_presses(&buttons, &joltage_target, &mut cache)
            })
            .sum(),
    )
}

fn solve_gf2(a: &[Vec<u8>], b: &[u8]) -> Option<Vec<u8>> {
    let (m, n) = (a.len(), a[0].len());
    let mut aug: Vec<_> = a
        .iter()
        .zip(b)
        .map(|(row, &bi)| {
            let mut r = row.clone();
            r.push(bi);
            r
        })
        .collect();

    let mut row = 0;
    for col in 0..n {
        if let Some(pivot) = (row..m).find(|&r| aug[r][col] == 1) {
            aug.swap(row, pivot);
            for r in 0..m {
                if r != row && aug[r][col] == 1 {
                    for c in col..=n {
                        aug[r][c] ^= aug[row][c];
                    }
                }
            }
            row += 1;
        }
    }

    if aug[row..].iter().any(|row| row[n] != 0) {
        return None;
    }

    let mut x = vec![0; n];
    for i in (0..row.min(n)).rev() {
        let pivot = (0..n).position(|j| aug[i][j] == 1)?;
        x[pivot] = aug[i][n]
            ^ (pivot + 1..n)
                .filter(|&j| aug[i][j] == 1)
                .fold(0, |acc, j| acc ^ x[j]);
    }
    Some(x)
}

fn all_gf2_solutions(a: &[Vec<u8>], b: &[u8]) -> Vec<Vec<u8>> {
    let Some(base) = solve_gf2(a, b) else {
        return Vec::new();
    };
    let (m, n) = (a.len(), a[0].len());
    let mut rref = a.to_vec();
    let mut pivots = vec![None; n];
    let mut row = 0;

    for col in 0..n {
        if let Some(pivot) = (row..m).find(|&r| rref[r][col] == 1) {
            rref.swap(row, pivot);
            pivots[col] = Some(row);
            for r in 0..m {
                if r != row && rref[r][col] == 1 {
                    for c in 0..n {
                        rref[r][c] ^= rref[row][c];
                    }
                }
            }
            row += 1;
        }
    }

    let free_vars: Vec<_> = (0..n).filter(|&col| pivots[col].is_none()).collect();
    if free_vars.is_empty() {
        return vec![base];
    }

    // hacky ... for part 2, limit the number of free variables we explore
    // If there are too many, just return the base solution
    if free_vars.len() > 10 {
        return vec![base];
    }

    let basis: Vec<_> = free_vars
        .iter()
        .map(|&free_col| {
            let mut vec = vec![0u8; n];
            vec[free_col] = 1;
            for (col, &pivot_row) in pivots.iter().enumerate().take(free_col) {
                if let Some(row) = pivot_row {
                    vec[col] = rref[row][free_col];
                }
            }
            vec
        })
        .collect();

    let mut solutions: Vec<_> = (0..1 << free_vars.len())
        .map(|mask| {
            let mut sol = base.clone();
            for (i, basis_vec) in basis.iter().enumerate() {
                if (mask >> i) & 1 == 1 {
                    for j in 0..n {
                        sol[j] ^= basis_vec[j];
                    }
                }
            }
            sol
        })
        .collect();

    solutions.sort_unstable_by_key(|sol| sol.iter().filter(|&&x| x == 1).count());
    solutions
}

fn build_matrix(buttons: &ButtonMatrix, m: usize) -> Vec<Vec<u8>> {
    let mut matrix = vec![vec![0u8; buttons.len()]; m];
    for (j, indices) in buttons.iter().enumerate() {
        for &i in indices.iter().filter(|&&i| i < m) {
            matrix[i][j] = 1;
        }
    }
    matrix
}

fn min_joltage_presses(
    buttons: &ButtonMatrix,
    target: &[usize],
    cache: &mut HashMap<Vec<usize>, Option<u64>>,
) -> Option<u64> {
    if target.iter().all(|&x| x == 0) {
        return Some(0);
    }
    if let Some(&cached) = cache.get(target) {
        return cached;
    }

    let m = target.len();
    let matrix = build_matrix(buttons, m);
    let parity: Vec<_> = target.iter().map(|&x| (x & 1) as u8).collect();
    let parity_solutions = all_gf2_solutions(&matrix, &parity);

    if parity_solutions.is_empty() {
        cache.insert(target.to_vec(), None);
        return None;
    }

    let mut best = None;

    // hacky but works...
    // only try first 10 solutions (sorted by weight already)
    for solution in parity_solutions.iter().take(10) {
        let mut remaining = target.to_vec();
        let mut valid = true;

        for (j, &press) in solution.iter().enumerate() {
            if press == 1 {
                for &i in buttons[j].iter().filter(|&&i| i < m) {
                    if remaining[i] == 0 {
                        valid = false;
                        break;
                    }
                    remaining[i] -= 1;
                }
                if !valid {
                    break;
                }
            }
        }

        if valid && remaining.iter().all(|&x| x & 1 == 0) {
            let halved: Vec<_> = remaining.iter().map(|&x| x >> 1).collect();
            if let Some(sub) = min_joltage_presses(buttons, &halved, cache) {
                let solution_weight = solution.iter().filter(|&&x| x == 1).count() as u64;
                let total = solution_weight + (sub << 1);

                best = Some(best.map_or(total, |b: u64| b.min(total)));

                // Early exit if remaining is all zeros (can't improve)
                if sub == 0 {
                    break;
                }
            }
        }
    }

    cache.insert(target.to_vec(), best);
    best
}

fn parse_line(line: &str) -> (ButtonMatrix, Vec<u8>, Vec<usize>) {
    let parts: Vec<_> = line.split_whitespace().collect();

    let light_target: Vec<_> = parts[0][1..parts[0].len() - 1]
        .bytes()
        .map(|c| (c == b'#') as u8)
        .collect();

    let mut buttons = Vec::new();
    let mut joltage_target = Vec::new();

    for part in &parts[1..] {
        match part.as_bytes()[0] {
            b'(' => buttons.push(
                part[1..part.len() - 1]
                    .split(',')
                    .filter_map(|s| s.parse().ok())
                    .collect(),
            ),
            b'{' => {
                joltage_target = part[1..part.len() - 1]
                    .split(',')
                    .filter_map(|s| s.parse().ok())
                    .collect();
                break;
            }
            _ => {}
        }
    }

    (buttons, light_target, joltage_target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
