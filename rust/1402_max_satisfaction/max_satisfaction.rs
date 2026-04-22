/*
 * @Date: 2023-10-22
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-10-22
 * @FilePath: /algorithm/rust/1402_max_satisfaction/max_satisfaction.rs
 */

struct Solution;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort_unstable_by(|a, b| b.cmp(a));
        let mut f = 0; // f(0) = 0
        let mut s = 0; // satisfaction 的前缀和
        for &x in &satisfaction {
            s += x;
            if s <= 0 {
                // 后面不可能找到更大的 f(k)
                break;
            }
            f += s; // f(k) = f(k-1) + s
        }
        f
    }
}

fn main() {
    let tests = vec![
        (vec![-1, -8, 0, 5, -9], 14),
        (vec![4, 3, 2], 20),
        (vec![-1, -4, -5], 0),
    ];

    for (satisfaction, ans) in tests {
        assert_eq!(Solution::max_satisfaction(satisfaction), ans);
    }
}
