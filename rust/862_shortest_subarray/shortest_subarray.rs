/*
 * @Date: 2022-10-26
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-26
 * @FilePath: /algorithm/862_shortest_subarray/shortest_subarray.rs
 */

pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut pre_sum = vec![0; n + 1];

    for i in 0..n {
        pre_sum[i + 1] = pre_sum[i] + nums[i] as i64;
    }

    let mut ans = n + 1;
    let mut queue = std::collections::VecDeque::new();

    for i in 0..=n {
        while let Some(&j) = queue.back() {
            if pre_sum[i] <= pre_sum[j] {
                queue.pop_back();
            } else {
                break;
            }
        }
        while let Some(&j) = queue.front() {
            if pre_sum[i] >= pre_sum[j] + k as i64 {
                ans = ans.min(i - j);
                queue.pop_front();
            } else {
                break;
            }
        }

        queue.push_back(i);
    }

    if ans == n + 1 {
        -1
    } else {
        ans as i32
    }
}

fn main() {
    {
        let nums = vec![1];
        let k = 1;
        let ans = 1;
        assert_eq!(shortest_subarray(nums, k), ans);
    }

    {
        let nums = vec![1, 2];
        let k = 4;
        let ans = -1;
        assert_eq!(shortest_subarray(nums, k), ans);
    }

    {
        let nums = vec![2, -1, 2];
        let k = 3;
        let ans = 3;
        assert_eq!(shortest_subarray(nums, k), ans);
    }
}
