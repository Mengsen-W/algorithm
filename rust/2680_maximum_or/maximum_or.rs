struct Solution;

impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let mut or_sum = 0i64;
        let mut multi_bits = 0i64;
        for &x in &nums {
            multi_bits |= or_sum & (x as i64);
            or_sum |= x as i64;
        }
        let mut res = 0;
        for &x in &nums {
            let val = x as i64;
            res = res.max((or_sum ^ val) | (val << (k as i64)) | multi_bits);
        }
        res
    }
}

fn main() {
    let tests = vec![(vec![12, 9], 1, 30), (vec![8, 1, 2], 2, 35)];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::maximum_or(nums, k), expected);
    }
}
