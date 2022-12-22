/*
 * @Date: 2022-12-22
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-22
 * @FilePath: /algorithm/1799_max_score/max_score.rs
 */

pub fn max_score(nums: Vec<i32>) -> i32 {
    fn gcd(a: i32, b: i32) -> i32 {
        if a % b == 0 {
            return b;
        }
        return gcd(b, a % b);
    }
    let m = nums.len();
    let mut dp: Vec<i32> = vec![0; 1 << m];
    let mut gcd_tmp = vec![vec![0; m]; m];
    for i in 0..m {
        for j in i + 1..m {
            gcd_tmp[i][j] = gcd(nums[i], nums[j]);
        }
    }
    let all = 1 << m;
    for s in 1..all {
        let t = (s as usize).count_ones() as i32;
        if t & 1 != 0 {
            continue;
        }

        for i in 0..m {
            if (s >> i) & 1 != 0 {
                for j in i + 1..m {
                    if (s >> j) & 1 != 0 {
                        dp[s] = std::cmp::max(
                            dp[s],
                            dp[s ^ (1 << i) ^ (1 << j)] + t / 2 * gcd_tmp[i][j],
                        );
                    }
                }
            }
        }
    }
    dp[all - 1]
}

fn main() {
    {
        let nums = vec![1, 2];
        let ans = 1;
        assert_eq!(max_score(nums), ans);
    }

    {
        let nums = vec![3, 4, 6, 8];
        let ans = 11;
        assert_eq!(max_score(nums), ans);
    }

    {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let ans = 14;
        assert_eq!(max_score(nums), ans);
    }
}
