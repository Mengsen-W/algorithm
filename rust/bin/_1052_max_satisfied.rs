/*
 * @Date: 2024-04-23
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2024-04-23
 * @FilePath: /algorithm/rust/1052_max_satisfied/max_satisfied.rs
 */

struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let mut ans = 0;
        let n = customers.len();
        let k = minutes as usize;
        let mut cnt = 0;
        for i in 0..n {
            if i < k || grumpy[i] == 0 {
                cnt += customers[i];
            }
        }
        for i in k..n {
            ans = ans.max(cnt);
            if grumpy[i] == 1 {
                cnt += customers[i];
            }
            if grumpy[i - k] == 1 {
                cnt -= customers[i - k];
            }
        }
        ans.max(cnt)
    }
}

fn main() {
    let tests = vec![
        (
            vec![1, 0, 1, 2, 1, 1, 7, 5],
            vec![0, 1, 0, 1, 0, 1, 0, 1],
            3,
            16,
        ),
        (vec![1], vec![0], 1, 1),
    ];

    for (customers, grumpy, minutes, ans) in tests {
        assert_eq!(Solution::max_satisfied(customers, grumpy, minutes), ans);
    }
}
