/*
 * @Date: 2021-08-07 13:27:19
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-08-07 14:19:11
 */

struct Solution;

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        Self::circular_array_loop_double_pointer(nums)
        // Self::circular_array_loop_dfs(nums)
    }

    fn circular_array_loop_double_pointer(mut nums: Vec<i32>) -> bool {
        let n = nums.len();
        for i in 0..n {
            if nums[i] == 0 {
                continue;
            }
            let next = |i| ((i as i32 + nums[i]) % n as i32 + n as i32) as usize % n;
            let (mut slow, mut fast) = (i, next(i));
            while nums[slow] * nums[fast] > 0 && nums[slow] * nums[next(fast)] > 0 {
                if slow == fast {
                    if slow != next(slow) {
                        return true;
                    }
                    break;
                }
                slow = next(slow);
                fast = next(next(fast));
            }
            let mut add = i;
            loop {
                let next = |i| ((i as i32 + nums[i]) % n as i32 + n as i32) as usize % n;
                if nums[add] * nums[next(add)] <= 0 {
                    break;
                }
                let t = add;
                add = next(add);
                nums[t] = 0;
            }
        }
        false
    }

    fn circular_array_loop_dfs(mut nums: Vec<i32>) -> bool {
        const OFFSET: i32 = 100010;
        let n = nums.len();
        for i in 0..n {
            if nums[i] >= OFFSET {
                continue;
            }
            let (mut cur, tag) = (i as i32, OFFSET + i as i32);
            let mut last;
            let flag = nums[cur as usize] > 0;
            let n = n as i32;
            loop {
                let next = ((cur + nums[cur as usize]) % n + n) % n;
                last = nums[cur as usize];
                nums[cur as usize] = tag;
                cur = next;
                if cur == i as i32 {
                    break;
                }
                let cur = cur as usize;
                if nums[cur] >= OFFSET {
                    break;
                }
                if flag && nums[cur] < 0 {
                    break;
                }
                if !flag && nums[cur] > 0 {
                    break;
                }
            }
            if last % n != 0 && nums[cur as usize] == tag {
                return true;
            }
        }
        false
    }
}

fn main() {
    {
        let nums = vec![2, -1, 1, 2, 2];
        assert!(Solution::circular_array_loop_dfs(nums.clone()));
        assert!(Solution::circular_array_loop_double_pointer(nums.clone()));
    }
    {
        let nums = vec![-1, 2];
        assert!(!Solution::circular_array_loop_dfs(nums.clone()));
        assert!(!Solution::circular_array_loop_double_pointer(nums.clone()));
    }
    {
        let nums = vec![2, -1, 1, -2, -2];
        assert!(!Solution::circular_array_loop_dfs(nums.clone()));
        assert!(!Solution::circular_array_loop_double_pointer(nums.clone()));
    }
}
