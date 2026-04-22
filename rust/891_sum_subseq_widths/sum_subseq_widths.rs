/*
 * @Date: 2022-11-18
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-11-18
 * @FilePath: /algorithm/891_sum_subseq_widths/sum_subseq_widths.rs
 */

pub fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
    let mut nums: Vec<i64> = nums.iter().map(|i| *i as i64).collect();
    nums.sort_unstable();
    let m: i64 = 1000000007;
    let (mut res, mut x, mut y) = (0, nums[0], 2);
    for j in 1..nums.len() {
        res = (res + nums[j] * (y - 1) - x) % m;
        x = (x * 2 + nums[j]) % m;
        y = y * 2 % m;
    }
    ((res + m) % m) as i32
}

fn main() {
    {
        let nums = vec![2, 1, 3];
        let ans = 6;
        assert_eq!(sum_subseq_widths(nums), ans);
    }

    {
        let nums = vec![2];
        let ans = 0;
        assert_eq!(sum_subseq_widths(nums), ans);
    }

    {
        let nums = vec![
            5, 69, 89, 92, 31, 16, 25, 45, 63, 40, 16, 56, 24, 40, 75, 82, 40, 12, 50, 62, 92, 44,
            67, 38, 92, 22, 91, 24, 26, 21, 100, 42, 23, 56, 64, 43, 95, 76, 84, 79, 89, 4, 16, 94,
            16, 77, 92, 9, 30, 13,
        ];
        let ans = 857876214;
        assert_eq!(sum_subseq_widths(nums), ans);
    }
}
