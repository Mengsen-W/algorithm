/*
 * @Date: 2023-03-16
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-16
 * @FilePath: /algorithm/rust/2488_count_subarrays/count_subarrays.rs
 */

pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    nums.into_iter()
        .fold(
            (0, 0, false, std::collections::HashMap::from([(0, 1)])),
            |mut s, x| {
                let c = (x > k) as i32 - (x < k) as i32;
                s.1 += c;
                if c == 0 {
                    s.2 = true
                }
                if s.2 {
                    s.0 += s.3.get(&s.1).copied().unwrap_or(0)
                        + s.3.get(&(s.1 - 1)).copied().unwrap_or(0)
                } else {
                    *s.3.entry(s.1).or_insert(0) += 1
                }
                s
            },
        )
        .0
}

fn main() {
    {
        let nums = vec![3, 2, 1, 4, 5];
        let k = 4;
        let ans = 3;
        assert_eq!(count_subarrays(nums, k), ans);
    }

    {
        let nums = vec![2, 3, 1];
        let k = 3;
        let ans = 1;
        assert_eq!(count_subarrays(nums, k), ans);
    }
}
