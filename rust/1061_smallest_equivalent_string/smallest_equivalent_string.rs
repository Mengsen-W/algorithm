struct Solution;

struct UnionFind {
    f: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let f = (0..n).collect();
        Self { f }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.f[x] != x {
            self.f[x] = self.find(self.f[x]);
        }
        self.f[x]
    }

    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return;
        }
        // 总是让字典序更小的作为集合代表字符
        if x > y {
            std::mem::swap(&mut x, &mut y);
        }
        self.f[y] = x;
    }
}

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut uf = UnionFind::new(26);
        for (a, b) in s1.bytes().zip(s2.bytes()) {
            uf.unite((a - b'a') as usize, (b - b'a') as usize);
        }

        base_str
            .bytes()
            .map(|c| {
                let rep = uf.find((c - b'a') as usize);
                (b'a' + rep as u8) as char
            })
            .collect()
    }
}

fn main() {
    let tests = vec![
        ("parker", "morris", "parser", "makkek"),
        ("hello", "world", "hold", "hdld"),
        ("leetcode", "programs", "sourcecode", "aauaaaaada"),
    ];

    for (s1, s2, base_str, expected) in tests {
        assert_eq!(
            Solution::smallest_equivalent_string(
                s1.to_string(),
                s2.to_string(),
                base_str.to_string()
            ),
            expected.to_string()
        );
    }
}
