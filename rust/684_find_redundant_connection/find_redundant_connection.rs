struct Solution;

impl Solution {
    fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(edges.len());

        for edge in edges {
            if uf.union(edge[0] as usize - 1, edge[1] as usize - 1) {
                return edge;
            }
        }

        vec![]
    }
}

struct UnionFind {
    parent: Vec<usize>,
    cnt: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();

        UnionFind { parent, cnt: n }
    }

    fn find(&mut self, i: usize) -> usize {
        if i != self.parent[i] {
            self.parent[i] = self.find(self.parent[i]);
        }

        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let x = self.find(i);
        let y = self.find(j);

        if x == y {
            true
        } else {
            self.parent[x] = y;
            false
        }
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 2], vec![1, 3], vec![2, 3]], vec![2, 3]),
        (
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]],
            vec![1, 4],
        ),
    ];

    for (edges, ans) in tests {
        assert_eq!(Solution::find_redundant_connection(edges), ans);
    }
}
