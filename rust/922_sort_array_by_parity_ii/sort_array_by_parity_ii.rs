struct Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums;
        let mut j = 1;
        for i in (0..n).step_by(2) {
            if nums[i] % 2 == 1 {
                while nums[j] % 2 == 1 {
                    j += 2;
                }
                nums.swap(i, j);
            }
        }
        nums
    }
}

fn main() {
    let tests = vec![
        (vec![4, 2, 5, 7], vec![4, 5, 2, 7]),
        (vec![2, 3], vec![2, 3]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::sort_array_by_parity_ii(nums), ans);
    }
}
