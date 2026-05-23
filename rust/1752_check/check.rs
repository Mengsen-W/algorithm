struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut a = 2;
        for i in nums.windows(2) {
            if i[0] > i[1] {
                a -= 1;
                if nums[0] < nums[nums.len() - 1] {
                    return false;
                }
                if a == 0 {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let tests = vec![
        (vec![3, 4, 5, 1, 2], true),
        (vec![2, 1, 3, 4], false),
        (vec![1, 2, 3], true),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::check(nums), expected);
    }
}
