struct Solution;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![pivot; n];
        let mut left = 0;
        let mut right = n - 1;

        for i in 0..n {
            if nums[i] < pivot {
                ans[left] = nums[i];
                left += 1;
            } else if nums[i] > pivot {
                ans[right] = nums[i];
                if right > 0 {
                    right -= 1;
                }
            }
        }

        ans[right + 1..].reverse();
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![9, 12, 5, 10, 14, 3, 10],
            10,
            vec![9, 5, 3, 10, 10, 12, 14],
        ),
        (vec![-3, 4, 3, 2], 2, vec![-3, 2, 4, 3]),
    ];

    for (nums, pivot, expected) in tests {
        assert_eq!(Solution::pivot_array(nums, pivot), expected);
    }
}
