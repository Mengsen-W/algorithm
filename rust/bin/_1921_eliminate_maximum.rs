/*
 * @Date: 2023-09-03
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-09-03
 * @FilePath: /algorithm/rust/1921_eliminate_maximum/eliminate_maximum.rs
 */

struct Solution;
impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut t: Vec<i32> = dist
            .into_iter()
            .zip(speed.into_iter())
            .map(|(d, s)| (d - 1) / s + 1)
            .collect();
        t.sort_unstable();
        let mut i = 0;
        while i < t.len() && t[i] as usize > i {
            i += 1;
        }
        i as i32
    }
}

fn main() {
    let tests = vec![
        (vec![1, 3, 4], vec![1, 1, 1], 3),
        (vec![1, 1, 2, 3], vec![1, 1, 1, 1], 1),
        (vec![3, 2, 4], vec![5, 3, 2], 1),
    ];

    for (dist, speed, ans) in tests {
        assert_eq!(Solution::eliminate_maximum(dist, speed), ans);
    }
}
