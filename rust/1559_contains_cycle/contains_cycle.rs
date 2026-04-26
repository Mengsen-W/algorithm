struct Solution;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];
        UnionFind { parent, size }
    }

    fn find_set(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find_set(self.parent[x]);
        }
        self.parent[x]
    }

    fn unite(&mut self, x: usize, y: usize) {
        let (mut root_x, mut root_y) = (x, y);
        if self.size[root_x] < self.size[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }
        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];
    }

    fn find_and_unite(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find_set(x);
        let root_y = self.find_set(y);
        if root_x != root_y {
            self.unite(root_x, root_y);
            true
        } else {
            false
        }
    }
}

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut uf = UnionFind::new(m * n);

        for i in 0..m {
            for j in 0..n {
                if i > 0 && grid[i][j] == grid[i - 1][j] {
                    let cell1 = i * n + j;
                    let cell2 = (i - 1) * n + j;
                    if !uf.find_and_unite(cell1, cell2) {
                        return true;
                    }
                }
                if j > 0 && grid[i][j] == grid[i][j - 1] {
                    let cell1 = i * n + j;
                    let cell2 = i * n + j - 1;
                    if !uf.find_and_unite(cell1, cell2) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn main() {
    let tests = vec![
        (
            vec![
                vec!['a', 'a', 'a', 'a'],
                vec!['a', 'b', 'b', 'a'],
                vec!['a', 'b', 'b', 'a'],
                vec!['a', 'a', 'a', 'a'],
            ],
            true,
        ),
        (
            vec![
                vec!['c', 'c', 'c', 'a'],
                vec!['c', 'd', 'c', 'c'],
                vec!['c', 'c', 'e', 'c'],
                vec!['f', 'c', 'c', 'c'],
            ],
            true,
        ),
        (
            vec![
                vec!['a', 'b', 'b'],
                vec!['b', 'z', 'b'],
                vec!['b', 'b', 'a'],
            ],
            false,
        ),
    ];

    for (grid, expected) in tests {
        assert_eq!(Solution::contains_cycle(grid), expected);
    }
}
