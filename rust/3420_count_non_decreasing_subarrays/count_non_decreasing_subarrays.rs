struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut ans: i64 = 0;
        let mut operations: i64 = 0;
        // 优先队列维护窗口最大值
        let mut q: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        // 记录被该下标覆盖的元素集合
        let mut cnt: Vec<Vec<usize>> = vec![Vec::new(); n];
        // 右侧第一个大于等于的下标
        let mut pos: Vec<usize> = vec![n; n];
        // 递减单调栈
        let mut st: Vec<usize> = Vec::new();

        let mut left = 0usize;
        for right in 0..n {
            // 维护单调栈
            while let Some(&top) = st.last() {
                if nums[right] >= nums[top] {
                    pos[top] = right;
                    st.pop();
                } else {
                    break;
                }
            }
            // 栈顶即为 nums[right] 的左边最近的更大元素
            if let Some(&top) = st.last() {
                cnt[top].push(right);
            }
            st.push(right);

            // 优先队首元素即为窗口最大值
            q.push((nums[right], right));
            // 窗口加入 nums[right] 操作次数需要增加
            let max_val = q.peek().unwrap().0;
            operations += std::cmp::max(0, (max_val - nums[right]) as i64);

            // 最小操作次数超过 k，此时需要缩小窗口
            while operations > k as i64 {
                // 移除窗口左侧元素 nums[left]，操作次数需要较少
                for &i in &cnt[left] {
                    // 移除贡献
                    let pr = std::cmp::min(pos[i], right + 1);
                    operations -= ((nums[left] - nums[i]) as i64) * ((pr - i) as i64);
                }
                left += 1;
            }

            // 从优先队列中移除小于下标 left 的元素
            while let Some(&(_, idx)) = q.peek() {
                if idx < left {
                    q.pop();
                } else {
                    break;
                }
            }

            // 以 right 为结尾的合法连续子数组数目为 right - left + 1
            ans += (right - left + 1) as i64;
        }

        ans
    }
}

fn main() {
    let tests = vec![
        (vec![6, 3, 1, 2, 4, 4], 7, 17),
        (vec![6, 3, 1, 3, 6], 4, 12),
    ];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::count_non_decreasing_subarrays(nums, k), expected);
    }
}
