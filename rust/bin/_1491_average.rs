/*
 * @Author: Mengsen.Wang
 * @Date: 2021-02-17 23:02:22
 * @Last Modified by: Mengsen.Wang
 * @Last Modified time: 2021-02-17 23:33:34
 */

struct Solution;

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        (salary.iter().sum::<i32>() - salary.iter().min().unwrap() - salary.iter().max().unwrap())
            as f64
            / (salary.len() - 2) as f64
    }
}

fn main() {
    let tests = vec![
        (vec![4000, 3000, 1000, 2000], 2500.00000),
        (vec![1000, 2000, 3000], 2000.00000),
        (vec![6000, 5000, 4000, 3000, 2000, 1000], 3500.00000),
        (vec![8000, 9000, 2000, 3000, 6000, 1000], 4750.00000),
    ];

    for (salary, ans) in tests {
        assert_eq!(Solution::average(salary), ans);
    }
}
