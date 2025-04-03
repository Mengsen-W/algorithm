struct Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut imax = 0;
        let mut dmax = 0;
        for &num in nums.iter() {
            res = res.max(dmax * num as i64);
            dmax = dmax.max(imax - num as i64);
            imax = imax.max(num as i64);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![12, 6, 1, 2, 7], 77),
        (vec![1, 10, 3, 4, 19], 133),
        (vec![1, 2, 3], 0),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::maximum_triplet_value(nums), ans);
    }
}
