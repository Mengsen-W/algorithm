struct Solution;

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        for i in 0..n {
            if Self::is_prime(nums[i][i]) {
                res = res.max(nums[i][i]);
            }
            if Self::is_prime(nums[i][n - i - 1]) {
                res = res.max(nums[i][n - i - 1]);
            }
        }
        res
    }

    fn is_prime(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        let mut factor = 2;
        while factor * factor <= num {
            if num % factor == 0 {
                return false;
            }
            factor += 1;
        }
        true
    }
}

fn main() {
    let tests = vec![
        (vec![vec![1, 2, 3], vec![5, 6, 7], vec![9, 10, 11]], 11),
        (vec![vec![1, 2, 3], vec![5, 17, 7], vec![9, 11, 10]], 17),
    ];

    for (nums, ans) in tests {
        assert_eq!(Solution::diagonal_prime(nums), ans);
    }
}
