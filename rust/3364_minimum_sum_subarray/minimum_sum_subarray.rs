struct Solution;

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let n = nums.len();
        let l = l as usize;
        let r = r as usize;
        let mut min_sum = i32::MAX;

        let mut initial = 0;
        for &num in nums.iter().take(l - 1) {
            initial += num;
        }

        for length in l..=r {
            initial += nums[length - 1];
            let mut sum = initial;

            if sum > 0 && sum < min_sum {
                min_sum = sum;
            }

            for hi in length..n {
                sum += nums[hi] - nums[hi - length];
                if sum > 0 && sum < min_sum {
                    min_sum = sum;
                }
            }
        }

        if min_sum == i32::MAX {
            -1
        } else {
            min_sum
        }
    }
}

fn main() {
    let tests = vec![
        (vec![3, -2, 1, 4], 2, 3, 1),
        (vec![-2, 2, -3, 1], 2, 3, -1),
        (vec![1, 2, 3, 4], 2, 4, 3),
    ];

    for (nums, l, r, expected) in tests {
        assert_eq!(Solution::minimum_sum_subarray(nums, l, r), expected);
    }
}
