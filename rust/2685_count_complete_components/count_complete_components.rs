struct Solution;

struct DSU {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let mut rank = vec![1; n];
        for i in 0..n {
            parent[i] = i;
        }
        Self { parent, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.find(p);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }
        if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else if self.rank[ry] > self.rank[rx] {
            self.parent[rx] = ry;
        } else {
            self.parent[rx] = ry;
            self.rank[ry] += 1;
        }
    }
}

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut dsu = DSU::new(n);

        for edge in &edges {
            dsu.union(edge[0] as usize, edge[1] as usize);
        }

        let mut num_v = vec![0; n];
        let mut num_e = vec![0; n];
        for i in 0..n {
            num_v[dsu.find(i)] += 1;
        }
        for edge in &edges {
            num_e[dsu.find(edge[0] as usize)] += 1;
        }

        let mut ans = 0;
        for i in 0..n {
            if dsu.find(i) == i && num_e[i] == num_v[i] * (num_v[i] - 1) / 2 {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    let test = vec![
        (6, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]], 3),
        (
            6,
            vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]],
            1,
        ),
    ];

    for (n, edges, expected) in test {
        assert_eq!(Solution::count_complete_components(n, edges), expected);
    }
}
