struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        use rand::{thread_rng, Rng};
        let n = nums.len();
        loop {
            let (x, y) = (thread_rng().gen_range(0, n), thread_rng().gen_range(0, n));
            if x != y && nums[x] == nums[y] {
                return nums[x];
            }
        }
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3], 3),
        (vec![2, 1, 2, 5, 3, 2], 2),
        (vec![5, 1, 5, 2, 5, 3, 5, 4], 5),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::repeated_n_times(nums), ans);
    }
}
