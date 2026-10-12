struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut cnt = vec![0i32; 30];

        // 统计每个位上 1 的数量
        for &num in nums.iter() {
            for i in 0..30 {
                if num & (1 << i) != 0 {
                    cnt[i] += 1;
                }
            }
        }

        let mut res: i64 = 0;
        for _ in 0..k {
            let mut cur: i64 = 0;
            // 从高位到低位构造当前数字
            for j in (0..30).rev() {
                if cnt[j] > 0 {
                    cur |= 1i64 << j;
                    cnt[j] -= 1;
                }
            }
            res = (res + cur * cur) % MOD;
        }

        res as i32
    }
}

fn main() {
    let tests = vec![(vec![2, 6, 5, 8], 2, 261), (vec![4, 5, 4, 7], 3, 90)];

    for (nums, k, expected) in tests {
        assert_eq!(Solution::max_sum(nums, k), expected);
    }
}
