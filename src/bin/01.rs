advent_of_code::solution!(1);

const MAX_VALUE: i32 = 100;
const INITIAL_DIAL: i32 = 50;

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .try_fold((0u64, INITIAL_DIAL), |(mut zeros, mut dial), line| {
            let (direction, turn_str) = line.split_at(1);
            let number: i32 = turn_str.parse().ok()?;

            dial = match direction {
                "L" => (dial + MAX_VALUE - number) % (MAX_VALUE),
                "R" => (dial + number) % (MAX_VALUE),
                _ => return None,
            };

            if dial == 0 {
                zeros += 1;
            }

            Some((zeros, dial))
        })
        .map(|(zeros, _)| zeros)
}

pub fn part_two(input: &str) -> Option<i32> {
    input
        .lines()
        .try_fold((0, INITIAL_DIAL), |(answer, pos), line| {
            let (direction, turn_str) = line.split_at(1);
            let turn: i32 = turn_str.parse().ok()?;

            let mut new_answer = answer + turn / MAX_VALUE;
            let remainder = turn % MAX_VALUE;

            let new_pos = match direction {
                "L" => {
                    if remainder > pos && pos != 0 {
                        new_answer += 1;
                    }
                    (pos - turn).rem_euclid(MAX_VALUE)
                }
                "R" => {
                    if pos + remainder > MAX_VALUE {
                        new_answer += 1;
                    }
                    (pos + turn) % MAX_VALUE
                }
                _ => return None,
            };

            let final_answer = if new_pos == 0 {
                new_answer + 1
            } else {
                new_answer
            };

            Some((final_answer, new_pos))
        })
        .map(|(answer, _)| answer)
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
        assert_eq!(result, Some(1));
    }
}
