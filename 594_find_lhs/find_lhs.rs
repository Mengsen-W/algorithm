/*
 * @Date: 2021-11-20 00:40:32
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-20 00:54:29
 */

struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let cnt: std::collections::HashMap<i32, i32> =
            nums.iter()
                .fold(std::collections::HashMap::new(), |mut acc, &x| {
                    *acc.entry(x).or_insert(0) += 1;
                    acc
                });
        let mut res = 0;
        for (k, v) in &cnt {
            if let Some(v2) = cnt.get(&(k + 1)) {
                res = std::cmp::max(res, v + v2);
            }
        }
        res
    }
}

fn main() {
    assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
    assert_eq!(Solution::find_lhs(vec![1, 2, 3, 4]), 2);
    assert_eq!(Solution::find_lhs(vec![1, 1, 1, 1]), 0);
}
