struct Solution;

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums.clone();
        for x in nums.iter_mut() {
            let mut res = -1;
            let mut d = 1;
            let val = *x;
            while (val & d) != 0 {
                res = val - d;
                d <<= 1;
            }
            *x = res;
        }
        nums.clone()
    }
}

fn main() {
    let tests = vec![
        (vec![2, 3, 5, 7], vec![-1, 1, 4, 3]),
        (vec![11, 13, 31], vec![9, 12, 15]),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::min_bitwise_array(nums), expected);
    }
}
