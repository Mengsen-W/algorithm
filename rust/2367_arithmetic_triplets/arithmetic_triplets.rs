/*
 * @Date: 2023-03-31
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-31
 * @FilePath: /algorithm/rust/2367_arithmetic_triplets/arithmetic_triplets.rs
 */

pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    use std::cmp::max;
    let mut ans = 0;
    let n = nums.len();
    let (mut i, mut j, mut k) = (0, 1, 2);
    while i < n - 2 && j < n - 1 && k < n {
        j = max(j, i + 1);
        while j < n - 1 && nums[j] - nums[i] < diff {
            j += 1;
        }
        if j >= n - 1 || nums[j] - nums[i] > diff {
            i += 1;
            continue;
        }
        k = max(k, j + 1);
        while k < n && nums[k] - nums[j] < diff {
            k += 1;
        }
        if k < n && nums[k] - nums[j] == diff {
            ans += 1;
        }
        i += 1;
    }
    return ans;
}

fn main() {
    {
        let nums = vec![0, 1, 4, 6, 7, 10];
        let diff = 3;
        let ans = 2;
        assert_eq!(arithmetic_triplets(nums, diff), ans);
    }

    {
        let nums = vec![4, 5, 6, 7, 8, 9];
        let diff = 2;
        let ans = 2;
        assert_eq!(arithmetic_triplets(nums, diff), ans);
    }
}
