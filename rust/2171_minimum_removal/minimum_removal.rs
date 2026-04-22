/*
 * @Date: 2024-01-18
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-01-18
 * @FilePath: /algorithm/rust/2171_minimum_removal/minimum_removal.rs
 */

struct Solution;

impl Solution {
    pub fn minimum_removal(mut beans: Vec<i32>) -> i64 {
        beans.sort_unstable();
        let mut sum = 0i64;
        let mut mx = 0i64;
        let n = beans.len();
        for i in 0..n {
            let v = beans[i] as i64;
            sum += v;
            mx = mx.max((n - i) as i64 * v);
        }
        sum - mx
    }
}

fn main() {
    let test = vec![(vec![4, 1, 6, 5], 4), (vec![2, 10, 3, 2], 7)];

    for (beans, ans) in test {
        assert_eq!(Solution::minimum_removal(beans), ans);
    }
}
