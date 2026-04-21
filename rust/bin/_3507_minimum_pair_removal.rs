struct Solution;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums = nums.clone();
        let mut next: Vec<isize> = (0..n as isize).map(|i| i + 1).collect();
        next[n - 1] = -1;

        let mut count = 0;

        while (n as i32 - count) > 1 {
            let mut curr: isize = 0;
            let mut target: isize = 0;
            let mut target_adj_sum = nums[0] + nums[next[0] as usize];
            let mut is_ascending = true;

            while curr != -1 && next[curr as usize] != -1 {
                let curr_idx = curr as usize;
                let next_idx = next[curr_idx] as usize;

                if nums[curr_idx] > nums[next_idx] {
                    is_ascending = false;
                }

                let curr_adj_sum = nums[curr_idx] + nums[next_idx];
                if curr_adj_sum < target_adj_sum {
                    target = curr;
                    target_adj_sum = curr_adj_sum;
                }
                curr = next[curr_idx];
            }

            if is_ascending {
                break;
            }

            count += 1;
            let target_idx = target as usize;
            let next_target = next[target_idx] as usize;
            next[target_idx] = next[next_target];
            nums[target_idx] = target_adj_sum;
        }

        count
    }
}

fn main() {
    let tests = vec![(vec![5, 2, 3, 1], 2), (vec![1, 2, 2], 0)];

    for (nums, ans) in tests {
        assert_eq!(Solution::minimum_pair_removal(nums), ans);
    }
}
