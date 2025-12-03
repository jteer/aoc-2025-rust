advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i64> {
    let total = input.lines().map(|line| max_digits(line, 2)).sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<i64> {
    let total = input.lines().map(|line| max_digits(line, 12)).sum();
    Some(total)
}

fn max_digits(line: &str, k: usize) -> i64 {
    let digits: Vec<u8> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let n = digits.len();
    if k > n {
        return 0;
    }

    let remove = n - k;
    if remove > 0 {
        let mut stack: Vec<u8> = Vec::with_capacity(k);

        for (i, &digit) in digits.iter().enumerate() {
            while !stack.is_empty() && stack.len() + (n - i) > k && digit > *stack.last().unwrap() {
                stack.pop();
            }
            if stack.len() < k {
                stack.push(digit);
            }
        }
        stack.iter().fold(0i64, |acc, &d| acc * 10 + d as i64)
    } else {
        digits.iter().fold(0i64, |acc, &d| acc * 10 + d as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
