struct Solution;

impl Solution {
    pub fn minimize_sum(nums: Vec<i32>) -> i32 {
        let mut mn1 = i32::MAX;
        let mut mn2 = i32::MAX;
        let mut mn3 = i32::MAX;
        let mut mx1 = i32::MIN;
        let mut mx2 = i32::MIN;
        let mut mx3 = i32::MIN;

        for &x in &nums {
            if x < mn1 {
                mn3 = mn2;
                mn2 = mn1;
                mn1 = x;
            } else if x < mn2 {
                mn3 = mn2;
                mn2 = x;
            } else if x < mn3 {
                mn3 = x;
            }

            if x > mx1 {
                mx3 = mx2;
                mx2 = mx1;
                mx1 = x;
            } else if x > mx2 {
                mx3 = mx2;
                mx2 = x;
            } else if x > mx3 {
                mx3 = x;
            }
        }

        (mx1 - mn3).min(mx2 - mn2).min(mx3 - mn1)
    }
}

fn main() {
    let tests = vec![(vec![1, 4, 3], 0), (vec![1, 4, 7, 8, 5], 3)];

    for (nums, expected) in tests {
        assert_eq!(Solution::minimize_sum(nums), expected);
    }
}
