struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        if n <= 2 {
            return n;
        }
        let mut ans = 1;
        while ans <= n {
            ans <<= 1;
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 2], 2), (vec![3, 1, 2], 4)];

    for (test, expected) in tests {
        assert_eq!(Solution::unique_xor_triplets(test), expected);
    }
}
