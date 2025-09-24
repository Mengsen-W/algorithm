struct Solution;

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;
        let mut bins = Vec::new();
        let mut n = n;
        let mut rep = 1;
        while n > 0 {
            if n % 2 == 1 {
                bins.push(rep);
            }
            n /= 2;
            rep *= 2;
        }

        let m = bins.len();
        let mut results = vec![vec![0; m]; m];
        for i in 0..m {
            let mut cur: i64 = 1;
            for j in i..m {
                cur = (cur * bins[j]) % MOD;
                results[i][j] = cur as i32;
            }
        }

        queries
            .iter()
            .map(|query| results[query[0] as usize][query[1] as usize])
            .collect()
    }
}

fn main() {
    let tests = vec![
        (15, vec![vec![0, 1], vec![2, 2], vec![0, 3]], vec![2, 4, 64]),
        (2, vec![vec![0, 0]], vec![2]),
    ];

    for (n, queries, expected) in tests {
        assert_eq!(Solution::product_queries(n, queries), expected);
    }
}
