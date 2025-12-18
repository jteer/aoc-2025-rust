use std::collections::HashMap;

advent_of_code::solution!(11);

fn build_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        if let Some((node, rest)) = line.split_once(": ") {
            let neighbors: Vec<&str> = rest.split_whitespace().collect();
            graph.insert(node, neighbors);
        }
    }
    graph
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = build_graph(input);
    let mut memo = HashMap::new();

    fn dfs<'a>(
        graph: &HashMap<&'a str, Vec<&'a str>>,
        node: &'a str,
        memo: &mut HashMap<&'a str, u64>,
    ) -> u64 {
        if node == "out" {
            return 1;
        }
        if let Some(&cached) = memo.get(node) {
            return cached;
        }
        let count = graph.get(node).map_or(0, |neighbors| {
            neighbors.iter().map(|&next| dfs(graph, next, memo)).sum()
        });
        memo.insert(node, count);
        count
    }

    Some(dfs(&graph, "you", &mut memo))
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = build_graph(input);
    let mut memo = HashMap::new();

    fn dfs<'a>(
        graph: &HashMap<&'a str, Vec<&'a str>>,
        node: &'a str,
        seen_dac: bool,
        seen_fft: bool,
        memo: &mut HashMap<(&'a str, bool, bool), u64>,
    ) -> u64 {
        let seen_dac = seen_dac || node == "dac";
        let seen_fft = seen_fft || node == "fft";

        if node == "out" {
            return if seen_dac && seen_fft { 1 } else { 0 };
        }

        let key = (node, seen_dac, seen_fft);
        if let Some(&cached) = memo.get(&key) {
            return cached;
        }

        let count = graph.get(node).map_or(0, |neighbors| {
            neighbors
                .iter()
                .map(|&next| dfs(graph, next, seen_dac, seen_fft, memo))
                .sum()
        });

        memo.insert(key, count);
        count
    }

    Some(dfs(&graph, "svr", false, false, &mut memo))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
