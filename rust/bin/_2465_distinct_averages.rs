/*
 * @Date: 2023-06-04
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-04
 * @FilePath: /algorithm/rust/2465_distinct_averages/distinct_averages.rs
 */

pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    (0..n / 2)
        .zip((n / 2..n).rev())
        .map(|(l, r)| nums[l] + nums[r])
        .collect::<std::collections::HashSet<_>>()
        .len() as i32
}

fn main() {
    {
        let nums = vec![4, 1, 4, 0, 3, 5];
        let ans = 2;
        assert_eq!(distinct_averages(nums), ans);
    }

    {
        let nums = vec![1, 100];
        let ans = 1;
        assert_eq!(distinct_averages(nums), ans);
    }
}
