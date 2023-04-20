/*
 * @Date: 2023-04-20
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-04-20
 * @FilePath: /algorithm/rust/1043_max_sum_after_partitioning/max_sum_after_partitioning.rs
 */

pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    let (n, mut dp) = (arr.len(), vec![0; k as usize]);
    for i in 1..=n {
        let (mut curr, mut maximum) = (0, 0);
        for j in 1..=(i.min(k as usize)) {
            curr = curr.max(arr[i - j]);
            maximum = maximum.max(dp[(i - j) % k as usize] + curr * j as i32);
        }
        dp[i % k as usize] = maximum;
    }
    dp[n % k as usize]
}

fn main() {
    {
        let arr = vec![1, 15, 7, 9, 2, 5, 10];
        let k = 3;
        let ans = 84;
        assert_eq!(max_sum_after_partitioning(arr, k), ans);
    }

    {
        let arr = vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3];
        let k = 4;
        let ans = 83;
        assert_eq!(max_sum_after_partitioning(arr, k), ans);
    }

    {
        let arr = vec![1];
        let k = 1;
        let ans = 1;
        assert_eq!(max_sum_after_partitioning(arr, k), ans);
    }
}
