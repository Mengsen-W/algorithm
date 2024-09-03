struct Solution;

impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        let mut negative_count = 0;
        let mut zero_count = 0;
        let mut positive_count = 0;
        let mut prod: i64 = 1;
        let mut max_negative = i32::MIN;

        for &num in &nums {
            if num < 0 {
                negative_count += 1;
                prod *= num as i64;
                if num > max_negative {
                    max_negative = num;
                }
            } else if num == 0 {
                zero_count += 1;
            } else {
                prod *= num as i64;
                positive_count += 1;
            }
        }

        if negative_count == 1 && zero_count == 0 && positive_count == 0 {
            return nums[0] as i64;
        }
        if negative_count <= 1 && positive_count == 0 {
            return 0;
        }
        if prod < 0 {
            prod / (max_negative as i64)
        } else {
            prod as i64
        }
    }
}

fn main() {
    let tests = vec![(vec![3, -1, -5, 2, 5, -9], 1350), (vec![-4, -5, -4], 20)];

    for (nums, ans) in tests {
        assert_eq!(Solution::max_strength(nums), ans);
    }
}
