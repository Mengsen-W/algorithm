/*
 * @Date: 2023-07-05
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-05
 * @FilePath: /algorithm/rust/2600_k_items_with_maximum_sum/k_items_with_maximum_sum.rs
 */

struct Solution;

impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        if k <= num_ones {
            k
        } else if k <= num_ones + num_zeros {
            num_ones
        } else {
            num_ones - (k - num_ones - num_zeros)
        }
    }
}

fn main() {
    let test_map = vec![(3, 2, 0, 2, 2), (3, 2, 0, 4, 3)];

    for item in test_map {
        assert_eq!(
            Solution::k_items_with_maximum_sum(item.0, item.1, item.2, item.3),
            item.4
        );
    }
}
