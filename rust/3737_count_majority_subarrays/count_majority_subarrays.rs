struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        for i in 0..n {
            let mut cnt = 0;
            for j in i..n {
                cnt += if nums[j] == target { 1 } else { -1 };
                if cnt > 0 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 2, 3], 2, 5),
        (vec![1, 1, 1, 1], 1, 10),
        (vec![1, 2, 3], 4, 0),
    ];

    for (nums, target, expected) in tests {
        assert_eq!(Solution::count_majority_subarrays(nums, target), expected);
    }
}
