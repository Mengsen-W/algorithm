struct Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }

        if nums[0] >= nums[1] {
            return false;
        }

        let mut cnt = 1;
        for i in 2..nums.len() {
            if nums[i - 1] == nums[i] {
                return false;
            }

            let prev_trend = nums[i - 2] < nums[i - 1];
            let curr_trend = nums[i - 1] < nums[i];

            if prev_trend != curr_trend {
                cnt += 1;
            }
        }

        cnt == 3
    }
}

fn main() {
    let tests = vec![(vec![1, 3, 5, 4, 2, 6], true), (vec![2, 1, 3], false)];

    for (nums, ans) in tests {
        assert_eq!(Solution::is_trionic(nums), ans);
    }
}
