struct Solution;

impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut state = vec![0; n];
        let mut pre_max = nums[0];
        for i in 1..n - 1 {
            if nums[i] > pre_max {
                state[i] = 1;
                pre_max = nums[i];
            }
        }
        let mut suf_min = nums[n - 1];
        let mut res = 0;
        for i in (1..n - 1).rev() {
            if state[i] == 1 && nums[i] < suf_min {
                res += 2;
            } else if nums[i - 1] < nums[i] && nums[i] < nums[i + 1] {
                res += 1;
            }
            suf_min = suf_min.min(nums[i]);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3], 2),
        (vec![2, 4, 6, 4], 1),
        (vec![3, 2, 1], 0),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::sum_of_beauties(nums), ans);
    }
}
