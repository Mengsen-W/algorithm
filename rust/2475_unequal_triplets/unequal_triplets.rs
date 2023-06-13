/*
 * @Date: 2023-06-13
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-13
 * @FilePath: /algorithm/rust/2475_unequal_triplets/unequal_triplets.rs
 */

pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
    let count = nums
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, x| {
            (*acc.entry(x).or_insert(0)) += 1;
            acc
        });
    let (mut res, n, mut t) = (0, nums.len(), 0);
    for v in count.values() {
        res = res + (t * v * (n - t - v));
        t = t + v;
    }
    res as i32
}

fn main() {
    {
        let nums = vec![4, 4, 2, 4, 3];
        let ans = 3;
        assert_eq!(unequal_triplets(nums), ans);
    }

    {
        let nums = vec![1, 1, 1, 1, 1];
        let ans = 0;
        assert_eq!(unequal_triplets(nums), ans);
    }
}
