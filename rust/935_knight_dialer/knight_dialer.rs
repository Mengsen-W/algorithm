struct Solution;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let moves = vec![
            vec![4, 6],
            vec![6, 8],
            vec![7, 9],
            vec![4, 8],
            vec![3, 9, 0],
            vec![],
            vec![1, 7, 0],
            vec![2, 6],
            vec![1, 3],
            vec![2, 4],
        ];
        let mut d = vec![vec![0; 10], vec![1; 10]];
        for i in 2..=n {
            let x = (i % 2) as usize;
            for j in 0..10 {
                d[x][j] = 0;
                for &k in &moves[j] {
                    d[x][j] = (d[x][j] + d[1 - x][k]) % MOD;
                }
            }
        }
        d[(n % 2) as usize]
            .iter()
            .fold(0, |res, &x| (res + x) % MOD)
    }
}

fn main() {
    let tests = vec![(1, 10), (2, 20), (3131, 136006598)];

    for (n, ans) in tests {
        assert_eq!(Solution::knight_dialer(n), ans);
    }
}
