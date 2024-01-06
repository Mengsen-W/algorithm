/*
 * @Date: 2024-01-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-05
 * @FilePath: /algorithm/rust/1944_can_see_persons_count/can_see_persons_count.rs
 */

struct Solution;
impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut ans = vec![0; n];
        let mut st = Vec::new();
        for (i, &h) in heights.iter().enumerate().rev() {
            while !st.is_empty() && *st.last().unwrap() < h {
                st.pop();
                ans[i] += 1;
            }
            if !st.is_empty() {
                // 还可以再看到一个人
                ans[i] += 1;
            }
            st.push(h);
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![10, 6, 8, 5, 11, 9], vec![3, 1, 2, 1, 1, 0]),
        (vec![5, 1, 2, 3, 10], vec![4, 1, 1, 1, 0]),
    ];

    for (heights, ans) in tests {
        assert_eq!(Solution::can_see_persons_count(heights), ans);
    }
}
