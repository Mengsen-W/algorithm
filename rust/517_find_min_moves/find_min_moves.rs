/*
 * @Date: 2021-09-29 09:46:10
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-09-29 10:21:24
 */

struct Solution;

impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let tot: i32 = machines.iter().sum();
        let n = machines.len() as i32;
        if (tot % n) != 0 {
            return -1;
        }
        let avg = tot / n;
        let (mut ans, mut sum) = (0, 0);
        for mut num in machines {
            num -= avg;
            sum += num;
            ans = ans.max(sum.abs().max(num));
        }
        ans
    }
}

fn main() {
    {
        let machines = vec![1, 0, 5];
        assert_eq!(Solution::find_min_moves(machines), 3);
    }
    {
        let machines = vec![0, 3, 0];
        assert_eq!(Solution::find_min_moves(machines), 2);
    }
    {
        let machines = vec![0, 2, 0];
        assert_eq!(Solution::find_min_moves(machines), -1);
    }
}
