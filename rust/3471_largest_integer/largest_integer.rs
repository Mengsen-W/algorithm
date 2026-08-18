struct Solution;

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        if n == k {
            return *nums.iter().max().unwrap();
        }
        let mut count = [0; 51];
        for &x in &nums {
            count[x as usize] += 1;
        }
        if k == 1 {
            for i in (0..=50).rev() {
                if count[i as usize] == 1 {
                    return i;
                }
            }
            return -1;
        }
        let mut res = -1;
        if count[nums[0] as usize] == 1 {
            res = res.max(nums[0]);
        }
        if count[nums[n - 1] as usize] == 1 {
            res = res.max(nums[n - 1]);
        }
        res
    }
}

fn main() {
    let tests = vec![
        (vec![3, 9, 2, 1, 7], 3, 7),
        (vec![3, 9, 7, 2, 1, 7], 4, 3),
        (vec![0, 0], 1, -1),
    ];

    for (nums, k, ans) in tests {
        assert_eq!(Solution::largest_integer(nums, k), ans);
    }
}
