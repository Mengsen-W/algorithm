struct Solution;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut f = vec![-1; (k + 1) as usize];
        f[0] = 0;
        for pile in piles {
            for i in (1..=k as usize).rev() {
                let mut value = 0;
                for t in 1..=pile.len() {
                    value += pile[t - 1];
                    if i >= t && f[i - t] != -1 {
                        f[i] = f[i].max(f[i - t] + value);
                    }
                }
            }
        }
        f[k as usize]
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 100, 3], vec![7, 8, 9]], 2, 101),
        (
            vec![
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![100],
                vec![1, 1, 1, 1, 1, 1, 700],
            ],
            7,
            706,
        ),
    ];

    for (piles, k, ans) in tests {
        assert_eq!(Solution::max_value_of_coins(piles, k), ans);
    }
}
