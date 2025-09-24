struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut p0 = 0;
        let mut p1 = 0;
        for &num in nums.iter() {
            if num == 0 {
                p1 = p0;
                p0 = 0;
            } else {
                p0 += 1;
                p1 += 1;
            }
            ans = ans.max(p1);
        }
        if ans == nums.len() as i32 {
            ans -= 1;
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (vec![1, 1, 0, 1], 3),
        (vec![0, 1, 1, 1, 0, 1, 1, 0, 1], 5),
        (vec![1, 1, 1], 2),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::longest_subarray(nums), ans);
    }
}
