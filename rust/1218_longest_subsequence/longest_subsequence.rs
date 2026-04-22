/*
 * @Date: 2021-11-06 00:36:11
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-11-06 00:55:46
 */

struct Solution;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut vis = std::collections::HashMap::new();
        arr.iter().fold(0, |ans, x| {
            let k = *vis.get(&(x - difference)).unwrap_or(&0);
            i32::max(
                *vis.entry(x)
                    .and_modify(|v| {
                        if *v < k + 1 {
                            *v = k + 1
                        }
                    })
                    .or_insert(k + 1),
                ans,
            )
        })
    }
}

fn main() {
    {
        let arr = vec![1, 2, 3, 4];
        let difference = 1;
        assert_eq!(Solution::longest_subsequence(arr, difference), 4);
    }
    {
        let arr = vec![1, 3, 5, 7];
        let difference = 1;
        assert_eq!(Solution::longest_subsequence(arr, difference), 1);
    }
    {
        let arr = vec![1, 5, 7, 8, 5, 3, 4, 2, 1];
        let difference = -2;
        assert_eq!(Solution::longest_subsequence(arr, difference), 4);
    }
}
