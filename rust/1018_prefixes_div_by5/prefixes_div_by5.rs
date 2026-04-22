struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut ans = vec![false; nums.len()];
        let mut x = 0;
        for (i, bit) in nums.into_iter().enumerate() {
            x = (x << 1 | bit) % 5;
            ans[i] = x == 0;
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![0, 1, 1], vec![true, false, false]),
        (vec![1, 1, 1], vec![false, false, false]),
    ];

    for (nums, expect) in tests {
        assert_eq!(Solution::prefixes_div_by5(nums), expect,);
    }
}
