struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut res = 0 as i64;
        let (mut border, mut last_min, mut last_max) = (-1, -1, -1);
        for (i, &x) in nums.iter().enumerate() {
            if x < min_k || x > max_k {
                border = i as i64;
                last_min = -1 as i64;
                last_max = -1 as i64;
            }
            if x == min_k {
                last_min = i as i64;
            }
            if x == max_k {
                last_max = i as i64;
            }
            if last_min != -1 && last_max != -1 {
                res += last_min.min(last_max) - border;
            }
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 5, 2, 7, 5], 1, 5, 2),
        (vec![1, 1, 1, 1], 1, 1, 10),
    ];

    for (nums, min_k, max_k, ans) in tests {
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), ans);
    }
}
