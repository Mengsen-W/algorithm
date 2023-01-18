/*
 * @Date: 2021-10-06 10:12:13
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-10-06 10:43:24
 */

struct Solution;
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let (mut a, mut b, mut c) = (None, None, None);
        for num in nums {
            let n = Some(num);
            if n >= a {
                if n != a {
                    c = b;
                    b = a;
                    a = n;
                }
            } else if n >= b {
                if n != b {
                    c = b;
                    b = n;
                }
            } else if n > c {
                c = n
            }
        }
        c.map_or(a.unwrap_or(0), |v| v)
    }
}

fn main() {
    {
        let nums = vec![3, 2, 1];
        let ans = 1;
        assert_eq!(Solution::third_max(nums), ans);
    }
    {
        let nums = vec![2, 1];
        let ans = 2;
        assert_eq!(Solution::third_max(nums), ans);
    }
    {
        let nums = vec![2, 2, 3, 1];
        let ans = 1;
        assert_eq!(Solution::third_max(nums), ans);
    }
}
