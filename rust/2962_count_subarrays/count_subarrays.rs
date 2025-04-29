struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mx = *nums.iter().max().unwrap();
        let mut ans: i64 = 0;
        let mut cnt = 0;
        let mut left = 0;

        for &x in &nums {
            if x == mx {
                cnt += 1;
            }
            while cnt == k {
                if nums[left] == mx {
                    cnt -= 1;
                }
                left += 1;
            }
            ans += left as i64;
        }

        ans
    }
}

fn main() {
    let tests = vec![(vec![1, 3, 2, 3, 3], 2, 6), (vec![1, 4, 2, 1], 3, 0)];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::count_subarrays(nums, k), ans);
    }
}
