/*
 * @Date: 2023-06-28
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-06-28
 * @FilePath: /algorithm/rust/1681_minimum_incompatibility/minimum_incompatibility.rs
 */

pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = (n / k as usize) as u32;
    let mut dp = vec![i32::MAX >> 2; 1 << nums.len()];
    if k == 1 {
        return 0;
    }
    for s in 1..(1 << n) as usize {
        if s.count_ones() % k != 0 {
            continue;
        }
        if s.count_ones() == k {
            // 基础情况，只选择了(n/k)个元素
            let mut hs = std::collections::HashSet::new();
            for i in 0..n {
                if (s >> i) & 1 == 1 {
                    hs.insert(nums[i]);
                }
            }
            if hs.len() == k as usize {
                dp[s] = hs.iter().max().unwrap() - hs.iter().min().unwrap();
            }
            continue;
        }
        let mut ss = s;
        while ss != 0 {
            // 子集枚举
            ss = (ss - 1) & s;
            dp[s] = dp[s].min(dp[ss] + dp[s ^ ss]);
        }
    }
    if dp[(1 << n) - 1] >= i32::MAX >> 2 {
        -1
    } else {
        dp[(1 << n) - 1]
    }
}

fn main() {
    let test_map = vec![
        (vec![1, 2, 1, 4], 2, 4),
        (vec![6, 3, 8, 1, 3, 1, 2, 2], 4, 6),
        (vec![5, 3, 3, 6, 3, 3], 3, -1),
    ];
    for item in test_map {
        assert_eq!(minimum_incompatibility(item.0, item.1), item.2);
    }
}
