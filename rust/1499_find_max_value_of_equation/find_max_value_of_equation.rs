/*
 * @Date: 2023-07-21
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-07-21
 * @FilePath: /algorithm/rust/1499_find_max_value_of_equation/find_max_value_of_equation.rs
 */

struct Solution;
impl Solution {
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::VecDeque;
        let mut ans = i32::MIN;
        let mut dq: VecDeque<(i32, i32)> = Default::default();
        for point in points {
            let (px, py) = (point[0], point[1]);
            while dq.front().map_or(false, |x| x.0 < px - k) {
                dq.pop_front();
            }

            dq.front().map(|&(x, y)| ans = ans.max(px + py + y - x));

            while dq.back().map_or(false, |&(x, y)| py - px >= y - x) {
                dq.pop_back();
            }
            dq.push_back((px, py));
        }
        ans
    }
}

fn main() {
    let tests = vec![
        (
            vec![vec![1, 3], vec![2, 0], vec![5, 10], vec![6, -10]],
            1,
            4,
        ),
        (vec![vec![0, 0], vec![3, 0], vec![9, 2]], 3, 3),
    ];

    for (points, k, ans) in tests {
        assert_eq!(Solution::find_max_value_of_equation(points, k), ans)
    }
}
