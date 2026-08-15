struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut total_xor = 0;
        let mut all_zero = true;

        for &x in &nums {
            total_xor ^= x;
            if x > 0 {
                all_zero = false;
            }
        }

        if total_xor > 0 {
            return n;
        }

        if all_zero {
            0
        } else {
            n - 1
        }
    }
}

fn main() {
    let tests = vec![(vec![1, 2, 3], 2), (vec![2, 3, 4], 3)];

    for (nums, ans) in tests {
        assert_eq!(Solution::longest_subsequence(nums), ans);
    }
}
