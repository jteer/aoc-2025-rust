advent_of_code::solution!(8);

fn parse_points(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .map(|line| {
            let coords: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    use std::collections::BinaryHeap;
    let points = parse_points(input);
    let n = points.len();

    // For tests to pass / example input
    let k = if n > 20 { 1000 } else { 10 };

    let mut heap = BinaryHeap::with_capacity(k);

    for (i, &(x1, y1, z1)) in points.iter().enumerate() {
        for (j, &(x2, y2, z2)) in points.iter().enumerate().skip(i + 1) {
            let dist_sq = {
                let dx = (x1 - x2) as i64;
                let dy = (y1 - y2) as i64;
                let dz = (z1 - z2) as i64;
                (dx * dx + dy * dy + dz * dz) as u64
            };

            if heap.len() < k {
                heap.push((dist_sq, i, j));
            } else if let Some(&(max_dist, _, _)) = heap.peek() {
                if dist_sq < max_dist {
                    heap.pop();
                    heap.push((dist_sq, i, j));
                }
            }
        }
    }

    let mut edges: Vec<_> = heap.into_iter().collect();
    edges.sort_unstable_by_key(|&(dist, _, _)| dist);

    let mut dsu = DisjointSet::new(n);
    for &(_, i, j) in &edges {
        dsu.union(i, j);
    }

    let mut comp_sizes: Vec<_> = (0..n)
        .filter_map(|i| {
            if dsu.parent[i] == i {
                Some(dsu.size[i] as usize)
            } else {
                None
            }
        })
        .collect();

    comp_sizes.sort_unstable_by(|a, b| b.cmp(a));
    Some(comp_sizes.iter().take(3).map(|&s| s as u64).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let points = parse_points(input);
    let n = points.len();

    let mut min_dist = vec![u64::MAX; n];
    let mut parent = vec![usize::MAX; n];
    let mut visited = vec![false; n];

    min_dist[0] = 0;
    parent[0] = 0;

    let mut last_edge = (0, 0);

    for _ in 0..n {
        let u = (0..n)
            .filter(|&i| !visited[i])
            .min_by_key(|&i| min_dist[i])?;
        visited[u] = true;

        if parent[u] != u {
            last_edge = (parent[u], u);
        }

        let (x1, y1, z1) = points[u];
        for v in 0..n {
            if visited[v] || v == u {
                continue;
            }

            let (x2, y2, z2) = points[v];
            let dist = {
                let dx = (x1 - x2) as i64;
                let dy = (y1 - y2) as i64;
                let dz = (z1 - z2) as i64;
                (dx * dx + dy * dy + dz * dz) as u64
            };

            if dist < min_dist[v] {
                min_dist[v] = dist;
                parent[v] = u;
            }
        }
    }

    Some((points[last_edge.0].0 * points[last_edge.1].0) as u64)
}

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<u16>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut i: usize) -> usize {
        let mut root = i;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        while i != root {
            let next = self.parent[i];
            self.parent[i] = root;
            i = next;
        }
        root
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i == root_j {
            return false;
        }
        if self.size[root_i] < self.size[root_j] {
            self.parent[root_i] = root_j;
            self.size[root_j] += self.size[root_i];
        } else {
            self.parent[root_j] = root_i;
            self.size[root_i] += self.size[root_j];
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(40));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(25272));
    }
}
