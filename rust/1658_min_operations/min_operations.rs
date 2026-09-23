struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let n = nums.len();
        let target = nums.iter().sum::<i32>() - x;

        let (mut l, mut r) = (0, 0);
        let (mut sum, mut max) = (0, -1);
        while l <= r && r <= n {
            if sum == target {
                max = max.max((r - l) as i32);
            }

            if sum <= target {
                sum += nums.get(r).unwrap_or(&0);
                r += 1;
            } else {
                sum -= nums.get(l).unwrap_or(&0);
                l += 1;
            }
        }

        if max != -1 {
            return n as i32 - max;
        }
        -1
    }
}

fn main() {
    let tests = vec![
        (vec![1, 1, 4, 2, 3], 5, 2),
        (vec![5, 6, 7, 8, 9], 4, -1),
        (vec![3, 2, 20, 1, 1, 3], 10, 5),
    ];

    for (nums, x, ans) in tests {
        assert_eq!(Solution::min_operations(nums, x), ans);
    }
}
