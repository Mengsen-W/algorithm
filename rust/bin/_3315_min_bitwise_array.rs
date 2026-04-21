struct Solution;

impl Solution {
    pub fn min_bitwise_array(mut nums: Vec<i32>) -> Vec<i32> {
        nums.iter_mut().for_each(|p| {
            let x = *p;
            match x {
                2 => *p = -1,
                _ => *p ^= (!x & (x + 1)) >> 1,
            }
        });
        nums
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
