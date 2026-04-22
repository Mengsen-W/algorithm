/*
 * @Date: 2023-10-19
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-19
 * @FilePath: /algorithm/rust/1726_tuple_same_product/tuple_same_product.rs
 */

struct Solution;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let n = nums.len();
        let mut product = HashMap::new();
        for i in 0..n {
            for j in 0..i {
                product
                    .entry(nums[i] * nums[j])
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }
        product.values().fold(0, |acc, x| acc + x * (x - 1) * 4)
    }
}

fn main() {
    let tests = vec![(vec![2, 3, 4, 6], 8), (vec![1, 2, 4, 5, 10], 16)];

    for (nums, ans) in tests {
        assert_eq!(Solution::tuple_same_product(nums), ans);
    }
}
