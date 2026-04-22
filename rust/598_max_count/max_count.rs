struct Solution;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut mina = m;
        let mut minb = n;
        for op in ops {
            mina = mina.min(op[0]);
            minb = minb.min(op[1]);
        }
        mina * minb
    }
}

fn main() {
    let tests = vec![
        (3, 3, vec![vec![2, 2], vec![3, 3]], 4),
        (
            3,
            3,
            vec![
                vec![2, 2],
                vec![3, 3],
                vec![3, 3],
                vec![3, 3],
                vec![2, 2],
                vec![3, 3],
                vec![3, 3],
                vec![3, 3],
                vec![2, 2],
                vec![3, 3],
                vec![3, 3],
                vec![3, 3],
            ],
            4,
        ),
    ];

    for (m, n, ops, ans) in tests {
        assert_eq!(Solution::max_count(m, n, ops), ans);
    }
}
