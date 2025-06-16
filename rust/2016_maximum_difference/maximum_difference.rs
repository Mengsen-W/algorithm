struct Solution;

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((-1, i32::MAX), |(mut ans, mut cur_min), num| {
                if cur_min < *num {
                    ans = ans.max(*num - cur_min);
                }
                cur_min = cur_min.min(*num);
                (ans, cur_min)
            })
            .0
    }
}

fn main() {
    let tests = vec![
        (vec![7, 1, 5, 4], 4),
        (vec![9, 4, 3, 2], -1),
        (vec![1, 5, 2, 10], 9),
    ];

    for (nums, expected) in tests {
        assert_eq!(Solution::maximum_difference(nums), expected);
    }
}
