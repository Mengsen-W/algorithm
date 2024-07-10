struct Solution;

impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        let mut l = 1;
        while l < n && nums[l - 1] < nums[l] {
            l += 1;
        }
        res += l as i32 + if l < n { 1 } else { 0 };
        for r in (0..n - 1).rev() {
            while l > 0 && nums[l - 1] >= nums[r + 1] {
                l -= 1;
            }
            res += l as i32 + if l <= r { 1 } else { 0 };
            if nums[r] >= nums[r + 1] {
                break;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4], 10),
        (vec![6, 5, 7, 8], 7),
        (vec![8, 7, 6, 6], 3),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::incremovable_subarray_count(nums), ans);
    }
}
