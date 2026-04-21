struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let nums = nums.repeat(2);
        let mut answer = vec![0; n];
        for i in 0..n {
            let mut j = i + 1;
            while j < n * 2 && nums[i] >= nums[j] {
                j += 1;
            }
            if j == n * 2 {
                answer[i] = -1;
            } else {
                answer[i] = nums[j];
            }
        }
        answer
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 1], vec![2, -1, 2]),
        (vec![1, 2, 3, 4, 3], vec![2, 3, 4, -1, 4]),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::next_greater_elements(nums), ans);
    }
}
