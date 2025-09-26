struct Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        let mut ans: i32 = 0;
        for i in 0..n {
            let mut k = i;
            for j in (i + 1)..n {
                while k + 1 < n && nums[k + 1] < nums[i] + nums[j] {
                    k += 1;
                }
                ans += 0.max(k as i32 - j as i32);
            }
        }
        ans
    }
}

fn main() {
    let tests = vec![(vec![2, 2, 3, 4], 3), (vec![4, 2, 3, 4], 4)];

    for (nums, expected) in tests {
        assert_eq!(Solution::triangle_number(nums), expected);
    }
}
