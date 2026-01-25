struct Solution;

impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        nums.sort();
        nums.iter()
            .enumerate()
            .fold(nums[k - 1] - nums[0], |ret, (i, v)| {
                if i >= k {
                    ret.min(v - nums[i - k + 1])
                } else {
                    ret
                }
            })
    }
}

fn main() {
    let tests = vec![(vec![90], 1, 0), (vec![9, 4, 7, 1], 2, 2)];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::minimum_difference(nums, k), expected);
    }
}
