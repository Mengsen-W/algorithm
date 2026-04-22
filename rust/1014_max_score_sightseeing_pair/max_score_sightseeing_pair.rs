struct Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut mx = values[0];
        for j in 1..values.len() {
            ans = ans.max(mx + values[j] - j as i32);
            // 边遍历边维护
            mx = mx.max(values[j] + j as i32);
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![8, 1, 5, 2, 6], 11), (vec![1, 2], 2)];

    for (values, ans) in tests {
        assert_eq!(Solution::max_score_sightseeing_pair(values), ans);
    }
}
