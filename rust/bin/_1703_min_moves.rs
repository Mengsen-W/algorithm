/*
 * @Date: 2022-12-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-18
 * @FilePath: /algorithm/1703_min_moves/min_moves.rs
 */

pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut g = vec![];
    let mut pre_sum = vec![0];

    for i in 0..nums.len() {
        if nums[i] == 1 {
            g.push(i - g.len());
            pre_sum.push(*pre_sum.last().unwrap() + *g.last().unwrap() as i32);
        }
    }

    let (m, mut res) = (g.len(), i32::MAX);

    for i in 0..=m - k {
        let mid = i + k / 2;
        res = res.min(
            (1 - k % 2) as i32 * g[mid as usize] as i32
                + (pre_sum[(i + k) as usize] - pre_sum[mid + 1]) as i32
                - (pre_sum[mid] - pre_sum[i]) as i32,
        )
    }
    res
}

fn main() {
    {
        let nums = vec![1, 0, 0, 1, 0, 1];
        let k = 2;
        let ans = 1;
        assert_eq!(min_moves(nums, k), ans);
    }

    {
        let nums = vec![1, 0, 0, 0, 0, 0, 1, 1];
        let k = 3;
        let ans = 5;
        assert_eq!(min_moves(nums, k), ans);
    }

    {
        let nums = vec![1, 1, 0, 1];
        let k = 2;
        let ans = 0;
        assert_eq!(min_moves(nums, k), ans);
    }
}
