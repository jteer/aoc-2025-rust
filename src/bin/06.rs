advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<i64> {
    let mut lines = input.lines();
    let ops: Vec<u8> = lines.next_back()?.bytes().filter(|&c| c != b' ').collect();
    let mut columns: Vec<Vec<i64>> = vec![Vec::new(); ops.len()];

    for line in lines {
        // "Could" improve by manually parse number and line but 🤷‍♂️
        for (i, word) in line.split_ascii_whitespace().enumerate() {
            columns[i].push(word.parse().ok()?);
        }
    }

    Some(
        columns
            .iter()
            .zip(ops)
            .map(|(col, op)| match op {
                b'*' => col.iter().product(),
                b'+' => col.iter().sum(),
                _ => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();

    let height = lines.len();
    let op_row = height - 1;
    let width = lines.first()?.len();
    let ops: Vec<u8> = lines[op_row]
        .iter()
        .copied()
        .filter(|&c| c != b' ')
        .collect();

    let mut columns = vec![Vec::new()];
    for col in 0..width {
        let mut number = 0i64;
        let mut has_digits = false;

        for line in &lines[..op_row] {
            if let Some(&b) = line.get(col) {
                if b.is_ascii_digit() {
                    number = number * 10 + (b - b'0') as i64;
                    has_digits = true;
                }
            }
        }
        if has_digits {
            columns.last_mut()?.push(number);
        } else {
            columns.push(Vec::new());
        }
    }

    Some(
        columns
            .iter()
            .zip(ops)
            .map(|(col, op)| match op {
                b'*' => col.iter().product(),
                b'+' => col.iter().sum(),
                _ => 0,
            })
            .sum(),
    )
}

// Original Solutions - not great
// pub fn part_one(input: &str) -> Option<i64> {
//     let lines: Vec<&str> = input.lines().collect();
//     let mut col_results: Vec<(i64, i64)> = vec![(0, 0); lines.first()?.len()];
//     for (line_i, line) in lines.iter().take(lines.len() - 1).enumerate() {
//         for (col, num_s) in line
//             .split_whitespace()
//             .collect::<Vec<&str>>()
//             .iter()
//             .enumerate()
//         {
//             let n: i64 = num_s.parse().ok()?;
//             if line_i == 0 {
//                 col_results[col] = (n, n);
//             } else {
//                 col_results[col].0 += n;
//                 col_results[col].1 *= n;
//             }
//         }
//     }
//     Some(
//         lines
//             .last()?
//             .split_whitespace()
//             .zip(col_results)
//             .map(|(op, (sum, product))| match op {
//                 "+" => sum,
//                 _ => product, // assuming only '+' and '*' exist
//             })
//             .sum(),
//     )
// }

// pub fn part_two(input: &str) -> Option<i64> {
//     let lines: Vec<&[u8]> = input.lines().map(|s| s.as_bytes()).collect();
//     let height = lines.len();
//     let op_row = height - 1;
//     let width = lines[0].len();

//     let mut boundaries = Vec::new();
//     let mut start = 0;

//     for col in 0..width {
//         let has_digits =
//             (0..op_row).any(|row| lines[row].get(col).map_or(false, |&b| b.is_ascii_digit()));

//         if !has_digits {
//             if start < col {
//                 boundaries.push((start, col - 1));
//             }
//             start = col + 1;
//         }
//     }

//     if start < width {
//         boundaries.push((start, width - 1));
//     }

//     let mut total = 0;
//     for &(start_col, end_col) in boundaries.iter() {
//         let op = lines[op_row][start_col..=end_col]
//             .iter()
//             .find(|&&b| b == b'+' || b == b'*')
//             .copied()
//             .unwrap_or(b'+');
//         let mut result = match op {
//             b'+' => 0,
//             b'*' => 1,
//             _ => return None,
//         };

//         for col in start_col..=end_col {
//             let mut number = 0;
//             for row in 0..op_row {
//                 if let Some(&b) = lines[row].get(col) {
//                     if b.is_ascii_digit() {
//                         number = number * 10 + (b - b'0') as i64;
//                     }
//                 }
//             }

//             if number == 0
//                 && (0..op_row).all(|row| lines[row].get(col).map_or(true, |&b| !b.is_ascii_digit()))
//             {
//                 continue;
//             }
//             match op {
//                 b'+' => result += number,
//                 b'*' => result *= number,
//                 _ => unreachable!(),
//             }
//         }

//         total += result;
//     }
//     Some(total)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
