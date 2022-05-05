/*
 * @Date: 2022-05-05 09:40:14
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2022-05-05 09:47:47
 * @FilePath: /algorithm/713_num_subarray_product_less_than_k/num_subarray_product_less_than_k.rs
 */

pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    (0..nums.len())
        .fold((0, 1, 0), |(mut i, mut prev, cnt), j| {
            prev *= nums[j];
            while i <= j && prev >= k {
                prev /= nums[i];
                i += 1;
            }
            (i, prev, cnt + (j as i32 - i as i32) + 1)
        })
        .2
}

fn main() {
    assert_eq!(num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100), 8);
    assert_eq!(num_subarray_product_less_than_k(vec![1, 2, 3], 0), 0);
}
