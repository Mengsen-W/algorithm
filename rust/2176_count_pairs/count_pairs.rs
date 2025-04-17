struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut res = 0; // 符合要求数对个数
        for i in 0..n - 1 {
            for j in i + 1..n {
                if (i * j) % k as usize == 0 && nums[i] == nums[j] {
                    res += 1;
                }
            }
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![3, 1, 2, 2, 2, 1, 3], 2, 4), (vec![1, 2, 3, 4], 1, 0)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::count_pairs(nums, k), ans);
    }
}
