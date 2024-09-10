struct Solution;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut pre = vec![0; n + 1];
        let mut ans: i64 = 0;

        for j in 0..n {
            let mut suf = 0;
            for k in (j + 1..n).rev() {
                if nums[j] > nums[k] {
                    ans += pre[nums[k] as usize] as i64 * suf as i64;
                } else {
                    suf += 1;
                }
            }
            for x in (nums[j] + 1) as usize..=n {
                pre[x] += 1;
            }
        }

        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 3, 2, 4, 5], 2), (vec![1, 2, 3, 4], 0)];

    for (nums, ans) in tests {
        assert_eq!(Solution::count_quadruplets(nums), ans);
    }
}
