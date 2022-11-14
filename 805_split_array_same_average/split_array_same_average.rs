/*
 * @Date: 2022-11-14
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-14
 * @FilePath: /algorithm/805_split_array_same_average/split_array_same_average.rs
 */

pub fn split_array_same_average(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let m = n / 2;
    if n == 1 {
        return false;
    }
    let sum: i32 = nums.iter().sum();
    let nums: Vec<i32> = nums.iter().map(|x| (*x) * n as i32 - sum).collect();

    let mut left = std::collections::HashSet::new();
    let mut i = 1;
    while i < (1 << m) {
        let mut tot = 0;
        for j in 0..m {
            if (i & (1 << j)) != 0 {
                tot += nums[j];
            }
        }
        if tot == 0 {
            return true;
        }
        left.insert(tot);
        i += 1;
    }

    let rsum: i32 = nums[m..].iter().sum();
    let mut i = 1;
    while i < (1 << (n - m)) {
        let mut tot = 0;
        for j in m..n {
            if (i & (1 << (j - m))) != 0 {
                tot += nums[j];
            }
        }
        if tot == 0 || (rsum != tot && left.get(&-tot) != None) {
            return true;
        }
        i += 1;
    }
    false
    // let (sum, n) = (nums.iter().sum::<i32>(), nums.len());
    // let mut dp = vec![0; sum as usize + 1];
    // dp[0] = 1;
    // for num in nums {
    //     for s in (num..=sum).rev() {
    //         if dp[(s - num) as usize] > 0 {
    //             dp[s as usize] |= (dp[(s - num) as usize] << 1);
    //         }
    //     }
    // }
    // for i in 1..n {
    //     if sum * i as i32 % n as i32 != 0 {
    //         continue;
    //     }
    //     let s = sum * i as i32 / n as i32;
    //     if dp[s as usize] > 0 && (dp[s as usize] & (1 << i as i32)) > 0 {
    //         return true;
    //     }
    // }
    // false
}

fn main() {
    {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert!(split_array_same_average(nums));
    }
    {
        let nums = vec![3, 1];
        assert!(!split_array_same_average(nums));
    }
}
