struct Solution;

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut f0: i64 = 0;
        let mut f1: i64 = i64::MIN;
        for &x in &nums {
            let xk = (x ^ k) as i64;
            let x = x as i64;
            let t = f1 + x;
            let alt = f0 + xk;
            let new_f0 = (f0 + x).max(f1 + xk);
            f1 = t.max(alt);
            f0 = new_f0;
        }
        f0
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 1], 3, vec![vec![0, 1], vec![0, 2]], 6),
        (vec![2, 3], 7, vec![vec![0, 1]], 9),
        (
            vec![7, 7, 7, 7, 7, 7],
            3,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]],
            42,
        ),
    ];

    for (nums, k, edges, expected) in tests {
        assert_eq!(Solution::maximum_value_sum(nums, k, edges), expected);
    }
}
