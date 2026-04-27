struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut ds = DisjointSet::new(m, n);

        for i in 0..m {
            for j in 0..n {
                Self::handler(&grid, m, n, &mut ds, i, j);
            }
        }

        ds.find(Self::get_id(0, 0, n)) == ds.find(Self::get_id(m - 1, n - 1, n))
    }

    fn get_id(x: usize, y: usize, n: usize) -> usize {
        x * n + y
    }

    fn detect_l(
        grid: &Vec<Vec<i32>>,
        m: usize,
        n: usize,
        ds: &mut DisjointSet,
        x: usize,
        y: usize,
    ) {
        if y > 0 && (grid[x][y - 1] == 4 || grid[x][y - 1] == 6 || grid[x][y - 1] == 1) {
            ds.merge(Self::get_id(x, y, n), Self::get_id(x, y - 1, n));
        }
    }

    fn detect_r(
        grid: &Vec<Vec<i32>>,
        m: usize,
        n: usize,
        ds: &mut DisjointSet,
        x: usize,
        y: usize,
    ) {
        if y + 1 < n && (grid[x][y + 1] == 3 || grid[x][y + 1] == 5 || grid[x][y + 1] == 1) {
            ds.merge(Self::get_id(x, y, n), Self::get_id(x, y + 1, n));
        }
    }

    fn detect_u(
        grid: &Vec<Vec<i32>>,
        m: usize,
        n: usize,
        ds: &mut DisjointSet,
        x: usize,
        y: usize,
    ) {
        if x > 0 && (grid[x - 1][y] == 3 || grid[x - 1][y] == 4 || grid[x - 1][y] == 2) {
            ds.merge(Self::get_id(x, y, n), Self::get_id(x - 1, y, n));
        }
    }

    fn detect_d(
        grid: &Vec<Vec<i32>>,
        m: usize,
        n: usize,
        ds: &mut DisjointSet,
        x: usize,
        y: usize,
    ) {
        if x + 1 < m && (grid[x + 1][y] == 5 || grid[x + 1][y] == 6 || grid[x + 1][y] == 2) {
            ds.merge(Self::get_id(x, y, n), Self::get_id(x + 1, y, n));
        }
    }

    fn handler(grid: &Vec<Vec<i32>>, m: usize, n: usize, ds: &mut DisjointSet, x: usize, y: usize) {
        match grid[x][y] {
            1 => {
                Self::detect_l(grid, m, n, ds, x, y);
                Self::detect_r(grid, m, n, ds, x, y);
            }
            2 => {
                Self::detect_u(grid, m, n, ds, x, y);
                Self::detect_d(grid, m, n, ds, x, y);
            }
            3 => {
                Self::detect_l(grid, m, n, ds, x, y);
                Self::detect_d(grid, m, n, ds, x, y);
            }
            4 => {
                Self::detect_r(grid, m, n, ds, x, y);
                Self::detect_d(grid, m, n, ds, x, y);
            }
            5 => {
                Self::detect_l(grid, m, n, ds, x, y);
                Self::detect_u(grid, m, n, ds, x, y);
            }
            6 => {
                Self::detect_r(grid, m, n, ds, x, y);
                Self::detect_u(grid, m, n, ds, x, y);
            }
            _ => {}
        }
    }
}

struct DisjointSet {
    f: Vec<usize>,
}

impl DisjointSet {
    fn new(m: usize, n: usize) -> Self {
        let size = m * n;
        let mut f = Vec::with_capacity(size);
        for i in 0..size {
            f.push(i);
        }
        DisjointSet { f }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.f[x] {
            return x;
        }
        self.f[x] = self.find(self.f[x]);
        self.f[x]
    }

    fn merge(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        self.f[root_x] = root_y;
    }
}

fn main() {
    let tests = vec![
        (vec![vec![2, 4, 3], vec![6, 5, 2]], true),
        (vec![vec![1, 2, 1], vec![1, 2, 1]], false),
        (vec![vec![1, 1, 2]], false),
        (vec![vec![1, 1, 1, 1, 1, 1, 3]], true),
        (
            vec![
                vec![2],
                vec![2],
                vec![2],
                vec![2],
                vec![2],
                vec![2],
                vec![6],
            ],
            true,
        ),
    ];

    for (grid, expected) in tests {
        assert_eq!(Solution::has_valid_path(grid), expected);
    }
}
