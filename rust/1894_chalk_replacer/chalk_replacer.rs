/*
 * @Date: 2021-09-10 09:10:49
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-10 09:46:55
 */

struct Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let n = chalk.len();
        let mut sum: i32 = 0;
        let mut new_chalk: Vec<i32> = Vec::new();
        let mut m = 0;
        for i in 0..n {
            sum = sum + chalk[i];
            if sum > k {
                return i as i32;
            }
            if sum == k {
                m = (i + 1) % n;
                return m as i32;
            }
            new_chalk.push(sum);
        }
        let num = k % sum;
        for i in 0..n {
            if num < new_chalk[i] {
                return i as i32;
            }
            if num == new_chalk[i] {
                m = (i + 1) % n;
                return m as i32;
            }
        }
        return m as i32;
    }
}

fn main() {
    {
        let chalk = vec![5, 1, 5];
        let k = 22;
        let ans = 0;
        assert_eq!(Solution::chalk_replacer(chalk, k), ans);
    }
    {
        let chalk = vec![3, 4, 1, 2];
        let k = 22;
        let ans = 0;
        assert_eq!(Solution::chalk_replacer(chalk, k), ans);
    }
}
